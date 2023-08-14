mod configs;
mod db;
mod routes;

use crate::db::connection::Connection;
use axum::Server;
use configs::Configs;
use std::net::SocketAddr;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    // initialize tracing subscriber
    let _subscriber = tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // ! configure tracing to start with default
    // ! until config.loglevel becomes available

    // initialize environment configs
    let config = Configs::init();
    debug!("{:?}", &config);

    // finalize tracing subscriber
    // subscriber.with_max_level(config.log_level()).init();

    // initialize database connection
    let _db = Connection::new().init(&config).await;

    // ip address and port
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("started server on {}", &addr);

    // start server
    Server::bind(&addr)
        .serve(routes::router::create_router().into_make_service())
        .await
        .unwrap();
}
