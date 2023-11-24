use axum::{self, routing};

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", routing::get(hello_world));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> &'static str {
    "Hello world"
}
