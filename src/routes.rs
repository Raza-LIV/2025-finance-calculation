use axum::{routing::get, Router};
use crate::handlers;

/// Creates the main application router with all routes configured
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
}
