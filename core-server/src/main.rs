use axum::{self, routing};
use elfmsg::*;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();
    let conf = ServerConfig::get();

    let app = axum::Router::new().route("/", routing::get(hello_world));

    info!("Starting server on ServerAddr: {}", conf.socket_addr);

    axum::Server::bind(&conf.socket_addr)
        .serve(app.into_make_service())
        .await
        .map_err(|err| {
            eprintln!("ServerError: {}", err);
            ()
        })?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello world"
}
