use thiserror::Error;

#[derive(Debug, Error)]
pub enum TestUtilError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Eventsource client error: {0}")]
    EventSource(#[from] eventsource_client::Error),
}
