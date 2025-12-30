use async_stream::stream;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{
        sse::{Event, Sse},
        IntoResponse, Json,
    },
};
use chrono::Utc;
use error_stack::Report;
use std::convert::Infallible;
use tokio::sync::broadcast::error::RecvError;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::Error;
use event_ray_core::{
    api_models::{PublishRequest, SseParams},
    app_event::AppEvent,
    error::{ApiError, ApiErrorResponse},
};

/// Handles health check requests.
/// Returns a static string "Event Ray is Up" to indicate the server is running.
pub async fn health_check() -> &'static str {
    "Event Ray is Up"
}

/// Handles incoming event publishing requests.
/// It takes the application state and a JSON payload (`PublishRequest`).
/// Creates an `AppEvent` with a new UUID and current timestamp,
/// then sends it through the broadcast channel in the `AppState`.
/// Returns `StatusCode::OK` on success, or a `Report<ApiError>` wrapped in `ApiErrorResponse` on failure.
pub async fn publish_event_handler(
    State(state): State<AppState>,
    Json(publish_request): Json<PublishRequest>,
) -> Result<impl IntoResponse, ApiErrorResponse> {
    println!("publisher handler invoked");
    let event = AppEvent {
        id: Uuid::new_v4(),
        ray_id: publish_request.ray_id,
        timestamp: Utc::now(),
        payload: publish_request.payload,
    };

    match state.event_sender.send(event) {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            let report = Report::new(Error::from(e)).change_context(ApiError::InternalServerError);
            Err(report.into())
        }
    }
}

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
    println!("sse handler invoked");
    let mut rx = state.event_sender.subscribe();

    let stream = stream! {
        loop {
            match rx.recv().await {
                Ok(event) => {
                    if event.ray_id == params.ray {
                        yield Result::<Event, Infallible>::Ok(
                            Event::default().data(event.payload)
                        );
                    }
                }
                Err(e) => match e {
                    RecvError::Lagged(missed_count) => {
                        println!("SSE stream lagged, missed {} messages", missed_count);
                        continue;
                    }
                    RecvError::Closed => {
                        let report = Report::new(Error::from(RecvError::Closed));
                        println!("FATAL: SSE stream sender has been dropped, closing connection. This indicates a critical server error. Report: {report:?}");
                        break;
                    }
                },
            }
        }
    };

    Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::default())
        .into_response()
}
