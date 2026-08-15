use super::*;

#[test]
fn production_urls_are_https_when_default_configuration_is_created() -> Result<()> {
    // Given / When
    let config = ClientConfig::production()?;

    // Then
    assert_eq!(config.api_base_url().scheme(), "https");
    assert_eq!(config.stream_base_url().scheme(), "https");
    Ok(())
}

#[test]
fn non_loopback_http_is_rejected_when_configuration_is_created() {
    // Given / When
    let result = ClientConfig::new("http://example.com", "https://example.com");

    // Then
    assert!(result.is_err());
}

#[test]
fn zero_stream_channel_capacity_is_rejected_when_options_are_built() {
    // Given
    let options = StreamOptions::new("notify-id");

    // When
    let result = options.with_channel_capacity(0);

    // Then
    assert!(matches!(
        result,
        Err(Error::InvalidStreamChannelCapacity { capacity: 0 })
    ));
}
