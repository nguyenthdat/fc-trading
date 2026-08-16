use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::body::to_bytes;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse as _, Response};
use axum::routing::any;
use serde_json::Value;
use ssi_fc_trading::{ClientConfig, DecimalNumber, NewOrderRequest};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::fixtures::fixture_token;

#[derive(Debug)]
pub struct RecordedRequest {
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

#[derive(Clone)]
struct ServerState {
    requests: mpsc::Sender<RecordedRequest>,
    failure: Option<Failure>,
}

#[derive(Clone)]
struct Failure {
    path: String,
    status: StatusCode,
    body: String,
}

pub struct TestServer {
    base_url: String,
    requests: Arc<Mutex<mpsc::Receiver<RecordedRequest>>>,
    shutdown: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<()>>,
}

impl TestServer {
    pub async fn start() -> Self {
        Self::start_with_failure(None).await
    }

    pub async fn failing(path: &str, status: StatusCode, body: &str) -> Self {
        Self::start_with_failure(Some(Failure {
            path: path.to_owned(),
            status,
            body: body.to_owned(),
        }))
        .await
    }

    async fn start_with_failure(failure: Option<Failure>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("test listener binds");
        let address = listener.local_addr().expect("test listener address");
        let (request_sender, request_receiver) = mpsc::channel(64);
        let state = ServerState {
            requests: request_sender,
            failure,
        };
        let router = Router::new().fallback(any(capture)).with_state(state);
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();
        let task = tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async {
                    let _result = shutdown_receiver.await;
                })
                .await
                .expect("test server runs");
        });
        Self {
            base_url: format!("http://{address}/"),
            requests: Arc::new(Mutex::new(request_receiver)),
            shutdown: Some(shutdown_sender),
            task: Some(task),
        }
    }

    pub fn config(&self) -> ClientConfig {
        ClientConfig::new(&self.base_url, &self.base_url).expect("loopback URL is accepted")
    }

    pub async fn next_request(&self) -> RecordedRequest {
        tokio::time::timeout(Duration::from_secs(2), self.requests.lock().await.recv())
            .await
            .expect("request arrives before timeout")
            .expect("request channel remains open")
    }

    pub async fn has_pending_request(&self) -> bool {
        self.requests.lock().await.try_recv().is_ok()
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

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _result = shutdown.send(());
        }
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

async fn capture(State(state): State<ServerState>, request: Request) -> Response {
    let (parts, body) = request.into_parts();
    let body = to_bytes(body, 2 * 1024 * 1024)
        .await
        .expect("test request body is bounded")
        .to_vec();
    let recorded = RecordedRequest {
        method: parts.method.to_string(),
        path: parts.uri.path().to_owned(),
        query: parts.uri.query().map(ToOwned::to_owned),
        headers: parts.headers,
        body: body.clone(),
    };
    state
        .requests
        .send(recorded)
        .await
        .expect("test capture receiver remains open");

    match &state.failure {
        Some(failure) if parts.uri.path() == failure.path => {
            return (failure.status, failure.body.clone()).into_response();
        }
        Some(_) | None => {}
    }
    if parts.uri.path() == "/api/v2/Trading/AccessToken" {
        let body: Value = serde_json::from_slice(&body).expect("token request is JSON");
        let kind = if body["isSave"].as_bool() == Some(true) {
            "write"
        } else {
            "read"
        };
        let token = fixture_token(kind);
        return (
            StatusCode::OK,
            [("content-type", "application/json")],
            format!(r#"{{"status":200,"message":"ok","data":{{"accessToken":"{token}"}}}}"#),
        )
            .into_response();
    }
    if parts.uri.path() == "/api/v2/Trading/NewOrder" {
        let fixtures: Value =
            serde_json::from_str(include_str!("../fixtures/core_order_responses.json"))
                .expect("order fixtures are valid JSON");
        return (
            StatusCode::OK,
            [("content-type", "application/json")],
            fixtures["new_order"].to_string(),
        )
            .into_response();
    }
    (
        StatusCode::OK,
        [("content-type", "application/json")],
        r#"{"status":200,"message":"ok","data":{}}"#,
    )
        .into_response()
}

pub fn sample_new_order() -> NewOrderRequest {
    NewOrderRequest {
        account: "123456".to_owned(),
        request_id: "42".to_owned(),
        instrument_id: "FPT".to_owned(),
        market: "VN".to_owned(),
        buy_sell: "B".to_owned(),
        order_type: "LO".to_owned(),
        price: "12345.5".parse().expect("fixture decimal"),
        quantity: 10,
        stop_order: false,
        stop_price: DecimalNumber::ZERO,
        stop_type: String::new(),
        stop_step: DecimalNumber::ZERO,
        loss_step: DecimalNumber::ZERO,
        profit_step: DecimalNumber::ZERO,
        channel_id: "TA".to_owned(),
        code: String::new(),
        device_id: String::new(),
        user_agent: String::new(),
        modifiable: None,
    }
}
