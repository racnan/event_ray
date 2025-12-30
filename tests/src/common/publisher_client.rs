use crate::common::error::TestUtilError;
use event_ray_core::api_models::PublishRequest;
use reqwest::Response;

/// Publisher test client for sending events to the Event Ray server
pub struct PublisherTestClient {
    /// HTTP client for making requests
    http_client: reqwest::Client,
    /// Base URL of the server
    base_url: String,
}

impl PublisherTestClient {
    /// Create a new publisher client
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url,
        }
    }

    /// Publish an event to a specific ray
    pub async fn publish_event(&self, ray_id: &str, payload: String) -> Result<(), TestUtilError> {
        let request_body = PublishRequest {
            ray_id: ray_id.to_string(),
            payload,
        };

        let response = self
            .http_client
            .post(self.events_url())
            .json(&request_body)
            .send()
            .await?;

        response.error_for_status()?;

        Ok(())
    }

    /// Publish a raw string body to the events endpoint.
    pub async fn publish_raw_body(&self, body: String) -> Result<Response, TestUtilError> {
        let response = self
            .http_client
            .post(self.events_url())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        Ok(response)
    }

    /// Returns the full URL for the events API endpoint.
    fn events_url(&self) -> String {
        format!("{}/api/events", self.base_url)
    }
}
