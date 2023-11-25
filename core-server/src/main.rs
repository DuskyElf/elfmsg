use axum::{self, routing};
use elfmsg::*;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let conf = ServerConfig::get();

    let app = axum::Router::new().route("/", routing::get(hello_world));

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
