use serde::Deserialize;
use serde_json::Value;

use crate::{Error, Result};

pub(super) const CLIENT_PROTOCOL: &str = "1.3";
pub(super) const CONNECTION_DATA: &str = r#"[{"name": "BroadcastHubV2"}]"#;
const HUB_NAME: &str = "BroadcastHubV2";

#[derive(Deserialize)]
pub(super) struct Negotiation {
    #[serde(rename = "ConnectionToken")]
    pub(super) connection_token: String,
    #[serde(rename = "ProtocolVersion")]
    pub(super) protocol_version: String,
}

#[derive(Deserialize)]
struct Envelope {
    #[serde(default, rename = "E")]
    error: Option<Value>,
    #[serde(default, rename = "M")]
    messages: Vec<Invocation>,
}

#[derive(Deserialize)]
struct Invocation {
    #[serde(rename = "H")]
    hub: String,
    #[serde(rename = "M")]
    method: String,
    #[serde(default, rename = "A")]
    arguments: Vec<Value>,
}

pub(super) fn decode_events(text: &str) -> Result<Vec<super::StreamEvent>> {
    let envelope: Envelope = serde_json::from_str(text)
        .map_err(|error| Error::SignalRProtocol(format!("invalid server frame: {error}")))?;
    let mut events = Vec::with_capacity(envelope.messages.len().saturating_add(1));
    if let Some(error) = envelope.error {
        events.push(super::StreamEvent::ServerError(value_text(&error)));
    }
    for invocation in envelope.messages {
        if !invocation.hub.eq_ignore_ascii_case(HUB_NAME) {
            continue;
        }
        let Some(argument) = invocation.arguments.first() else {
            continue;
        };
        match invocation.method.as_str() {
            "Broadcast" => events.push(super::StreamEvent::Broadcast(value_text(argument))),
            "Error" => events.push(super::StreamEvent::ServerError(value_text(argument))),
            _ => {}
        }
    }
    Ok(events)
}

fn value_text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}
