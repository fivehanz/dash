mod configs;
mod db;
mod services;

use crate::db::connection::Connection;

use configs::Configs;

use tracing::debug;

#[tokio::main]
async fn main() {
    // initialize tracing subscriber
    let _subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // ! configure tracing to start with default
    // ! until config.loglevel becomes available

    // initialize environment configs
    let config: Configs = Configs::init();
    debug!("{:#?}", &config);

    // finalize tracing subscriber
    debug!("log_level={}", config.log_level());

    // initialize database connection
    let _db = Connection::new().init(&config).await;

    // start grpc and app/http server concurrently
    tokio::try_join!(
        services::grpc::start_server(config.grpc_port),
        services::router::start_server(config.app_port)
    )
    .unwrap();
}
