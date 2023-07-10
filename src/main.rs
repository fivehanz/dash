use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, MethodRouter},
    Json, Router, Server,
};
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .init();

    // creates a TraceLayer for HTTP tracing with INFO level logging for spans and response handling.
    let http_trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    // declare static web router w/ not found fallback
    let static_router_service: MethodRouter =
        get_service(ServeDir::new("./dist").not_found_service(get(not_found_handler)));

    // declare all routes in app
    let app: Router = Router::new()
        .fallback_service(static_router_service)
        // .nest_service("/", static_router_service) // static web router
        .route("/healthz", get(hello_handler)) // health check
        .route("/api/v1", get(hello_handler)) // ! add api routes here
        // .fallback(not_found_handler)
        .layer(http_trace_layer);

    // ip address and port
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8080));
    // info log
    tracing::info!("Listening on {}", addr);

    // start server
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_handler() -> impl IntoResponse {
    (StatusCode::OK, Json("ok")) // ! convert it to health check
}

async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json("404 not found"))
}
