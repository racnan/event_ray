use tokio::sync::broadcast;

use event_ray_core::app_event::AppEvent;
/// Represents the shared application state.
/// This struct is used to share resources, like the event broadcast sender,
/// across different parts of the application (e.g., Axum handlers).
/// It can be cloned and passed to handlers that require access to shared state.
#[derive(Clone)]
pub struct AppState {
    /// A broadcast sender for `AppEvent`s.
    /// Used to send events to all subscribed SSE clients.
    pub event_sender: broadcast::Sender<AppEvent>,
}
