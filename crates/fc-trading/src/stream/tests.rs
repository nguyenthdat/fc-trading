use super::StreamEvent;
use super::protocol::decode_events;

#[test]
fn top_level_error_and_matching_hub_messages_preserve_order_when_decoded() -> crate::Result<()> {
    // Given
    let frame = r#"{"E":"top","M":[{"H":"broadcastHubV2","M":"Broadcast","A":["data"]},{"H":"BroadcastHubV2","M":"Error","A":["hub"]},{"H":"other","M":"Broadcast","A":["ignored"]}]}"#;

    // When
    let events = decode_events(frame)?;

    // Then
    assert_eq!(
        events,
        vec![
            StreamEvent::ServerError("top".to_owned()),
            StreamEvent::Broadcast("data".to_owned()),
            StreamEvent::ServerError("hub".to_owned()),
        ]
    );
    Ok(())
}
