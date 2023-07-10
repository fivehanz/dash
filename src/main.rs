use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Json, Router, Server,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // declare static web router
    let static_web_router: Router =
        Router::new().nest_service("/", get_service(ServeDir::new("./dist")));

    // declare all routes
    let routes: Router = Router::new()
        .merge(static_web_router)
        .route("/healthz", get(hello_handler))
        .route("/api/v1", get(hello_handler)); // ! add api routes here

    // ip address and port
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Listening on {}", addr); // ! convert to tracing

    // start server
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn hello_handler() -> impl IntoResponse {
    (StatusCode::OK, Json("ok")) // ! convert it to health check
}
