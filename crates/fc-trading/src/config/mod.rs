use std::net::IpAddr;
use std::time::Duration;

use url::Url;

use crate::{Error, Result};

const PRODUCTION_API_URL: &str = "https://fc-tradeapi.ssi.com.vn/";
const PRODUCTION_STREAM_URL: &str = "https://fc-tradehub.ssi.com.vn/";

#[derive(Clone, Debug)]
pub struct ClientConfig {
    api_base_url: Url,
    stream_base_url: Url,
    request_timeout: Duration,
    connect_timeout: Duration,
    token_refresh_skew: Duration,
    max_response_bytes: usize,
    max_websocket_message_bytes: usize,
}

impl ClientConfig {
    pub fn production() -> Result<Self> {
        Self::new(PRODUCTION_API_URL, PRODUCTION_STREAM_URL)
    }

    pub fn new(api_base_url: &str, stream_base_url: &str) -> Result<Self> {
        let api_base_url = parse_base_url(api_base_url, "API")?;
        let stream_base_url = parse_base_url(stream_base_url, "stream")?;
        Ok(Self {
            api_base_url,
            stream_base_url,
            request_timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            token_refresh_skew: Duration::from_secs(600),
            max_response_bytes: 2 * 1024 * 1024,
            max_websocket_message_bytes: 2 * 1024 * 1024,
        })
    }

    #[must_use]
    pub const fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    #[must_use]
    pub const fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    #[must_use]
    pub const fn with_token_refresh_skew(mut self, skew: Duration) -> Self {
        self.token_refresh_skew = skew;
        self
    }

    #[must_use]
    pub const fn with_max_response_bytes(mut self, bytes: usize) -> Self {
        self.max_response_bytes = bytes;
        self
    }

    #[must_use]
    pub const fn with_max_websocket_message_bytes(mut self, bytes: usize) -> Self {
        self.max_websocket_message_bytes = bytes;
        self
    }

    #[must_use]
    pub const fn api_base_url(&self) -> &Url {
        &self.api_base_url
    }

    #[must_use]
    pub const fn stream_base_url(&self) -> &Url {
        &self.stream_base_url
    }

    pub(crate) const fn request_timeout(&self) -> Duration {
        self.request_timeout
    }

    pub(crate) const fn connect_timeout(&self) -> Duration {
        self.connect_timeout
    }

    pub(crate) const fn token_refresh_skew(&self) -> Duration {
        self.token_refresh_skew
    }

    pub(crate) const fn max_response_bytes(&self) -> usize {
        self.max_response_bytes
    }

    pub(crate) const fn max_websocket_message_bytes(&self) -> usize {
        self.max_websocket_message_bytes
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ReconnectPolicy {
    initial_delay: Duration,
    maximum_delay: Duration,
}

impl ReconnectPolicy {
    pub fn new(initial_delay: Duration, maximum_delay: Duration) -> Result<Self> {
        if initial_delay.is_zero() || maximum_delay < initial_delay {
            return Err(Error::InvalidConfiguration(
                "reconnect delays must be non-zero and ordered".to_owned(),
            ));
        }
        Ok(Self {
            initial_delay,
            maximum_delay,
        })
    }

    pub(crate) const fn initial_delay(self) -> Duration {
        self.initial_delay
    }

    pub(crate) const fn maximum_delay(self) -> Duration {
        self.maximum_delay
    }
}

impl Default for ReconnectPolicy {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_secs(1),
            maximum_delay: Duration::from_secs(30),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StreamOptions {
    notify_id: String,
    channel_capacity: usize,
    reconnect: ReconnectPolicy,
}

impl StreamOptions {
    #[must_use]
    pub fn new(notify_id: impl Into<String>) -> Self {
        Self {
            notify_id: notify_id.into(),
            channel_capacity: 256,
            reconnect: ReconnectPolicy::default(),
        }
    }

    pub fn with_channel_capacity(mut self, capacity: usize) -> Result<Self> {
        if capacity == 0 {
            return Err(Error::InvalidStreamChannelCapacity { capacity });
        }
        self.channel_capacity = capacity;
        Ok(self)
    }

    #[must_use]
    pub const fn with_reconnect_policy(mut self, reconnect: ReconnectPolicy) -> Self {
        self.reconnect = reconnect;
        self
    }

    pub(crate) fn notify_id(&self) -> &str {
        &self.notify_id
    }

    pub(crate) const fn channel_capacity(&self) -> usize {
        self.channel_capacity
    }

    pub(crate) const fn reconnect(&self) -> ReconnectPolicy {
        self.reconnect
    }
}

impl Default for StreamOptions {
    fn default() -> Self {
        Self::new("-1")
    }
}

fn parse_base_url(value: &str, kind: &str) -> Result<Url> {
    let mut url = Url::parse(value).map_err(|error| {
        Error::InvalidConfiguration(format!("invalid {kind} base URL: {error}"))
    })?;
    let secure = url.scheme() == "https";
    let loopback = url.host_str().is_some_and(|host| {
        host.eq_ignore_ascii_case("localhost")
            || host
                .parse::<IpAddr>()
                .is_ok_and(|address| address.is_loopback())
    });
    if !(secure || url.scheme() == "http" && loopback) {
        return Err(Error::InvalidConfiguration(format!(
            "{kind} base URL must use HTTPS outside loopback"
        )));
    }
    if !url.path().ends_with('/') {
        let normalized = format!("{}/", url.path());
        url.set_path(&normalized);
    }
    Ok(url)
}

#[cfg(test)]
mod tests;
