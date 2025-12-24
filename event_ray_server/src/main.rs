use tokio::net::TcpListener;
use tokio::sync::broadcast;

use event_ray_server::{app_state::AppState, routes};
use event_ray_core::app_event::AppEvent;

#[cfg(feature = "redis-pubsub")]
use event_ray_server::redis_subscriber;

/// The main entry point for the Event Ray application.
/// Initializes the Tokio runtime, sets up the event broadcast channel,
/// application state, router, and starts the HTTP server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize broadcast channel
    let (event_sender, _) = broadcast::channel::<AppEvent>(100);

    // Verify Redis connection and spawn subscriber (if feature enabled)
    #[cfg(feature = "redis-pubsub")]
    {
        // Verify Redis connectivity before starting
        let client = redis::Client::open("redis://127.0.0.1:6379")?;
        let mut conn = client.get_multiplexed_async_connection().await?;
        redis::cmd("PING").query_async::<String>(&mut conn).await?;
        println!("Connected to Redis successfully");

        // Spawn Redis subscriber task
        let sender_clone = event_sender.clone();
        tokio::spawn(async move {
            if let Err(e) = redis_subscriber::run_redis_subscriber(
                "redis://127.0.0.1:6379",
                "event_ray:events",
                sender_clone,
            )
            .await
            {
                eprintln!("Redis subscriber error: {:?}", e);
            }
        });
        println!("Redis subscriber started (channel: event_ray:events)");
    }

    #[cfg(not(feature = "redis-pubsub"))]
    println!("Running in HTTP mode (no Redis subscriber)");

    // Construct AppState
    let app_state = AppState { event_sender };

    // Create router
    let app = routes::create_router(app_state);

    // Setup listener
    let listener = TcpListener::bind("0.0.0.0:8081").await?;
    println!("Server running on http://0.0.0.0:8081"); // Log the actual listening address

    // Start server
    axum::serve(listener, app).await?;
    Ok(())
}
