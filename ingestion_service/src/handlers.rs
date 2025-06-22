use axum::{extract::Json, http::StatusCode};
use event_ray_core::{api_models::PublishRequest, app_event::AppEvent};
use uuid::Uuid;
use chrono::Utc;

/// Handles health check requests.
/// Returns a static string "OK" to indicate the service is running.
pub async fn health_check() -> &'static str {
    "OK"
}

/// Handles incoming event ingestion requests.
/// It takes a JSON payload (`PublishRequest`).
/// Creates an `AppEvent` with a new UUID and current timestamp.
/// For this version, it logs the successfully created `AppEvent`.
/// Returns `StatusCode::ACCEPTED` on success.
pub async fn ingest_event_handler(
    Json(publish_request): Json<PublishRequest>,
) -> Result<StatusCode, StatusCode> {
    // Construct an AppEvent
    let event = AppEvent {
        id: Uuid::new_v4(),
        ray_id: publish_request.ray_id,
        timestamp: Utc::now(),
        payload: publish_request.payload,
    };

    // For this version, log the successfully created AppEvent
    println!("Ingestion service received event: {:?}", event);

    // Return 202 Accepted
    Ok(StatusCode::ACCEPTED)
}
