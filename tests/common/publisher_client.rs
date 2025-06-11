use event_ray::api_models::PublishRequest;

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
    pub async fn publish_event(&self, ray_id: &str, payload: String) -> Result<(), reqwest::Error> {
        let publish_url = format!("{}/api/events", self.base_url);
        
        let request_body = PublishRequest {
            ray_id: ray_id.to_string(),
            payload,
        };
        
        let response = self.http_client
            .post(&publish_url)
            .json(&request_body)
            .send()
            .await?;
        
        response.error_for_status()?;
        
        Ok(())
    }
}
