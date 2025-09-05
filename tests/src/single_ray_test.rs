use crate::common::{
    publisher_client::PublisherTestClient,
    server_manager::{ServerConfig, TestServerHandle},
    sse_client::SseTestClient,
};
use reqwest::StatusCode;
use std::time::Duration;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    /// Sets up the full test environment with both the Event Ray server and the Ingestion service.
    async fn setup_test_environment() -> (
        TestServerHandle, // Ingestion service handle
        TestServerHandle, // Event Ray server handle
        PublisherTestClient,
    ) {
        // Start the main Event Ray server first.
        let event_ray_server = TestServerHandle::new(ServerConfig::EventRay)
            .await
            .expect("Failed to start Event Ray server");

        // Start the Ingestion service, configured to point to the Event Ray server.
        let ingestion_server = TestServerHandle::new(ServerConfig::Ingestion {
            event_ray_server_url: event_ray_server.base_url.clone(),
        })
        .await
        .expect("Failed to start Ingestion server");

        // The publisher client now talks to the Ingestion service.
        let publisher_client = PublisherTestClient::new(ingestion_server.base_url.clone());

        (ingestion_server, event_ray_server, publisher_client)
    }

    #[tokio::test]
    async fn test_publish_and_receive_single_event_on_ray() {
        let (ingestion_server, event_ray_server, publisher) = setup_test_environment().await;
        let ray_id = Uuid::new_v4().to_string();
        let payload_str = "test_payload_1".to_string();

        // Connect SSE client to the main Event Ray server.
        let mut sse_client = SseTestClient::connect(&event_ray_server.base_url, &ray_id)
            .await
            .expect("Failed to connect SSE client");

        // Publish the event to the Ingestion service.
        println!("Publishing event via Ingestion service to ray: {}", ray_id);
        publisher
            .publish_event(&ray_id, payload_str.clone())
            .await
            .inspect_err(|e| println!("{e}"))
            .expect("Failed to publish event");

        // Receive and verify the event from the Event Ray server.
        match sse_client.receive_event(Duration::from_secs(5)).await {
            Ok(Some(received_payload)) => {
                assert_eq!(received_payload, payload_str);
            }
            Ok(None) => panic!("Stream ended prematurely, expected an event"),
            Err(e) => panic!("Failed to receive event: {:?}", e),
        }

        // Stop both servers.
        ingestion_server
            .stop()
            .await
            .expect("Failed to stop Ingestion server");
        event_ray_server
            .stop()
            .await
            .expect("Failed to stop Event Ray server");
    }

    #[tokio::test]
    async fn test_publish_invalid_schema_returns_400() {
        let (ingestion_server, event_ray_server, publisher) = setup_test_environment().await;

        // This JSON is malformed (missing a closing brace)
        let malformed_body = r#"{"ray_id": "some-ray", "payload": "some_payload""#.to_string();

        let response = publisher
            .publish_raw_body(malformed_body)
            .await
            .expect("Request failed");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // Stop both servers.
        ingestion_server
            .stop()
            .await
            .expect("Failed to stop Ingestion server");
        event_ray_server
            .stop()
            .await
            .expect("Failed to stop Event Ray server");
    }
}
