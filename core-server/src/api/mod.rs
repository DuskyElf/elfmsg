use axum::{self, routing, Json};
use serde::{Deserialize, Serialize};

pub fn router() -> axum::Router {
    axum::Router::new().route("/ping", routing::post(ping))
}

#[derive(Serialize, Deserialize, Debug)]
struct PingPayload {
    payload: u64,
}

// Takes in `u64` payload and adds 1 to it
async fn ping(Json(ping_payload): Json<PingPayload>) -> Json<PingPayload> {
    PingPayload {
        payload: ping_payload.payload + 1,
    }
    .into()
}

#[tokio::test]
async fn ping_test() {
    use super::ServerConfig;
    let conf = ServerConfig::get();

    let json_payload = PingPayload { payload: 42 };
    let response = reqwest::Client::new()
        .post(format!(
            "http://127.0.0.1:{}/api/ping",
            conf.socket_addr.port()
        ))
        .json(&json_payload)
        .send()
        .await
        .expect("Failed to send request, MAKE SURE the server is RUNNING");

    assert!(response.status().is_success());
    assert!(matches!(
        response.json::<PingPayload>().await.unwrap(),
        PingPayload { payload: 43 }
    ));
}
