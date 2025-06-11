use tokio::net::TcpListener;
use tokio::sync::broadcast;

use event_ray::{app_state::AppState, event::AppEvent, routes};

/// The main entry point for the Event Ray application.
/// Initializes the Tokio runtime, sets up the event broadcast channel,
/// application state, router, and starts the HTTP server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize broadcast channel
    let (event_sender, _) = broadcast::channel::<AppEvent>(100);

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
