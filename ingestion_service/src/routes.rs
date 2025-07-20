use crate::{app_state::AppState, handlers::health_check, handlers::ingest_event_handler};
use axum::{
    routing::{get, post},
    Router,
};

/// Creates the main Axum router for the ingestion service.
/// This function sets up all the routes and maps them to their respective handlers.
pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/events", post(ingest_event_handler))
        .with_state(app_state)
}
