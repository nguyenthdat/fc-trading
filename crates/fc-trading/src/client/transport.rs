use reqwest::header::{CONTENT_TYPE, HeaderValue};
use secrecy::ExposeSecret as _;
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::TradingClient;
use super::endpoints::Endpoint;
use crate::{ApiResponse, Error, HttpError, ResponseDecodeError, Result};

impl TradingClient {
    pub(super) async fn authenticated_get<Q, T>(
        &self,
        endpoint: Endpoint,
        query: &Q,
    ) -> Result<ApiResponse<T>>
    where
        Q: Serialize + Sync + ?Sized,
        T: DeserializeOwned,
    {
        let token = self.read_bearer(false).await?;
        let url = self.endpoint_url(endpoint.path)?;
        let request = self
            .inner
            .http
            .get(url)
            .query(query)
            .bearer_auth(token.expose_secret());
        self.send(request).await
    }

    pub(super) async fn authenticated_get_without_query<T>(
        &self,
        endpoint: Endpoint,
    ) -> Result<ApiResponse<T>>
    where
        T: DeserializeOwned,
    {
        let token = self.read_bearer(false).await?;
        let url = self.endpoint_url(endpoint.path)?;
        let request = self.inner.http.get(url).bearer_auth(token.expose_secret());
        self.send(request).await
    }

    pub(super) async fn signed_post<P, T>(
        &self,
        endpoint: Endpoint,
        payload: &P,
    ) -> Result<ApiResponse<T>>
    where
        P: Serialize + Sync + ?Sized,
        T: DeserializeOwned,
    {
        let token = self.write_bearer().await?;
        let body = serde_json::to_vec(payload).map_err(Error::Serialization)?;
        let signature = self.inner.credentials.sign(&body)?;
        let url = self.endpoint_url(endpoint.path)?;
        let request = self
            .inner
            .http
            .post(url)
            .bearer_auth(token.expose_secret())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header("X-Signature", signature)
            .body(body);
        self.send(request).await
    }

    pub(super) async fn unauthenticated_post<P, T>(
        &self,
        endpoint: Endpoint,
        payload: &P,
    ) -> Result<ApiResponse<T>>
    where
        P: Serialize + Sync + ?Sized,
        T: DeserializeOwned,
    {
        self.unauthenticated_post_path(endpoint.path, payload).await
    }

    pub(super) async fn unauthenticated_post_path<P, T>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<ApiResponse<T>>
    where
        P: Serialize + Sync + ?Sized,
        T: DeserializeOwned,
    {
        let body = serde_json::to_vec(payload).map_err(Error::Serialization)?;
        let url = self.endpoint_url(path)?;
        let request = self
            .inner
            .http
            .post(url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body);
        self.send(request).await
    }

    pub(super) fn endpoint_url(&self, path: &str) -> Result<url::Url> {
        self.inner
            .config
            .api_base_url()
            .join(path.trim_start_matches('/'))
            .map_err(|error| Error::InvalidConfiguration(format!("invalid endpoint URL: {error}")))
    }

    async fn send<T>(&self, request: reqwest::RequestBuilder) -> Result<ApiResponse<T>>
    where
        T: DeserializeOwned,
    {
        let response = request.send().await.map_err(Error::Transport)?;
        let status = response.status();
        let (body, truncated) =
            crate::read_bounded_response(response, self.inner.config.max_response_bytes()).await?;
        if !status.is_success() {
            return Err(HttpError::new(status, body, truncated).into());
        }
        serde_json::from_slice(&body)
            .map_err(|source| ResponseDecodeError::new(source, body, truncated).into())
    }
}
