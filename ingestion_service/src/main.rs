#[cfg(not(feature = "redis-pubsub"))]
use ingestion_service::publisher::http::HttpPublisher;
#[cfg(feature = "redis-pubsub")]
use ingestion_service::publisher::redis::RedisPublisher;
use ingestion_service::{app_state::AppState, publisher::EventPublisher, routes::create_router};

use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize publisher
    #[cfg(feature = "redis-pubsub")]
    let publisher: Arc<dyn EventPublisher> = Arc::new(RedisPublisher::new(
        "redis://127.0.0.1:6379".to_string(),
        "event_ray:events".to_string(),
    ));

    #[cfg(not(feature = "redis-pubsub"))]
    let publisher: Arc<dyn EventPublisher> = Arc::new(HttpPublisher::new(
        "http://localhost:8081/api/events".to_string(),
    ));

    let app_state = AppState { publisher };

    // Log which publisher mode is active
    #[cfg(feature = "redis-pubsub")]
    println!("Using Redis publisher (channel: event_ray:events)");

    #[cfg(not(feature = "redis-pubsub"))]
    println!("Using HTTP publisher");

    // Create router
    let app = create_router(app_state);

    // Setup listener
    let listener = TcpListener::bind("0.0.0.0:8082").await?;
    println!(
        "Ingestion service running on http://{}",
        listener.local_addr()?
    );

    // Start server
    axum::serve(listener, app).await?;
    Ok(())
}
