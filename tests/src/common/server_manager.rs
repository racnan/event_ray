use tokio::net::TcpListener;
use tokio::sync::oneshot;

use event_ray_server::app_state::AppState;

/// Handle for managing a test instance of the Event Ray server
pub struct TestServerHandle {
    /// Channel to signal server shutdown
    shutdown_tx: oneshot::Sender<()>,
    /// Base URL for connecting to the server (e.g., "http://127.0.0.1:PORT")
    pub base_url: String,
}

impl TestServerHandle {
    /// Create and start a new test server instance
    pub async fn new(app_state: AppState) -> Result<Self, Box<dyn std::error::Error>> {
        // Bind to a random available port
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let base_url = format!("http://{}", addr);

        println!("Starting server at {base_url}");

        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        // Get the router from the application
        let router = event_ray_server::routes::create_router(app_state);

        // Spawn the server task
        tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async {
                    shutdown_rx.await.ok();
                })
                .await
                .map_err(std::io::Error::other)
        });

        // Wait for server to be ready by checking health endpoint
        let client = reqwest::Client::new();
        let health_url = format!("{}/health", base_url);

        for _ in 0..50 {
            // Try for up to 5 seconds (50 * 100ms)
            if client.get(&health_url).send().await.is_ok() {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(Self {
            shutdown_tx,
            base_url,
        })
    }

    /// Stop the test server
    pub async fn stop(self) -> Result<(), Box<dyn std::error::Error>> {
        // Send shutdown signal
        let _ = self.shutdown_tx.send(());

        Ok(())
    }
}
