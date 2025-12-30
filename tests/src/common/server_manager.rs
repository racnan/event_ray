use crate::common::error::TestUtilError;
use event_ray_server::app_state::AppState as EventRayAppState;
use ingestion_service::{app_state::AppState as IngestionAppState, publisher::http::HttpPublisher};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

/// Defines the configuration for different server types that can be launched for testing.
pub enum ServerConfig {
    /// Configuration for the main Event Ray SSE server.
    EventRay,
    /// Configuration for the Ingestion Service, requiring the target URL of the Event Ray server.
    Ingestion { event_ray_server_url: String },
}

/// Handle for managing a test instance of a server.
pub struct TestServerHandle {
    /// Channel to signal server shutdown.
    shutdown_tx: oneshot::Sender<()>,
    /// Base URL for connecting to the server (e.g., "http://127.0.0.1:PORT").
    pub base_url: String,
}

impl TestServerHandle {
    /// Create and start a new test server instance based on the provided configuration.
    pub async fn new(config: ServerConfig) -> Result<Self, TestUtilError> {
        // Bind to a random available port.
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let base_url = format!("http://{}", addr);

        // Create shutdown channel.
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        // Configure and create the router based on the server type.
        let router = match config {
            ServerConfig::EventRay => {
                println!("Starting Event Ray server at {base_url}");
                let (event_sender, _) = tokio::sync::broadcast::channel(1024);
                let app_state = EventRayAppState { event_sender };
                event_ray_server::routes::create_router(app_state)
            }
            ServerConfig::Ingestion {
                event_ray_server_url,
            } => {
                println!("Starting Ingestion server at {base_url}");
                let publisher = HttpPublisher::new(format!("{}/api/events", event_ray_server_url));
                let app_state = IngestionAppState {
                    publisher: Arc::new(publisher),
                };
                ingestion_service::routes::create_router(app_state)
            }
        };

        // Spawn the server task.
        tokio::spawn(async move {
            axum::serve(listener, router)
                .with_graceful_shutdown(async {
                    shutdown_rx.await.ok();
                })
                .await
                .map_err(std::io::Error::other)
        });

        // Wait for the server to be ready by checking its health endpoint.
        let client = reqwest::Client::new();
        let health_url = format!("{}/health", base_url);

        for _ in 0..50 {
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

    /// Stop the test server.
    pub async fn stop(self) -> Result<(), TestUtilError> {
        // Send shutdown signal
        let _ = self.shutdown_tx.send(());

        Ok(())
    }
}
