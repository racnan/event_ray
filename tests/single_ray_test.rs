mod common;

use event_ray::app_state::AppState;
use event_ray::event::AppEvent;
use std::time::Duration;
use tokio::sync::broadcast;
use uuid::Uuid;
use common::server_manager::TestServerHandle;
use common::sse_client::SseTestClient;
use common::publisher_client::PublisherTestClient;

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_environment() -> (TestServerHandle, PublisherTestClient, String) {
        let (event_sender, _) = broadcast::channel::<AppEvent>(1024);
        let app_state = AppState { event_sender };
        let server = TestServerHandle::new(app_state).await.expect("Failed to start server");
        let base_url = server.base_url.clone();
        let publisher_client = PublisherTestClient::new(base_url.clone());
        (server, publisher_client, base_url)
    }

    #[tokio::test]
    async fn test_publish_and_receive_single_event_on_ray() {
        let (server_handle, publisher, base_url) = setup_test_environment().await;
        let ray_id = Uuid::new_v4().to_string();
        let payload_str = "test_payload_1".to_string();

        // Connect SSE client first
        let mut sse_client = SseTestClient::connect(&base_url, &ray_id)
            .await
            .expect("Failed to connect SSE client");

        // Now publish the event
        println!("Publishing event to ray: {}", ray_id);
        publisher.publish_event(&ray_id, payload_str.clone())
            .await
            .inspect_err(|e| println!("{e}"))
            .expect("Failed to publish event");

        // Receive and verify the event
        match sse_client.receive_event(Duration::from_secs(5)).await {
            Ok(Some(received_payload)) => {
                println!("evnt returned");
                assert_eq!(received_payload, payload_str);
            }
            Ok(None) => panic!("Stream ended prematurely, expected an event"),
            Err(e) => panic!("Failed to receive event: {:?}", e),
        }

        println!("end");

        server_handle.stop().await.expect("Failed to stop server");
    }
}
