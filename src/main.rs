mod configs;
mod db;
mod routes;

use axum::Server;
use configs::Configs;
use std::net::SocketAddr;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    // initialize tracing subscriber
    tracing_subscriber::fmt().finish();

    // initialize environment configs
    let config: Configs = Configs::init_configs();

    // finalize tracing subscriber
    tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(config.log_level())
        .init();

    // log configs
    debug!("{:?}", &config);

    // initialize database connection
    let db: Result<surrealdb::opt::auth::Jwt, surrealdb::Error> =
        db::connection::init(config.clone()).await;
    debug!("{:?}", &db);

    // ip address and port
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("started server on {}", &addr);

    // start server
    Server::bind(&addr)
        .serve(routes::router::create_router().into_make_service())
        .await
        .unwrap();
}
