mod crypto;

use std::fmt;

use base64::{
    Engine as _, engine::general_purpose::URL_SAFE, engine::general_purpose::URL_SAFE_NO_PAD,
};
use secrecy::{ExposeSecret as _, SecretString};
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{Error, Result};

use crypto::RequestSigner;

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum TwoFactorType {
    Pin = 0,
    Otp = 1,
    Ca = 2,
}

pub struct Credentials {
    consumer_id: String,
    consumer_secret: SecretString,
    signer: RequestSigner,
}

impl Credentials {
    pub fn from_base64_xml(
        consumer_id: impl Into<String>,
        consumer_secret: SecretString,
        private_key: SecretString,
    ) -> Result<Self> {
        let signer = RequestSigner::from_base64_xml(private_key.expose_secret())?;
        drop(private_key);
        Ok(Self {
            consumer_id: consumer_id.into(),
            consumer_secret,
            signer,
        })
    }

    pub(crate) fn consumer_id(&self) -> &str {
        &self.consumer_id
    }

    pub(crate) fn consumer_secret(&self) -> &str {
        self.consumer_secret.expose_secret()
    }

    pub(crate) fn sign(&self, body: &[u8]) -> Result<String> {
        self.signer.sign_hex(body)
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Credentials")
            .field("consumer_id", &"[REDACTED]")
            .field("consumer_secret", &"[REDACTED]")
            .field("private_key", &"[REDACTED]")
            .finish()
    }
}

pub(crate) struct CachedToken {
    value: SecretString,
    expires_at: u64,
}

impl CachedToken {
    pub(crate) fn parse(value: String) -> Result<Self> {
        let payload = value
            .split('.')
            .nth(1)
            .ok_or_else(|| Error::Authentication("access token is not a JWT".to_owned()))?;
        let decoded = URL_SAFE_NO_PAD
            .decode(payload)
            .or_else(|_| URL_SAFE.decode(payload))
            .map_err(|_| Error::Authentication("JWT payload is not valid base64url".to_owned()))?;
        let claims: Claims = serde_json::from_slice(&decoded)
            .map_err(|_| Error::Authentication("JWT payload is not valid JSON".to_owned()))?;
        Ok(Self {
            value: SecretString::from(value),
            expires_at: claims.exp,
        })
    }

    pub(crate) fn is_valid_at(&self, now: u64, refresh_skew: u64) -> bool {
        now.checked_add(refresh_skew)
            .is_some_and(|refresh_at| self.expires_at > refresh_at)
    }

    pub(crate) fn clone_secret(&self) -> SecretString {
        self.value.clone()
    }
}

#[derive(Deserialize)]
struct Claims {
    exp: u64,
}

#[cfg(test)]
mod tests;
