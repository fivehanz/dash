use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, MethodRouter},
    Json, Router, Server,
};
use tower_http::trace::{self, TraceLayer};
use tower_http::{services::ServeDir, trace::Trace};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // initialize tracing subscriber
    // tracing_subscriber::fmt()
    //     .with_target(false)
    //     .compact()
    //     .init();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // creates a TraceLayer for HTTP tracing with INFO level logging for spans and response handling.
    let http_trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    // declare static web router
    let static_router_service: MethodRouter = get_service(ServeDir::new("./dist"));

    // declare all routes in app
    let app: Router = Router::new()
        // .merge(static_web_router)
        .nest_service("/", static_router_service) // static web router
        .route("/healthz", get(hello_handler)) // health check
        .route("/api/v1", get(hello_handler)) // ! add api routes here
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
