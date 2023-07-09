use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router, Server,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // declare all routes
    let routes: Router = Router::new()
        .route("/healthz", get(hello_handler))
        .nest_service("/", get_service(ServeDir::new("./web/build/")));

    // ip address and port
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr); // ! convert to tracing

    // start server
    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn hello_handler() -> impl IntoResponse {
    Html("<h1>Hello, from the other side!</h1>") // ! convert it to health check
}

async fn static_web_router() -> Router {
    todo!("add static web router fn")
}
