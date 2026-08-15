//! Pure Rust client for SSI `FastConnect` Trading.

mod error;

pub mod auth;
pub(crate) mod client;
pub mod config;
pub mod models;
pub mod stream;

pub use auth::{Credentials, TwoFactorType};
pub use client::TradingClient;
pub use config::{ClientConfig, ReconnectPolicy, StreamOptions};
pub use error::{Error, HttpError, ResponseDecodeError, Result};
pub use models::{ApiResponse, DecimalNumber, RawData};
pub use models::{requests::*, responses::*};
pub use stream::{StreamEvent, TradingStream};

pub(crate) async fn read_bounded_response(
    response: reqwest::Response,
    limit: usize,
) -> Result<(Vec<u8>, bool)> {
    use futures_util::StreamExt as _;

    let mut stream = response.bytes_stream();
    let mut body = Vec::with_capacity(limit.min(16 * 1024));
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(Error::Transport)?;
        let remaining = limit.saturating_sub(body.len());
        if chunk.len() > remaining {
            body.extend_from_slice(&chunk[..remaining]);
            return Ok((body, true));
        }
        body.extend_from_slice(&chunk);
    }
    Ok((body, false))
}
