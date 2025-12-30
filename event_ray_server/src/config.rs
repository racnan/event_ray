use serde::Deserialize;

#[cfg(feature = "redis-pubsub")]
use event_ray_core::config::RedisConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub event_ray_server_host: String,
    pub event_ray_server_port: u16,
    pub broadcast_channel_capacity: usize,
    #[cfg(feature = "redis-pubsub")]
    #[serde(flatten)]
    pub redis: RedisConfig,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let config: Self = envy::from_env()
            .expect("Failed to load ServerConfig from environment");
        config.validate();
        config
    }

    fn validate(&self) {
        assert!(
            !self.event_ray_server_host.is_empty(),
            "EVENT_RAY_SERVER_HOST cannot be empty"
        );
        assert!(
            self.broadcast_channel_capacity > 0,
            "BROADCAST_CHANNEL_CAPACITY must be greater than 0"
        );
        #[cfg(feature = "redis-pubsub")]
        self.redis.validate();
    }
}
