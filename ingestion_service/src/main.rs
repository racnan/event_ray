use ingestion_service::{
    app_state::AppState, publisher::http::HttpPublisher, routes::create_router,
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize publisher
    let publisher = HttpPublisher::new("http://localhost:8081/api/events".to_string());
    let app_state = AppState {
        publisher: Arc::new(publisher),
    };

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
