mod configs;
mod routes;

use axum::Server;
use configs::Configs;
use std::net::SocketAddr;
use tracing::warn;

#[tokio::main]
async fn main() {
    // initialize environment configs
    let config = Configs::new().unwrap();

    // initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(config.log_level())
        .pretty()
        .init();

    // log congfigs
    tracing::trace!("{:#?}", &config);

    // ip address and port
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], config.port));
    warn!("Listening on {}", &addr);

    // start server
    Server::bind(&addr)
        .serve(routes::router::create_router().into_make_service())
        .await
        .unwrap();
}
