use tokio::net::TcpListener;
use tokio::sync::broadcast;

use event_ray_core::app_event::AppEvent;
use event_ray_server::{app_state::AppState, config::ServerConfig, routes};
use event_ray_server::{app_state::AppState, routes};

#[cfg(feature = "redis-pubsub")]
use event_ray_server::redis_subscriber;

/// The main entry point for the Event Ray application.
/// Initializes the Tokio runtime, sets up the event broadcast channel,
/// application state, router, and starts the HTTP server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file and config
    event_ray_core::config::init();
    let config = ServerConfig::from_env();

    // Initialize broadcast channel
    let (event_sender, _) = broadcast::channel::<AppEvent>(config.broadcast_channel_capacity);

    // Verify Redis connection and spawn subscriber (if feature enabled)
    #[cfg(feature = "redis-pubsub")]
    {
        // Verify Redis connectivity before starting
        let client = redis::Client::open(config.redis.redis_url.as_str())?;
        let mut conn = client.get_multiplexed_async_connection().await?;
        redis::cmd("PING").query_async::<String>(&mut conn).await?;
        println!("Connected to Redis successfully");

        // Spawn Redis subscriber task
        let redis_url = config.redis.redis_url.clone();
        let redis_channel = config.redis.redis_channel.clone();
        let sender_clone = event_sender.clone();
        tokio::spawn(async move {
            if let Err(e) =
                redis_subscriber::run_redis_subscriber(&redis_url, &redis_channel, sender_clone)
                    .await
            {
                eprintln!("Redis subscriber error: {:?}", e);
            }
        });
        println!(
            "Redis subscriber started (channel: {})",
            config.redis.redis_channel
        );
    }

    #[cfg(not(feature = "redis-pubsub"))]
    println!("Running in HTTP mode (no Redis subscriber)");

    // Construct AppState
    let app_state = AppState { event_sender };

    // Create router
    let app = routes::create_router(app_state);

    // Setup listener
    let addr = format!(
        "{}:{}",
        config.event_ray_server_host, config.event_ray_server_port
    );
    let listener = TcpListener::bind(&addr).await?;
    println!("Server running on http://{}", listener.local_addr()?);

    // Start server
    axum::serve(listener, app).await?;
    Ok(())
}
