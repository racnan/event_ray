use tokio::net::TcpListener;

mod handlers;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create router
    let app = routes::create_router();

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
