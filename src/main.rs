mod configs;
mod db;
mod routes;

use axum::Server;
use configs::Configs;
use std::net::SocketAddr;
use tracing::{info, warn};

#[tokio::main]
async fn main() {
    // initialize tracing subscriber
    tracing_subscriber::fmt().compact().init();

    // initialize environment configs
    let config = Configs::init_configs();

    // finalize tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(config.log_level())
        .finish();

    // log configs
    info!("{:#?}", &config);

    // ip address and port
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], config.port));
    warn!("started server on {}", &addr);

    // start server
    Server::bind(&addr)
        .serve(routes::router::create_router().into_make_service())
        .await
        .unwrap();
}
