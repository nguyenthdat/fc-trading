#![cfg(test)]

use std::env;
use std::time::Duration;

use secrecy::SecretString;
use ssi_fc_trading::{ClientConfig, Credentials, StreamOptions, TradingClient, TwoFactorType};
use zeroize::Zeroizing;

const OPT_IN: &str = "I_UNDERSTAND_SANDBOX_NETWORK_CALLS";
const PRODUCTION_API_HOST: &str = "fc-tradeapi.ssi.com.vn";
const PRODUCTION_STREAM_HOST: &str = "fc-tradehub.ssi.com.vn";

#[tokio::test]
#[ignore = "requires explicit SSI sandbox credentials and network access"]
async fn live_auth_read_and_stream_smoke() {
    if env::var("SSI_FCTRADING_SANDBOX_SMOKE").as_deref() != Ok(OPT_IN) {
        return;
    }

    // Given
    let api_url = required_env("SSI_FCTRADING_SANDBOX_API_URL");
    let stream_url = required_env("SSI_FCTRADING_SANDBOX_STREAM_URL");
    assert!(!api_url.contains(PRODUCTION_API_HOST));
    assert!(!stream_url.contains(PRODUCTION_STREAM_HOST));
    let config = ClientConfig::new(&api_url, &stream_url)
        .expect("sandbox URLs are valid")
        .with_request_timeout(Duration::from_secs(15))
        .with_connect_timeout(Duration::from_secs(5));
    let credentials = Credentials::from_base64_xml(
        required_env("SSI_FCTRADING_CONSUMER_ID"),
        SecretString::from(required_env("SSI_FCTRADING_CONSUMER_SECRET")),
        SecretString::from(required_env("SSI_FCTRADING_PRIVATE_KEY")),
    )
    .expect("sandbox credentials are valid");
    let client =
        TradingClient::new(config, credentials, two_factor()).expect("sandbox client is valid");
    let verification_code = Zeroizing::new(required_env("SSI_FCTRADING_VERIFICATION_CODE"));

    // When / Then: write authentication completes without a business mutation.
    tokio::time::timeout(
        Duration::from_secs(20),
        client.verify_code(&verification_code),
    )
    .await
    .expect("sandbox write authentication completes before timeout")
    .expect("sandbox write authentication succeeds");

    // When / Then: the safe read endpoint returns a successful envelope.
    let response = tokio::time::timeout(Duration::from_secs(20), client.rate_limit())
        .await
        .expect("sandbox rate-limit call completes before timeout")
        .expect("sandbox rate-limit call succeeds");
    assert_eq!(response.status, 200);

    // When / Then: SignalR negotiates, connects, and closes without waiting for an event.
    let notify_id = env::var("SSI_FCTRADING_NOTIFY_ID").unwrap_or_else(|_| "-1".to_owned());
    let stream = tokio::time::timeout(
        Duration::from_secs(20),
        client.stream(StreamOptions::new(notify_id)),
    )
    .await
    .expect("sandbox stream connects before timeout")
    .expect("sandbox stream negotiates and connects");
    tokio::time::timeout(Duration::from_secs(10), stream.close())
        .await
        .expect("sandbox stream closes before timeout")
        .expect("sandbox stream closes cleanly");
}

fn required_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("missing required environment variable {name}"))
}

fn two_factor() -> TwoFactorType {
    match env::var("SSI_FCTRADING_TWO_FACTOR_TYPE").as_deref() {
        Ok("1") => TwoFactorType::Otp,
        Ok("2") => TwoFactorType::Ca,
        Ok("0") | Err(_) => TwoFactorType::Pin,
        Ok(other) => panic!("invalid SSI_FCTRADING_TWO_FACTOR_TYPE {other}"),
    }
}
