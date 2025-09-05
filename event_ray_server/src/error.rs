use event_ray_core::app_event::AppEvent;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to send event to broadcast channel")]
    BroadcastSend(#[from] tokio::sync::broadcast::error::SendError<AppEvent>),
    #[error("Failed to receive event from broadcast channel")]
    BroadcastRecv(#[from] tokio::sync::broadcast::error::RecvError),
}
