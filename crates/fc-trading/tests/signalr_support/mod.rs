use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::{IntoResponse as _, Response};
use axum::routing::{get, post};
use futures_util::StreamExt as _;
use ssi_fc_trading::ClientConfig;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::fixtures::fixture_token;

#[derive(Debug)]
struct SignalRequest {
    path: String,
    query: String,
    headers: HeaderMap,
}

#[derive(Clone)]
struct SignalState {
    captures: mpsc::Sender<SignalRequest>,
    connections: Arc<AtomicUsize>,
    close_first: bool,
}

pub struct SignalServer {
    base_url: String,
    captures: Arc<Mutex<mpsc::Receiver<SignalRequest>>>,
    shutdown: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<()>>,
}

impl SignalServer {
    pub async fn start(close_first: bool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("signal test listener binds");
        let address = listener.local_addr().expect("signal test address");
        let (capture_sender, capture_receiver) = mpsc::channel(16);
        let state = SignalState {
            captures: capture_sender,
            connections: Arc::new(AtomicUsize::new(0)),
            close_first,
        };
        let router = Router::new()
            .route("/api/v2/Trading/AccessToken", post(access_token))
            .route("/v2.0/signalr/negotiate", post(negotiate))
            .route("/v2.0/signalr/connect", get(connect))
            .with_state(state);
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async {
                    let _result = shutdown_receiver.await;
                })
                .await
                .expect("signal test server runs");
        });
        Self {
            base_url: format!("http://{address}/"),
            captures: Arc::new(Mutex::new(capture_receiver)),
            shutdown: Some(shutdown_sender),
            task: Some(task),
        }
    }

    pub fn config(&self) -> ClientConfig {
        ClientConfig::new(&self.base_url, &self.base_url).expect("loopback URL is accepted")
    }

    async fn next_capture(&self) -> SignalRequest {
        tokio::time::timeout(Duration::from_secs(3), self.captures.lock().await.recv())
            .await
            .expect("SignalR request arrives")
            .expect("capture channel remains open")
    }

    pub async fn close(mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _result = shutdown.send(());
        }
        if let Some(task) = self.task.take() {
            let _result = task.await;
        }
    }
}

impl Drop for SignalServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _result = shutdown.send(());
        }
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

pub async fn assert_signal_requests(server: &SignalServer, notify_id: &str) {
    let negotiate = server.next_capture().await;
    assert_eq!(negotiate.path, "/v2.0/signalr/negotiate");
    assert_eq!(negotiate.headers["notifyid"], notify_id);
    assert_eq!(
        negotiate.headers["authorization"],
        format!("Bearer {}", fixture_token("read"))
    );
    let negotiate_query: BTreeMap<_, _> =
        url::form_urlencoded::parse(negotiate.query.as_bytes()).collect();
    assert_eq!(negotiate_query["clientProtocol"], "1.3");
    assert_eq!(
        negotiate_query["connectionData"],
        r#"[{"name": "BroadcastHubV2"}]"#
    );

    let connect = server.next_capture().await;
    assert_eq!(connect.path, "/v2.0/signalr/connect");
    assert_eq!(connect.headers["notifyid"], notify_id);
    let connect_query: BTreeMap<_, _> =
        url::form_urlencoded::parse(connect.query.as_bytes()).collect();
    assert_eq!(connect_query["transport"], "webSockets");
    assert_eq!(connect_query["clientProtocol"], "1.3");
    assert_eq!(connect_query["connectionToken"], "connection token/+");
}

async fn access_token() -> Response {
    let token = fixture_token("read");
    (
        StatusCode::OK,
        [("content-type", "application/json")],
        format!(r#"{{"status":200,"message":"ok","data":{{"accessToken":"{token}"}}}}"#),
    )
        .into_response()
}

async fn negotiate(State(state): State<SignalState>, headers: HeaderMap, uri: Uri) -> Response {
    state
        .captures
        .send(SignalRequest {
            path: uri.path().to_owned(),
            query: uri.query().unwrap_or_default().to_owned(),
            headers,
        })
        .await
        .expect("capture receiver remains open");
    (
        StatusCode::OK,
        [("content-type", "application/json")],
        r#"{"ConnectionToken":"connection token/+","ProtocolVersion":"1.3"}"#,
    )
        .into_response()
}

async fn connect(
    websocket: WebSocketUpgrade,
    State(state): State<SignalState>,
    headers: HeaderMap,
    uri: Uri,
) -> Response {
    state
        .captures
        .send(SignalRequest {
            path: uri.path().to_owned(),
            query: uri.query().unwrap_or_default().to_owned(),
            headers,
        })
        .await
        .expect("capture receiver remains open");
    let connection = state.connections.fetch_add(1, Ordering::SeqCst);
    websocket.on_upgrade(move |socket| serve_socket(socket, state.close_first, connection))
}

async fn serve_socket(mut socket: WebSocket, close_first: bool, connection: usize) {
    let payload = if close_first {
        if connection == 0 { "first" } else { "second" }
    } else {
        "payload"
    };
    let frame = if close_first {
        format!(r#"{{"M":[{{"H":"broadcastHubV2","M":"Broadcast","A":["{payload}"]}}]}}"#)
    } else {
        r#"{"M":[{"H":"broadcastHubV2","M":"Broadcast","A":["payload"]},{"H":"BroadcastHubV2","M":"Error","A":["problem"]}]}"#.to_owned()
    };
    socket
        .send(Message::Text(frame.into()))
        .await
        .expect("test frame sends");
    if close_first && connection == 0 {
        socket
            .send(Message::Close(None))
            .await
            .expect("test close sends");
        return;
    }
    while let Some(message) = socket.next().await {
        if matches!(message, Ok(Message::Close(_)) | Err(_)) {
            break;
        }
    }
}
