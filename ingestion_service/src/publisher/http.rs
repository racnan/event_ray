use super::EventPublisher;
use async_trait::async_trait;
use event_ray_core::{api_models::PublishRequest, app_event::AppEvent};
use reqwest::Client;
use std::error::Error;

pub struct HttpPublisher {
    client: Client,
    target_url: String,
}

impl HttpPublisher {
    pub fn new(target_url: String) -> Self {
        Self {
            client: Client::new(),
            target_url,
        }
    }
}

#[async_trait]
impl EventPublisher for HttpPublisher {
    async fn publish(&self, event: &AppEvent) -> Result<(), Box<dyn Error>> {
        let request_body = PublishRequest {
            ray_id: event.ray_id.clone(),
            payload: event.payload.clone(),
        };

        let res = self
            .client
            .post(&self.target_url)
            .json(&request_body)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to publish event: {}", res.status()).into())
        }
    }
}
