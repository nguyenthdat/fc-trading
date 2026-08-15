use reqwest::StatusCode;
use secrecy::ExposeSecret as _;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest as _;
use tokio_tungstenite::tungstenite::http::{HeaderValue, header::AUTHORIZATION};
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async_with_config};

use super::protocol::{CLIENT_PROTOCOL, CONNECTION_DATA, Negotiation};
use crate::{Error, HttpError, ResponseDecodeError, Result, StreamOptions, TradingClient};

pub(super) type Session = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub(super) async fn connect(client: &TradingClient, options: &StreamOptions) -> Result<Session> {
    match negotiate(client, options, false).await {
        Err(Error::Http(error)) if error.status() == StatusCode::UNAUTHORIZED => {
            client.invalidate_read_token().await;
            let negotiation = negotiate(client, options, true).await?;
            open_websocket(client, options, &negotiation).await
        }
        Err(error) => Err(error),
        Ok(negotiation) => open_websocket(client, options, &negotiation).await,
    }
}

async fn negotiate(
    client: &TradingClient,
    options: &StreamOptions,
    force_refresh: bool,
) -> Result<Negotiation> {
    let token = client.read_bearer(force_refresh).await?;
    let mut url = signalr_url(client, "negotiate")?;
    url.query_pairs_mut()
        .append_pair("connectionData", CONNECTION_DATA)
        .append_pair("clientProtocol", CLIENT_PROTOCOL);
    let response = client
        .inner
        .http
        .post(url)
        .bearer_auth(token.expose_secret())
        .header("NotifyID", options.notify_id())
        .send()
        .await
        .map_err(Error::Transport)?;
    let status = response.status();
    let (body, truncated) =
        crate::read_bounded_response(response, client.inner.config.max_response_bytes()).await?;
    if !status.is_success() {
        return Err(HttpError::new(status, body, truncated).into());
    }
    let negotiation: Negotiation = serde_json::from_slice(&body)
        .map_err(|source| ResponseDecodeError::new(source, body, truncated))?;
    if negotiation.protocol_version != CLIENT_PROTOCOL {
        return Err(Error::SignalRProtocol(format!(
            "server selected unsupported protocol {}",
            negotiation.protocol_version
        )));
    }
    Ok(negotiation)
}

async fn open_websocket(
    client: &TradingClient,
    options: &StreamOptions,
    negotiation: &Negotiation,
) -> Result<Session> {
    let token = client.read_bearer(false).await?;
    let mut url = signalr_url(client, "connect")?;
    let websocket_scheme = match url.scheme() {
        "https" => "wss",
        "http" => "ws",
        scheme => {
            return Err(Error::InvalidConfiguration(format!(
                "unsupported stream URL scheme {scheme}"
            )));
        }
    };
    url.set_scheme(websocket_scheme).map_err(|()| {
        Error::InvalidConfiguration("could not convert stream URL to WebSocket".to_owned())
    })?;
    url.query_pairs_mut()
        .append_pair("transport", "webSockets")
        .append_pair("connectionToken", &negotiation.connection_token)
        .append_pair("connectionData", CONNECTION_DATA)
        .append_pair("clientProtocol", CLIENT_PROTOCOL);
    let mut request = url
        .as_str()
        .into_client_request()
        .map_err(Error::WebSocket)?;
    request.headers_mut().insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token.expose_secret()))
            .map_err(|_| Error::Authentication("access token is not a valid header".to_owned()))?,
    );
    request.headers_mut().insert(
        "NotifyID",
        HeaderValue::from_str(options.notify_id()).map_err(|_| {
            Error::InvalidConfiguration("NotifyID is not a valid header".to_owned())
        })?,
    );
    let maximum = client.inner.config.max_websocket_message_bytes();
    let mut websocket_config = WebSocketConfig::default();
    websocket_config.max_message_size = Some(maximum);
    websocket_config.max_frame_size = Some(maximum);
    let (session, _) = connect_async_with_config(request, Some(websocket_config), false)
        .await
        .map_err(Error::WebSocket)?;
    Ok(session)
}

fn signalr_url(client: &TradingClient, operation: &str) -> Result<url::Url> {
    client
        .inner
        .config
        .stream_base_url()
        .join(&format!("v2.0/signalr/{operation}"))
        .map_err(|error| Error::InvalidConfiguration(format!("invalid SignalR URL: {error}")))
}
