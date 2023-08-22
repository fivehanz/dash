use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router, Server};
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::{info, Level};

/// Creates a router for handling HTTP requests.
pub fn create_router() -> Router {
    // creates a TraceLayer for HTTP tracing with INFO level logging for spans and response handling.
    let http_trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Micros),
        );

    // declare all routes in app
    let router: Router = Router::new()
        .route("/healthz", get(health_check_handler)) // health check
        .fallback(not_found_handler)
        .layer(http_trace_layer);

    router
}

pub async fn start_server(port: u16) -> Result<(), tokio::task::JoinError> {
    tokio::spawn(async move {
        // app ip address and port
        let app_addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));
        info!("start app/http server on {}", &app_addr);

        Server::bind(&app_addr)
            .serve(self::create_router().into_make_service())
            .await
            .unwrap();
    })
    .await
}

/// Handler for health check endpoint.
async fn health_check_handler() -> impl IntoResponse {
    (StatusCode::OK, Json("ok")) // ! convert it to health check
}

/// Handler for not found endpoint.
async fn not_found_handler() -> impl IntoResponse {
    Json("oh no! 404 not found.")
}
