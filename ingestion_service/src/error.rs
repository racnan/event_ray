use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Publisher failed to send event")]
    PublishFailed,
}
