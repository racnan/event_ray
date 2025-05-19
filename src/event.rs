use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an internal application event.
/// This struct defines the structure of events that are broadcasted within the server
/// and sent to subscribed SSE clients.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppEvent {
    /// A unique identifier for the event.
    pub id: Uuid,
    /// The identifier for the specific event stream (ray) this event belongs to.
    pub ray_id: String,
    /// The Coordinated Universal Time (UTC) timestamp when the event was created.
    pub timestamp: DateTime<Utc>,
    /// The actual content (payload) of the event, as a string.
    pub payload: String,
}
