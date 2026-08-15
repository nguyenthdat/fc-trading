#![cfg(test)]

mod fixtures;
mod http_support;

use axum::http::StatusCode;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::signature::Verifier as _;
use sha2::Sha256;
use ssi_fc_trading::{Error, TradingClient, TwoFactorType};

use fixtures::{fixture_credentials, fixture_token};
use http_support::{TestServer, sample_new_order};

#[tokio::test]
async fn otp_endpoint_sends_unauthenticated_contract_body_when_called() {
    // Given
    let server = TestServer::start().await;
    let (credentials, _) = fixture_credentials();
    let client = TradingClient::new(server.config(), credentials, TwoFactorType::Pin)
        .expect("client builds");

    // When
    let response = client.request_otp().await.expect("OTP request succeeds");

    // Then
    assert_eq!(response.status, 200);
    let request = server.next_request().await;
    assert_eq!(request.method, "POST");
    assert_eq!(request.path, "/api/v2/Trading/GetOTP");
    assert_eq!(
        request.body,
        br#"{"consumerID":"consumer-id","consumerSecret":"consumer-secret"}"#
    );
    assert!(!request.headers.contains_key("authorization"));
    assert!(!request.headers.contains_key("x-signature"));
    drop(client);
    server.close().await;
}

#[tokio::test]
async fn read_endpoint_acquires_read_token_and_sends_bearer_when_called() {
    // Given
    let server = TestServer::start().await;
    let (credentials, _) = fixture_credentials();
    let client = TradingClient::new(server.config(), credentials, TwoFactorType::Pin)
        .expect("client builds");

    // When
    let response = client.rate_limit().await.expect("rate-limit call succeeds");

    // Then
    assert_eq!(response.status, 200);
    let token_request = server.next_request().await;
    assert_eq!(token_request.method, "POST");
    assert_eq!(token_request.path, "/api/v2/Trading/AccessToken");
    assert_eq!(
        token_request.body,
        br#"{"consumerID":"consumer-id","consumerSecret":"consumer-secret","twoFactorType":0,"code":"1","isSave":false}"#
    );
    assert!(!token_request.headers.contains_key("authorization"));
    assert!(!token_request.headers.contains_key("x-signature"));

    let read_request = server.next_request().await;
    assert_eq!(read_request.method, "GET");
    assert_eq!(read_request.path, "/api/v2/Trading/rateLimit");
    assert_eq!(read_request.query, None);
    assert_eq!(
        read_request.headers["authorization"],
        format!("Bearer {}", fixture_token("read"))
    );
    assert!(!read_request.headers.contains_key("x-signature"));
    drop(client);
    server.close().await;
}

#[tokio::test]
async fn signed_order_uses_verified_write_token_and_signs_transmitted_body() {
    // Given
    let server = TestServer::start().await;
    let (credentials, public_key) = fixture_credentials();
    let client = TradingClient::new(server.config(), credentials, TwoFactorType::Pin)
        .expect("client builds");
    client
        .verify_code("654321")
        .await
        .expect("write token acquired");
    let token_request = server.next_request().await;
    assert_eq!(token_request.path, "/api/v2/Trading/AccessToken");

    // When
    client
        .new_order(&sample_new_order())
        .await
        .expect("new order response decodes");

    // Then
    let request = server.next_request().await;
    assert_eq!(request.method, "POST");
    assert_eq!(request.path, "/api/v2/Trading/NewOrder");
    assert_eq!(
        request.headers["authorization"],
        format!("Bearer {}", fixture_token("write"))
    );
    let signature = request.headers["x-signature"]
        .to_str()
        .expect("signature header is text");
    assert_eq!(signature, signature.to_lowercase());
    let signature = hex::decode(signature).expect("signature is hexadecimal");
    let signature = Signature::try_from(signature.as_slice()).expect("signature length");
    VerifyingKey::<Sha256>::new(public_key)
        .verify(&request.body, &signature)
        .expect("signature verifies over transmitted bytes");
    drop(client);
    server.close().await;
}

#[tokio::test]
async fn signed_order_fails_locally_when_write_token_is_missing() {
    // Given
    let server = TestServer::start().await;
    let (credentials, _) = fixture_credentials();
    let client = TradingClient::new(server.config(), credentials, TwoFactorType::Pin)
        .expect("client builds");

    // When
    let error = client
        .new_order(&sample_new_order())
        .await
        .expect_err("write token is required");

    // Then
    assert!(matches!(error, Error::WriteAuthenticationRequired));
    assert!(!server.has_pending_request().await);
    drop(client);
    server.close().await;
}

#[tokio::test]
async fn http_error_preserves_status_and_body_without_retry_when_received() {
    // Given
    let server = TestServer::failing(
        "/api/v2/Trading/rateLimit",
        StatusCode::TOO_MANY_REQUESTS,
        r#"{"status":429,"message":"quota"}"#,
    )
    .await;
    let (credentials, _) = fixture_credentials();
    let client = TradingClient::new(server.config(), credentials, TwoFactorType::Pin)
        .expect("client builds");

    // When
    let error = client
        .rate_limit()
        .await
        .expect_err("429 is transport error");

    // Then
    let Error::Http(error) = error else {
        panic!("expected HTTP error");
    };
    assert_eq!(error.status(), StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(error.body(), br#"{"status":429,"message":"quota"}"#);
    let _token_request = server.next_request().await;
    let _rate_request = server.next_request().await;
    assert!(!server.has_pending_request().await);
    drop(client);
    server.close().await;
}
