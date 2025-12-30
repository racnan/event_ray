use ingestion_service::{
    app_state::AppState, config::IngestionConfig, publisher::EventPublisher, routes::create_router,
};
#[cfg(feature = "redis-pubsub")]
use ingestion_service::publisher::redis::RedisPublisher;
#[cfg(not(feature = "redis-pubsub"))]
use ingestion_service::publisher::http::HttpPublisher;

use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file and config
    event_ray_core::config::init();
    let config = IngestionConfig::from_env();

    // Initialize publisher
    #[cfg(feature = "redis-pubsub")]
    let publisher: Arc<dyn EventPublisher> = Arc::new(RedisPublisher::new(
        config.redis.redis_url.clone(),
        config.redis.redis_channel.clone(),
    ));

    #[cfg(not(feature = "redis-pubsub"))]
    let publisher: Arc<dyn EventPublisher> = Arc::new(HttpPublisher::new(
        config.event_ray_target_url.clone(),
    ));

    let app_state = AppState { publisher };

    // Log which publisher mode is active
    #[cfg(feature = "redis-pubsub")]
    println!("Using Redis publisher (channel: {})", config.redis.redis_channel);

    #[cfg(not(feature = "redis-pubsub"))]
    println!("Using HTTP publisher (target: {})", config.event_ray_target_url);

    // Create router
    let app = create_router(app_state);

    // Setup listener
    let addr = format!("{}:{}", config.ingestion_service_host, config.ingestion_service_port);
    let listener = TcpListener::bind(&addr).await?;
    println!(
        "Ingestion service running on http://{}",
        listener.local_addr()?
    );

    // Start server
    axum::serve(listener, app).await?;
    Ok(())
}
