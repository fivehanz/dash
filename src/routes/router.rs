use crate::db::user::User;
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, MethodRouter},
    Json, Router,
};
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tracing::{debug, warn, Level};
use uuid::Uuid;

/// Creates a router for handling HTTP requests.
pub fn create_router() -> Router {
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
        .route("/create_user", get(create_user))
        .route("/read_user/:id", get(read_user))
        // .fallback(not_found_handler)
        .layer(http_trace_layer);

    router
}

/// Handler for health check endpoint.
async fn health_check_handler() -> impl IntoResponse {
    (StatusCode::OK, Json("ok")) // ! convert it to health check
}

/// Handler for not found endpoint.
async fn not_found_handler() -> impl IntoResponse {
    Json("oh no! 404 not found.")
}

// test create user router
async fn create_user() -> impl IntoResponse {
    match User::new(
        "example@email.com".to_string(),
        "password".to_string(),
        "hanz".to_string(),
        "Haniel".to_string(),
    )
    .await
    {
        Ok(user) => {
            debug!("processed create_user request {:#?}", &user);
            Ok(Json(user))
        }
        Err(e) => {
            warn!("failed to process create_user, {:#?}", &e);
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
    }
}

async fn read_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    match User::get_user(id).await {
        Some(user) => {
            debug!("processed get_user request {:#?}", &user);
            Ok(Json(user))
        }
        None => {
            warn!("failed to process get_user, {:?}", &id);
            Err((StatusCode::BAD_REQUEST, Json("user not found")))
        }
    }
}
