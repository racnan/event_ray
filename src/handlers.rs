use crate::{
    api_models::{PublishRequest, SseParams},
    app_state::AppState,
    event::AppEvent,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use uuid::Uuid;

/// Handles health check requests.
/// Returns a static string "Event Ray is Up" to indicate the server is running.
pub async fn health_check() -> &'static str {
    "Event Ray is Up"
}

/// Handles incoming event publishing requests.
/// It takes the application state and a JSON payload (`PublishRequest`).
/// Creates an `AppEvent` with a new UUID and current timestamp,
/// then sends it through the broadcast channel in the `AppState`.
/// Returns `StatusCode::OK` on success, or `StatusCode::INTERNAL_SERVER_ERROR` on failure.
pub async fn publish_event_handler(
    State(state): State<AppState>,
    Json(publish_request): Json<PublishRequest>,
) -> StatusCode {
    let event = AppEvent {
        id: Uuid::new_v4(),
        ray_id: publish_request.ray_id,
        timestamp: Utc::now(),
        payload: publish_request.payload,
    };

    match state.event_sender.send(event) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

use async_stream::stream;
use axum::{
    response::sse::{Event, Sse},
    response::IntoResponse,
};
use std::convert::Infallible;

/// Handles Server-Sent Events (SSE) requests.
/// Takes the application state and SSE parameters (`SseParams`) from the query string.
/// Subscribes to the event broadcast channel.
/// Filters events based on the `ray_id` provided in `SseParams`.
/// Yields matching events to the client as SSE `Event`s.
/// Includes a keep-alive mechanism.
pub async fn sse_handler(
    State(state): State<AppState>,
    Query(params): Query<SseParams>,
) -> impl IntoResponse {
    println!("handler invoked");
    let mut rx = state.event_sender.subscribe();

    let stream = stream! {
        while let Ok(event) = rx.recv().await {
    println!("app event: {:?}", event);
            if event.ray_id == params.ray {
                yield Result::<Event, Infallible>::Ok(
                    Event::default().data(event.payload)
                );
            }
        }
    };

    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::default())
        .into_response()
}
