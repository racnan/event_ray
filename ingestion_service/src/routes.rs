use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{health_check, ingest_event_handler};

/// Creates the main Axum router for the ingestion service.
/// This function sets up all the routes and maps them to their respective handlers.
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/events", post(ingest_event_handler))
}
