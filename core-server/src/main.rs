use axum::{self, routing};
use elfmsg::*;

#[tokio::main]
async fn main() {
    let conf = ServerConfig::get();

    let app = axum::Router::new().route("/", routing::get(hello_world));

    axum::Server::bind(&conf.socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> &'static str {
    "Hello world"
}
