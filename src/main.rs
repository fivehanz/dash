mod configs;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, MethodRouter},
    Json, Router, Server,
};
use configs::Configs;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tracing::{warn, Level};

#[tokio::main]
async fn main() {
    // initialize environment configs
    let config = Configs::new().unwrap();

    // initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(config.log_level())
        .compact()
        .init();

    // log congfigs
    tracing::trace!("{:?}", &config);

    // creates a TraceLayer for HTTP tracing with INFO level logging for spans and response handling.
    let http_trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    // declare static web router w/ not found fallback
    let static_router_service: MethodRouter =
        get_service(ServeDir::new("./dist").not_found_service(get(not_found_handler)));

    // declare all routes in app
    let router: Router = Router::new()
        .fallback_service(static_router_service)
        // .nest_service("/", static_router_service) // static web router
        .route("/healthz", get(health_check_handler)) // health check
        // .fallback(not_found_handler)
        .layer(http_trace_layer);

    // ip address and port
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], config.port));
    // info log
    warn!("Listening on {}", addr);

    // start server
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn health_check_handler() -> impl IntoResponse {
    (StatusCode::OK, Json("ok")) // ! convert it to health check
}

async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json("oh no! 404 not found."))
}
