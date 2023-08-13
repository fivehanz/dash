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
    tracing_subscriber::fmt()
        .pretty()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .init();

    // initialize environment configs
    let config: Configs = Configs::init_configs();
    // log configs
    info!("{:?}", &config);

    // finalize tracing subscriber
    tracing_subscriber::fmt()
        .with_max_level(config.log_level())
        .finish();

    // initialize database connection
    let db = db::connection::init(config.clone()).await;
    info!("{:?}", &db);

    // ip address and port
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], config.port));
    warn!("started server on {}", &addr);

    // start server
    Server::bind(&addr)
        .serve(routes::router::create_router().into_make_service())
        .await
        .unwrap();
}
