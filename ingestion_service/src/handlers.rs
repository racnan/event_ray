use crate::app_state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use error_stack::ResultExt;
use event_ray_core::{
    api_models::PublishRequest,
    app_event::AppEvent,
    error::{ApiError, ApiErrorResponse},
};
use uuid::Uuid;

/// Handles health check requests.
/// Returns a static string "OK" to indicate the service is running.
pub async fn health_check() -> &'static str {
    "OK"
}

/// Handles incoming event ingestion requests.
/// It takes a JSON payload (`PublishRequest`), creates an `AppEvent`,
/// and uses the `EventPublisher` from the shared state to forward it.
/// Returns `StatusCode::ACCEPTED` on success or `ApiErrorResponse` on failure.
pub async fn ingest_event_handler(
    State(state): State<AppState>,
    Json(publish_request): Json<PublishRequest>,
) -> Result<impl IntoResponse, ApiErrorResponse> {
    // Construct an AppEvent
    let event = AppEvent {
        id: Uuid::new_v4(),
        ray_id: publish_request.ray_id,
        timestamp: Utc::now(),
        payload: publish_request.payload,
    };

    // Publish the event
    state
        .publisher
        .publish(&event)
        .await
        .change_context(ApiError::InternalServerError)?;

    println!(
        "Ingestion service successfully published event: {:?}",
        event
    );

    // Return 202 Accepted
    Ok(StatusCode::ACCEPTED)
}
