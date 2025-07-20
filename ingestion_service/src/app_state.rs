use crate::publisher::EventPublisher;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub publisher: Arc<dyn EventPublisher>,
}
