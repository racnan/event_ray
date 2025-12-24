use super::EventPublisher;
use crate::error::Error;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use event_ray_core::app_event::AppEvent;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};

pub struct RedisPublisher {
    client: Client,
    channel: String,
}

impl RedisPublisher {
    pub fn new(redis_url: String, channel: String) -> Self {
        let client = Client::open(redis_url).expect("Failed to create Redis client");
        Self { client, channel }
    }
}

#[async_trait]
impl EventPublisher for RedisPublisher {
    async fn publish(&self, event: &AppEvent) -> Result<(), Report<Error>> {
        // Get a connection from the Redis client
        let mut conn: MultiplexedConnection = self
            .client
            .get_multiplexed_async_connection()
            .await
            .change_context(Error::PublishFailed)
            .attach("Failed to get Redis connection")?;

        // Serialize the AppEvent to JSON
        let json_payload = serde_json::to_string(event)
            .change_context(Error::PublishFailed)
            .attach("Failed to serialize AppEvent to JSON")?;

        // Publish to Redis channel
        conn.publish::<_, _, ()>(&self.channel, json_payload)
            .await
            .change_context(Error::PublishFailed)
            .attach(format!("Failed to publish to Redis channel: {}", self.channel))?;

        Ok(())
    }
}
