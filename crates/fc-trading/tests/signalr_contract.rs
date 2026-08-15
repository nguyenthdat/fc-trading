#![cfg(test)]

mod fixtures;
mod signalr_support;

use std::time::Duration;

use ssi_fc_trading::{ReconnectPolicy, StreamEvent, StreamOptions, TradingClient, TwoFactorType};

use fixtures::fixture_credentials;
use signalr_support::{SignalServer, assert_signal_requests};

#[tokio::test]
async fn signalr_stream_negotiates_dispatches_and_closes_when_connected() {
    // Given
    let server = SignalServer::start(false).await;
    let (credentials, _) = fixture_credentials();
    let client = TradingClient::new(server.config(), credentials, TwoFactorType::Pin)
        .expect("client builds");

    // When
    let mut stream = client
        .stream(StreamOptions::new("77"))
        .await
        .expect("stream connects");

    // Then
    assert!(matches!(
        stream.next().await,
        Some(Ok(StreamEvent::Broadcast(payload))) if payload == "payload"
    ));
    assert!(matches!(
        stream.next().await,
        Some(Ok(StreamEvent::ServerError(message))) if message == "problem"
    ));
    assert_signal_requests(&server, "77").await;
    stream.close().await.expect("stream closes cleanly");
    drop(client);
    server.close().await;
}

#[tokio::test]
async fn signalr_stream_renegotiates_and_preserves_notify_id_after_remote_close() {
    // Given
    let server = SignalServer::start(true).await;
    let (credentials, _) = fixture_credentials();
    let client = TradingClient::new(server.config(), credentials, TwoFactorType::Pin)
        .expect("client builds");
    let reconnect = ReconnectPolicy::new(Duration::from_millis(10), Duration::from_millis(20))
        .expect("test reconnect policy");

    // When
    let mut stream = client
        .stream(StreamOptions::new("99").with_reconnect_policy(reconnect))
        .await
        .expect("stream connects");

    // Then
    assert!(matches!(
        stream.next().await,
        Some(Ok(StreamEvent::Broadcast(payload))) if payload == "first"
    ));
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(3), stream.next()).await,
        Ok(Some(Ok(StreamEvent::Broadcast(payload)))) if payload == "second"
    ));
    assert_signal_requests(&server, "99").await;
    assert_signal_requests(&server, "99").await;
    stream.close().await.expect("reconnected stream closes");
    drop(client);
    server.close().await;
}
