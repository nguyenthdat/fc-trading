use std::fmt;

use reqwest::StatusCode;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("invalid client configuration: {0}")]
    InvalidConfiguration(String),
    #[error("stream channel capacity must be greater than zero, got {capacity}")]
    InvalidStreamChannelCapacity { capacity: usize },
    #[error("invalid FastConnect private key: {0}")]
    InvalidPrivateKey(String),
    #[error("request serialization failed")]
    Serialization(#[source] serde_json::Error),
    #[error("HTTP transport failed")]
    Transport(#[source] reqwest::Error),
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    ResponseDecode(#[from] ResponseDecodeError),
    #[error("authentication failed: {0}")]
    Authentication(String),
    #[error("write authentication is required; call verify_code first")]
    WriteAuthenticationRequired,
    #[error("classic SignalR protocol error: {0}")]
    SignalRProtocol(String),
    #[error("WebSocket transport failed")]
    WebSocket(#[source] tokio_tungstenite::tungstenite::Error),
    #[error("stream supervisor failed")]
    StreamSupervisor(#[source] tokio::task::JoinError),
}

pub struct HttpError {
    status: StatusCode,
    body: Vec<u8>,
    truncated: bool,
}

impl HttpError {
    pub(crate) const fn new(status: StatusCode, body: Vec<u8>, truncated: bool) -> Self {
        Self {
            status,
            body,
            truncated,
        }
    }

    #[must_use]
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    #[must_use]
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.truncated
    }
}

impl fmt::Debug for HttpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpError")
            .field("status", &self.status)
            .field("body", &"[REDACTED]")
            .field("truncated", &self.truncated)
            .finish()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "HTTP request failed with status {}", self.status)
    }
}

impl std::error::Error for HttpError {}

pub struct ResponseDecodeError {
    source: serde_json::Error,
    body: Vec<u8>,
    truncated: bool,
}

impl ResponseDecodeError {
    pub(crate) const fn new(source: serde_json::Error, body: Vec<u8>, truncated: bool) -> Self {
        Self {
            source,
            body,
            truncated,
        }
    }

    #[must_use]
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.truncated
    }
}

impl fmt::Debug for ResponseDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResponseDecodeError")
            .field("source", &self.source)
            .field("body", &"[REDACTED]")
            .field("truncated", &self.truncated)
            .finish()
    }
}

impl fmt::Display for ResponseDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("HTTP response was not valid FastConnect JSON")
    }
}

impl std::error::Error for ResponseDecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
