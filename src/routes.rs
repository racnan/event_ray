use axum::{
    routing::{get, post},
    Router,
};

use crate::{app_state::AppState, handlers};

/// Creates the main application router.
/// Takes an `AppState` instance which is shared across handlers
/// and configures all the necessary application routes.
pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route(
            "/api/events",
            post(handlers::publish_event_handler).with_state(app_state.clone()),
        )
        .route("/sse", get(handlers::sse_handler).with_state(app_state))
}
