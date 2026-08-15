use base64::{
    Engine as _, engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD,
};
use rand::SeedableRng as _;
use rand_chacha::ChaCha20Rng;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::signature::Verifier as _;
use rsa::{RsaPrivateKey, RsaPublicKey, traits::PrivateKeyParts as _, traits::PublicKeyParts as _};
use secrecy::SecretString;
use sha2::Sha256;

use super::*;

#[test]
fn signature_is_lowercase_hex_and_verifies_when_body_is_signed() -> Result<()> {
    // Given
    let (encoded, public_key) = fixture_key()?;
    let credentials = Credentials::from_base64_xml(
        "consumer",
        SecretString::from("secret"),
        SecretString::from(encoded),
    )?;
    let body = br#"{"account":"123456"}"#;

    // When
    let encoded_signature = credentials.sign(body)?;

    // Then
    assert!(
        encoded_signature
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    );
    assert_eq!(encoded_signature, encoded_signature.to_lowercase());
    let signature_bytes = hex::decode(encoded_signature)
        .map_err(|error| Error::Authentication(format!("fixture signature hex: {error}")))?;
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|error| Error::Authentication(format!("fixture signature length: {error}")))?;
    VerifyingKey::<Sha256>::new(public_key)
        .verify(body, &signature)
        .map_err(|error| Error::Authentication(format!("fixture signature verify: {error}")))?;
    Ok(())
}

#[test]
fn cached_token_is_invalid_inside_refresh_skew_when_claims_are_parsed() -> Result<()> {
    // Given
    let claims = URL_SAFE_NO_PAD.encode(br#"{"exp":1600}"#);
    let token = CachedToken::parse(format!("header.{claims}.signature"))?;

    // When
    let valid = token.is_valid_at(1_000, 600);

    // Then
    assert!(!valid);
    Ok(())
}

fn fixture_key() -> Result<(String, RsaPublicKey)> {
    let mut rng = ChaCha20Rng::seed_from_u64(11);
    let key = RsaPrivateKey::new(&mut rng, 1_024)
        .map_err(|error| Error::Authentication(format!("fixture key generation: {error}")))?;
    let public_key = RsaPublicKey::from(&key);
    let xml = format!(
        "<RSAKeyValue><Modulus>{}</Modulus><Exponent>{}</Exponent><P>{}</P><Q>{}</Q><D>{}</D></RSAKeyValue>",
        STANDARD.encode(key.n().to_bytes_be()),
        STANDARD.encode(key.e().to_bytes_be()),
        STANDARD.encode(key.primes()[0].to_bytes_be()),
        STANDARD.encode(key.primes()[1].to_bytes_be()),
        STANDARD.encode(key.d().to_bytes_be()),
    );
    Ok((STANDARD.encode(xml), public_key))
}
