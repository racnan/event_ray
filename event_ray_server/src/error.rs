use event_ray_core::app_event::AppEvent;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to send event to broadcast channel")]
    BroadcastSend(#[from] tokio::sync::broadcast::error::SendError<AppEvent>),
    #[error("Failed to receive event from broadcast channel")]
    BroadcastRecv(#[from] tokio::sync::broadcast::error::RecvError),
    #[cfg(feature = "redis-pubsub")]
    #[error("Redis connection or subscription failed")]
    RedisConnection(#[from] redis::RedisError),
    #[cfg(feature = "redis-pubsub")]
    #[error("Failed to deserialize event from JSON")]
    Deserialization(#[from] serde_json::Error),
    #[cfg(feature = "redis-pubsub")]
    #[error("Redis stream ended unexpectedly")]
    RedisStreamEnded,
}
