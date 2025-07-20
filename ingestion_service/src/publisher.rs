pub mod http;

use async_trait::async_trait;
use event_ray_core::app_event::AppEvent;
use std::error::Error;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: &AppEvent) -> Result<(), Box<dyn Error>>;
}
