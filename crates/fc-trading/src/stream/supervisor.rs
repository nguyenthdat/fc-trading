use futures_util::{SinkExt as _, StreamExt as _};
use reqwest::StatusCode;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use super::StreamEvent;
use super::connection::{Session, connect};
use super::protocol::decode_events;
use crate::{Error, Result, StreamOptions, TradingClient};

pub(super) async fn run(
    client: TradingClient,
    options: StreamOptions,
    mut session: Session,
    sender: mpsc::Sender<Result<StreamEvent>>,
    cancellation: CancellationToken,
) -> Result<()> {
    loop {
        if run_session(&mut session, &sender, &cancellation).await? == SessionOutcome::Cancelled {
            return Ok(());
        }
        let reconnect = options.reconnect();
        let mut delay = reconnect.initial_delay();
        loop {
            tokio::select! {
                () = cancellation.cancelled() => return Ok(()),
                () = tokio::time::sleep(delay) => {}
            }
            match connect(&client, &options).await {
                Ok(connected) => {
                    session = connected;
                    break;
                }
                Err(error) => {
                    let terminal = matches!(
                        &error,
                        Error::Http(http)
                            if http.status() == StatusCode::UNAUTHORIZED
                                || http.status() == StatusCode::FORBIDDEN
                    );
                    if send(&sender, Err(error), &cancellation).await == SendOutcome::Closed {
                        return Ok(());
                    }
                    if terminal {
                        return Ok(());
                    }
                    delay = delay.saturating_mul(2).min(reconnect.maximum_delay());
                }
            }
        }
    }
}

async fn run_session(
    session: &mut Session,
    sender: &mpsc::Sender<Result<StreamEvent>>,
    cancellation: &CancellationToken,
) -> Result<SessionOutcome> {
    loop {
        tokio::select! {
            () = cancellation.cancelled() => {
                session.send(Message::Close(None)).await.map_err(Error::WebSocket)?;
                return Ok(SessionOutcome::Cancelled);
            }
            message = session.next() => {
                match message {
                    Some(Ok(Message::Text(text))) => {
                        match decode_events(text.as_str()) {
                            Ok(events) => {
                                for event in events {
                                    if send(sender, Ok(event), cancellation).await == SendOutcome::Closed {
                                        return Ok(SessionOutcome::Cancelled);
                                    }
                                }
                            }
                            Err(error) => {
                                if send(sender, Err(error), cancellation).await == SendOutcome::Closed {
                                    return Ok(SessionOutcome::Cancelled);
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Close(_)) | Err(_)) | None => {
                        return Ok(SessionOutcome::Disconnected);
                    }
                    Some(Ok(Message::Binary(_) | Message::Ping(_) | Message::Pong(_) | Message::Frame(_))) => {}
                }
            }
        }
    }
}

async fn send(
    sender: &mpsc::Sender<Result<StreamEvent>>,
    event: Result<StreamEvent>,
    cancellation: &CancellationToken,
) -> SendOutcome {
    tokio::select! {
        () = cancellation.cancelled() => SendOutcome::Closed,
        result = sender.send(event) => {
            if result.is_ok() { SendOutcome::Sent } else { SendOutcome::Closed }
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum SessionOutcome {
    Cancelled,
    Disconnected,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum SendOutcome {
    Sent,
    Closed,
}
