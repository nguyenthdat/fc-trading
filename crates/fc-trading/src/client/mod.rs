mod authentication;
mod cash;
mod endpoints;
mod ors;
mod stock;
mod trading;
mod transport;

use std::sync::Arc;

use reqwest::redirect::Policy;
use tokio::sync::Mutex;

use crate::auth::CachedToken;
use crate::{ClientConfig, Credentials, Error, Result, TwoFactorType};

pub struct TradingClient {
    pub(super) inner: Arc<Inner>,
}

pub struct Inner {
    pub config: ClientConfig,
    pub credentials: Credentials,
    pub two_factor: TwoFactorType,
    pub http: reqwest::Client,
    pub read_token: Mutex<Option<CachedToken>>,
    pub write_token: Mutex<Option<CachedToken>>,
}

impl TradingClient {
    pub fn new(
        config: ClientConfig,
        credentials: Credentials,
        two_factor: TwoFactorType,
    ) -> Result<Self> {
        let http = reqwest::Client::builder()
            .connect_timeout(config.connect_timeout())
            .timeout(config.request_timeout())
            .redirect(Policy::none())
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .map_err(Error::Transport)?;
        Ok(Self {
            inner: Arc::new(Inner {
                config,
                credentials,
                two_factor,
                http,
                read_token: Mutex::new(None),
                write_token: Mutex::new(None),
            }),
        })
    }
}

impl Clone for TradingClient {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[cfg(test)]
mod tests;
