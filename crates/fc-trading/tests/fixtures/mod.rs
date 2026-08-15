use std::time::{SystemTime, UNIX_EPOCH};

use base64::{
    Engine as _, engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD,
};
use rand::SeedableRng as _;
use rand_chacha::ChaCha20Rng;
use rsa::{RsaPrivateKey, RsaPublicKey, traits::PrivateKeyParts as _, traits::PublicKeyParts as _};
use secrecy::SecretString;
use ssi_fc_trading::Credentials;

pub fn fixture_credentials() -> (Credentials, RsaPublicKey) {
    let mut rng = ChaCha20Rng::seed_from_u64(23);
    let key = RsaPrivateKey::new(&mut rng, 1_024).expect("fixture key generation");
    let public_key = RsaPublicKey::from(&key);
    let xml = format!(
        "<RSAKeyValue><Modulus>{}</Modulus><Exponent>{}</Exponent><P>{}</P><Q>{}</Q><D>{}</D></RSAKeyValue>",
        STANDARD.encode(key.n().to_bytes_be()),
        STANDARD.encode(key.e().to_bytes_be()),
        STANDARD.encode(key.primes()[0].to_bytes_be()),
        STANDARD.encode(key.primes()[1].to_bytes_be()),
        STANDARD.encode(key.d().to_bytes_be()),
    );
    let credentials = Credentials::from_base64_xml(
        "consumer-id",
        SecretString::from("consumer-secret"),
        SecretString::from(STANDARD.encode(xml)),
    )
    .expect("fixture credentials parse");
    (credentials, public_key)
}

pub fn fixture_token(kind: &str) -> String {
    let expires_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time follows epoch")
        .as_secs()
        .checked_add(3_600)
        .expect("fixture expiry fits u64");
    let claims = URL_SAFE_NO_PAD.encode(format!(r#"{{"exp":{expires_at},"kind":"{kind}"}}"#));
    format!("header.{claims}.signature")
}
