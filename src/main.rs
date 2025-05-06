use async_stream::stream;
use axum::{
    http::StatusCode,
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::Stream;

// Type aliases for better readability
type EventSender = Arc<mpsc::Sender<()>>;
type EventReceiver = Arc<Mutex<mpsc::Receiver<()>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize event channel
    let (tx, rx) = mpsc::channel(10);
    let tx = Arc::new(tx);
    let rx = Arc::new(Mutex::new(rx));
    
    // Setup routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/sse", get({
            let rx = rx.clone();
            move || sse_handler(rx)
        }))
        .route("/push", get({
            let tx = tx.clone();
            move || push_handler(tx)
        }));

    // Setup listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
    println!("Server running on http://0.0.0.0:8081"); // Log the actual listening address

    // Start server
    axum::serve(listener, app).await?;
    Ok(())
}

/// Health check endpoint handler
async fn health_check() -> &'static str {
    "Event Ray is Up"
}

/// Server-Sent Events handler
/// Creates a stream of events that clients can subscribe to
async fn sse_handler(
    rx: EventReceiver,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream! {
        loop {
            let mut rx = rx.lock().await;
            if let Some(_) = rx.recv().await {
                yield Ok(Event::default().data("Hello"));
            }
        }
    };

    Sse::new(stream)
}

/// Push event handler
/// Sends a new event to all connected SSE clients
async fn push_handler(tx: EventSender) -> StatusCode {
    match tx.send(()).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
