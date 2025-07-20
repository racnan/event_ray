use crate::app_state::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use chrono::Utc;
use event_ray_core::{api_models::PublishRequest, app_event::AppEvent};
use uuid::Uuid;

/// Handles health check requests.
/// Returns a static string "OK" to indicate the service is running.
pub async fn health_check() -> &'static str {
    "OK"
}

/// Handles incoming event ingestion requests.
/// It takes a JSON payload (`PublishRequest`), creates an `AppEvent`,
/// and uses the `EventPublisher` from the shared state to forward it.
/// Returns `StatusCode::ACCEPTED` on success or `StatusCode::INTERNAL_SERVER_ERROR` on failure.
pub async fn ingest_event_handler(
    State(state): State<AppState>,
    Json(publish_request): Json<PublishRequest>,
) -> Result<StatusCode, StatusCode> {
    // Construct an AppEvent
    let event = AppEvent {
        id: Uuid::new_v4(),
        ray_id: publish_request.ray_id,
        timestamp: Utc::now(),
        payload: publish_request.payload,
    };

    // Publish the event
    if let Err(e) = state.publisher.publish(&event).await {
        eprintln!("Failed to publish event: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    println!("Ingestion service successfully published event: {:?}", event);

    // Return 202 Accepted
    Ok(StatusCode::ACCEPTED)
}
