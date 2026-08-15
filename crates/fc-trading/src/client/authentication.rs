use std::time::{SystemTime, UNIX_EPOCH};

use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use super::TradingClient;
use super::endpoints::REQUEST_OTP;
use crate::auth::CachedToken;
use crate::{ApiResponse, Error, RawData, Result};

const ACCESS_TOKEN_PATH: &str = "/api/v2/Trading/AccessToken";

impl TradingClient {
    pub async fn request_otp(&self) -> Result<ApiResponse<RawData>> {
        let request = OtpRequest {
            consumer_id: self.inner.credentials.consumer_id(),
            consumer_secret: self.inner.credentials.consumer_secret(),
        };
        self.unauthenticated_post(REQUEST_OTP, &request).await
    }

    pub async fn verify_code(&self, code: &str) -> Result<()> {
        let token = self.obtain_token(code, true).await?;
        *self.inner.write_token.lock().await = Some(token);
        Ok(())
    }

    pub(crate) async fn read_bearer(&self, force_refresh: bool) -> Result<SecretString> {
        let mut cached = self.inner.read_token.lock().await;
        let now = unix_timestamp()?;
        let skew = self.inner.config.token_refresh_skew().as_secs();
        if !force_refresh
            && let Some(token) = cached.as_ref()
            && token.is_valid_at(now, skew)
        {
            return Ok(token.clone_secret());
        }
        let token = self.obtain_token("1", false).await?;
        let secret = token.clone_secret();
        *cached = Some(token);
        drop(cached);
        Ok(secret)
    }

    pub(crate) async fn write_bearer(&self) -> Result<SecretString> {
        let mut cached = self.inner.write_token.lock().await;
        let now = unix_timestamp()?;
        let result = match cached.as_ref() {
            Some(token) if token.is_valid_at(now, 0) => Ok(token.clone_secret()),
            Some(_) => {
                *cached = None;
                Err(Error::WriteAuthenticationRequired)
            }
            None => Err(Error::WriteAuthenticationRequired),
        };
        drop(cached);
        result
    }

    pub(crate) async fn invalidate_read_token(&self) {
        *self.inner.read_token.lock().await = None;
    }

    async fn obtain_token(&self, code: &str, save: bool) -> Result<CachedToken> {
        let request = AccessTokenRequest {
            consumer_id: self.inner.credentials.consumer_id(),
            consumer_secret: self.inner.credentials.consumer_secret(),
            two_factor_type: self.inner.two_factor,
            code,
            is_save: save,
        };
        let response: ApiResponse<AccessTokenData> = self
            .unauthenticated_post_path(ACCESS_TOKEN_PATH, &request)
            .await?;
        if response.status != 200 {
            return Err(Error::Authentication(response.message));
        }
        let token = response
            .data
            .ok_or_else(|| Error::Authentication("token response did not contain data".to_owned()))?
            .access_token;
        CachedToken::parse(token)
    }
}

#[derive(Serialize)]
struct OtpRequest<'a> {
    #[serde(rename = "consumerID")]
    consumer_id: &'a str,
    #[serde(rename = "consumerSecret")]
    consumer_secret: &'a str,
}

#[derive(Serialize)]
struct AccessTokenRequest<'a> {
    #[serde(rename = "consumerID")]
    consumer_id: &'a str,
    #[serde(rename = "consumerSecret")]
    consumer_secret: &'a str,
    #[serde(rename = "twoFactorType")]
    two_factor_type: crate::TwoFactorType,
    code: &'a str,
    #[serde(rename = "isSave")]
    is_save: bool,
}

#[derive(Deserialize)]
struct AccessTokenData {
    #[serde(rename = "accessToken")]
    access_token: String,
}

fn unix_timestamp() -> Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|_| Error::Authentication("system clock precedes Unix epoch".to_owned()))
}
