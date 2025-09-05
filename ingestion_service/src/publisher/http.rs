use super::EventPublisher;
use crate::error::Error;
use async_trait::async_trait;
use error_stack::Report;
use event_ray_core::{api_models::PublishRequest, app_event::AppEvent};
use reqwest::Client;

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
    async fn publish(&self, event: &AppEvent) -> Result<(), Report<Error>> {
        let request_body = PublishRequest {
            ray_id: event.ray_id.clone(),
            payload: event.payload.clone(),
        };

        let res = self
            .client
            .post(&self.target_url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Report::new(e).change_context(Error::PublishFailed))?;

        if res.status().is_success() {
            Ok(())
        } else {
            let report = match res.error_for_status() {
                Err(e) => Report::new(e).change_context(Error::PublishFailed),
                Ok(_) => {
                    // This case should be logically impossible.
                    // is_success() was false, so error_for_status() should always be an Err.
                    Report::new(Error::PublishFailed).attach(
                        "Logic error: is_success() was false, but error_for_status() returned Ok",
                    )
                }
            };
            Err(report)
        }
    }
}
