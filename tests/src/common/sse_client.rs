use futures::stream::StreamExt;
use std::time::Duration;

use eventsource_client::{BoxStream, Client, SSE};
/// SSE test client for subscribing to event streams
pub struct SseTestClient {
    /// The SSE client
    stream: BoxStream<eventsource_client::Result<SSE>>,
}

impl SseTestClient {
    /// Connect to an SSE endpoint for a specific ray
    pub async fn connect(base_url: &str, ray_id: &str) -> Result<Self, eventsource_client::Error> {
        let sse_url = format!("{}/sse?ray={}", base_url, ray_id);

        // Create the client
        let client = eventsource_client::ClientBuilder::for_url(&sse_url)?.build();

        // This makes an API call to route
        // It is required since we need to call the handler and estabilish connection.
        let mut stream = client.stream();
        let _ = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next()).await;

        Ok(Self { stream })
    }

    /// Receive the next event from the stream with a timeout
    /// Returns Ok(Some(event)) if an event was received
    /// Returns Ok(None) if the stream ended or timed out
    /// Returns Err if there was an error receiving the event
    pub async fn receive_event(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        loop {
            match tokio::time::timeout(timeout, self.stream.next()).await {
                Ok(Some(Ok(sse_event))) => {
                    match sse_event {
                        SSE::Event(event) => {
                            // Parse the event data as JSON into AppEvent
                            return Ok(Some(event.data));
                        }
                        SSE::Comment(_) | SSE::Connected(_) => {
                            // Ignore comments and connection events, continue loop
                            continue;
                        }
                    }
                }
                Ok(Some(Err(e))) => return Err(Box::new(e)),
                Ok(None) => return Ok(None), // Stream ended
                Err(_) => return Ok(None),   // Timeout
            }
        }
    }
}
