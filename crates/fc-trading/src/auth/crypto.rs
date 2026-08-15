use base64::{Engine as _, engine::general_purpose::STANDARD};
use quick_xml::de::from_str;
use rand::rngs::OsRng;
use rsa::BigUint;
use rsa::RsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::signature::{RandomizedSigner as _, SignatureEncoding as _};
use rsa::traits::PublicKeyParts as _;
use serde::Deserialize;
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::{Error, Result};

const MAX_ENCODED_KEY_BYTES: usize = 128 * 1024;
const MINIMUM_RSA_BITS: usize = 1_024;

pub(super) struct RequestSigner {
    key: SigningKey<Sha256>,
}

impl RequestSigner {
    pub(super) fn from_base64_xml(encoded: &str) -> Result<Self> {
        if encoded.len() > MAX_ENCODED_KEY_BYTES {
            return Err(Error::InvalidPrivateKey(
                "encoded key is too large".to_owned(),
            ));
        }
        let decoded = STANDARD
            .decode(encoded.trim())
            .map_err(|_| Error::InvalidPrivateKey("key is not valid base64".to_owned()))?;
        let xml = Zeroizing::new(String::from_utf8(decoded).map_err(|_| {
            Error::InvalidPrivateKey("decoded key XML is not valid UTF-8".to_owned())
        })?);
        let components: RsaKeyValue = from_str(&xml)
            .map_err(|_| Error::InvalidPrivateKey("key XML is malformed".to_owned()))?;
        let mut private_key = RsaPrivateKey::from_components(
            decode_component(&components.modulus, "Modulus")?,
            decode_component(&components.exponent, "Exponent")?,
            decode_component(&components.d, "D")?,
            vec![
                decode_component(&components.p, "P")?,
                decode_component(&components.q, "Q")?,
            ],
        )
        .map_err(|_| Error::InvalidPrivateKey("RSA components are inconsistent".to_owned()))?;
        private_key
            .validate()
            .map_err(|_| Error::InvalidPrivateKey("RSA key validation failed".to_owned()))?;
        if private_key.n().bits() < MINIMUM_RSA_BITS {
            return Err(Error::InvalidPrivateKey(
                "RSA key must be at least 1024 bits".to_owned(),
            ));
        }
        private_key
            .precompute()
            .map_err(|_| Error::InvalidPrivateKey("RSA precomputation failed".to_owned()))?;
        Ok(Self {
            key: SigningKey::new(private_key),
        })
    }

    pub(super) fn sign_hex(&self, body: &[u8]) -> Result<String> {
        let signature = self
            .key
            .try_sign_with_rng(&mut OsRng, body)
            .map_err(|_| Error::Authentication("request signing failed".to_owned()))?;
        Ok(hex::encode(signature.to_bytes()))
    }
}

#[derive(Deserialize)]
#[serde(rename = "RSAKeyValue")]
struct RsaKeyValue {
    #[serde(rename = "Modulus")]
    modulus: String,
    #[serde(rename = "Exponent")]
    exponent: String,
    #[serde(rename = "P")]
    p: String,
    #[serde(rename = "Q")]
    q: String,
    #[serde(rename = "D")]
    d: String,
}

fn decode_component(value: &str, name: &str) -> Result<BigUint> {
    let bytes = STANDARD.decode(value.trim()).map_err(|_| {
        Error::InvalidPrivateKey(format!("RSA component {name} is not valid base64"))
    })?;
    if bytes.is_empty() {
        return Err(Error::InvalidPrivateKey(format!(
            "RSA component {name} is empty"
        )));
    }
    Ok(BigUint::from_bytes_be(&bytes))
}
