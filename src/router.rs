use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "homepage" }))
        .route("/health", get(|| async { "good" }))
}
