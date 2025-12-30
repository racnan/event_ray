use serde::Deserialize;

#[cfg(feature = "redis-pubsub")]
use event_ray_core::config::RedisConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct IngestionConfig {
    pub ingestion_service_host: String,
    pub ingestion_service_port: u16,
    #[cfg(not(feature = "redis-pubsub"))]
    pub event_ray_target_url: String,
    #[cfg(feature = "redis-pubsub")]
    #[serde(flatten)]
    pub redis: RedisConfig,
}

impl IngestionConfig {
    pub fn from_env() -> Self {
        let config: Self = envy::from_env()
            .expect("Failed to load IngestionConfig from environment");
        config.validate();
        config
    }

    fn validate(&self) {
        assert!(
            !self.ingestion_service_host.is_empty(),
            "INGESTION_SERVICE_HOST cannot be empty"
        );
        #[cfg(not(feature = "redis-pubsub"))]
        assert!(
            !self.event_ray_target_url.is_empty(),
            "EVENT_RAY_TARGET_URL cannot be empty"
        );
        #[cfg(feature = "redis-pubsub")]
        self.redis.validate();
    }
}
