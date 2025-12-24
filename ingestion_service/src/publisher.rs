pub mod http;
#[cfg(feature = "redis-pubsub")]
pub mod redis;

use crate::error::Error;
use async_trait::async_trait;
use error_stack::Report;
use event_ray_core::app_event::AppEvent;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: &AppEvent) -> Result<(), Report<Error>>;
}
