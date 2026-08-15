#![cfg(test)]

use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::SeedableRng as _;
use rand_chacha::ChaCha20Rng;
use rsa::{RsaPrivateKey, traits::PrivateKeyParts as _, traits::PublicKeyParts as _};
use secrecy::SecretString;
use ssi_fc_trading::Credentials;

#[test]
fn credentials_debug_redacts_secret_and_private_key_when_constructed() {
    // Given
    let encoded_key = fixture_xml_key();
    let secret = "consumer-secret-fixture";

    // When
    let credentials = Credentials::from_base64_xml(
        "consumer-id-fixture",
        SecretString::from(secret),
        SecretString::from(encoded_key.clone()),
    )
    .expect("fixture key parses");
    let debug = format!("{credentials:?}");

    // Then
    assert!(!debug.contains(secret));
    assert!(!debug.contains(&encoded_key));
}

fn fixture_xml_key() -> String {
    let mut rng = ChaCha20Rng::seed_from_u64(7);
    let key = RsaPrivateKey::new(&mut rng, 1_024).expect("fixture key generation");
    let xml = format!(
        "<RSAKeyValue><Modulus>{}</Modulus><Exponent>{}</Exponent><P>{}</P><Q>{}</Q><DP></DP><DQ></DQ><InverseQ></InverseQ><D>{}</D></RSAKeyValue>",
        STANDARD.encode(key.n().to_bytes_be()),
        STANDARD.encode(key.e().to_bytes_be()),
        STANDARD.encode(key.primes()[0].to_bytes_be()),
        STANDARD.encode(key.primes()[1].to_bytes_be()),
        STANDARD.encode(key.d().to_bytes_be()),
    );
    STANDARD.encode(xml)
}
