use serde::{Deserialize, Serialize};

/// Represents the structure of an incoming event publishing request.
/// Contains the `ray_id` to identify the event stream and the `payload` (content) of the event.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishRequest {
    pub ray_id: String,
    pub payload: String,
}

/// Represents the query parameters for an SSE (Server-Sent Events) request.
/// Contains the `ray` (ray_id) to filter events for a specific stream.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SseParams {
    pub ray: String,
}
