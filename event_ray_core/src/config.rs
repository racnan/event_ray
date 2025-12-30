/// Load .env file into environment variables.
/// Call once at startup before loading any config.
/// Uses .ok() because .env file is optional - production may use real env vars.
pub fn init() {
    dotenvy::dotenv().ok();
}

/// Redis configuration - shared by both services when redis-pubsub enabled
#[cfg(feature = "redis-pubsub")]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct RedisConfig {
    pub redis_url: String,
    pub redis_channel: String,
}

#[cfg(feature = "redis-pubsub")]
impl RedisConfig {
    pub fn validate(&self) {
        assert!(!self.redis_url.is_empty(), "REDIS_URL cannot be empty");
        assert!(!self.redis_channel.is_empty(), "REDIS_CHANNEL cannot be empty");
    }
}
