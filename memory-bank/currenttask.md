# Current Task: Introduce Configuration System

## Objective

Introduce a configuration system to replace all hardcoded values in the codebase using environment variables with `.env` file support.

**Approach:**
- Use `dotenvy` to load `.env` files into environment variables
- Use `envy` with `serde` to deserialize environment variables into typed config structs
- Fail fast: no default values in code, validate on startup (empty string checks, etc.)
- Shell environment variables override `.env` file values

**Config structure:**
- `event_ray_core`:
  - `init()` function to load `.env`
  - `RedisConfig` struct (shared by both services when `redis-pubsub` enabled)
- `event_ray_server`: `ServerConfig` struct
- `ingestion_service`: `IngestionConfig` struct

**Configurable values:**
- Server host/port for both services
- Broadcast channel capacity
- Redis URL and channel name
- Event Ray target URL (for HTTP mode inter-service communication)

## Implementation Steps

### Step 1: Add dependencies

Add `dotenvy` and `envy` crates using `cargo add`:

```bash
cargo add dotenvy envy -p event_ray_core
cargo add envy -p event_ray_server
cargo add envy -p ingestion_service
```

Note: All crates already have `serde` with derive feature.

### Step 2: Create config module in `event_ray_core`

1. Add `redis-pubsub` feature to `event_ray_core/Cargo.toml` (needed for feature-gated `RedisConfig`)

2. Update feature definitions in both `event_ray_server/Cargo.toml` and `ingestion_service/Cargo.toml` to propagate the feature to `event_ray_core`:

```toml
[features]
redis-pubsub = ["dep:redis", "event_ray_core/redis-pubsub"]
```

3. Create `event_ray_core/src/config.rs`:

```rust
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
```

4. Export the module in `event_ray_core/src/lib.rs`:

```rust
pub mod config;
```

### Step 3: Create config module in `ingestion_service`

1. Create `ingestion_service/src/config.rs`:

```rust
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
```

2. Export the module in `ingestion_service/src/lib.rs`:

```rust
pub mod config;
```

3. Update `ingestion_service/src/main.rs`:
   - Call `event_ray_core::config::init()` at startup
   - Load `IngestionConfig::from_env()`
   - Replace hardcoded host/port with config values
   - Use `config.redis` for Redis publisher setup (when `redis-pubsub` enabled)

### Step 4: Create config module in `event_ray_server`

1. Create `event_ray_server/src/config.rs`:

```rust
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
```

2. Export the module in `event_ray_server/src/lib.rs`:

```rust
pub mod config;
```

3. Update `event_ray_server/src/main.rs`:
   - Call `event_ray_core::config::init()` at startup
   - Load `ServerConfig::from_env()`
   - Replace hardcoded host/port with config values
   - Use `config.broadcast_channel_capacity` for broadcast channel setup
   - Use `config.redis` for Redis subscriber setup (when `redis-pubsub` enabled)

### Step 5: Create `.env` files

1. Create `.env` for local development (uses `localhost` for safety):

```bash
# Event Ray Server
EVENT_RAY_SERVER_HOST=localhost
EVENT_RAY_SERVER_PORT=8081
BROADCAST_CHANNEL_CAPACITY=100

# Ingestion Service
INGESTION_SERVICE_HOST=localhost
INGESTION_SERVICE_PORT=8082

# HTTP mode: target URL for ingestion -> server communication
EVENT_RAY_TARGET_URL=http://localhost:8081/api/events

# Redis (used when redis-pubsub feature is enabled)
REDIS_URL=redis://127.0.0.1:6379
REDIS_CHANNEL=event_ray:events
```

2. Create `.env.docker` for Docker (uses `0.0.0.0` to accept external connections):

```bash
# Event Ray Server
EVENT_RAY_SERVER_HOST=0.0.0.0
EVENT_RAY_SERVER_PORT=8081
BROADCAST_CHANNEL_CAPACITY=100

# Ingestion Service
INGESTION_SERVICE_HOST=0.0.0.0
INGESTION_SERVICE_PORT=8082

# HTTP mode: target URL uses Docker service name for inter-container communication
EVENT_RAY_TARGET_URL=http://event_ray_server:8081/api/events

# Redis (used when redis-pubsub feature is enabled)
REDIS_URL=redis://redis:6379
REDIS_CHANNEL=event_ray:events
```

3. Both files committed to git (no secrets, safe defaults).

### Step 6: Update Docker Compose setup

1. Update Dockerfiles to support feature flags via build args.

   In `Dockerfile.event_ray_server`, change build command to:
   ```dockerfile
   ARG FEATURES=""
   RUN if [ -z "$FEATURES" ]; then \
         cargo build --release -p event_ray_server; \
       else \
         cargo build --release -p event_ray_server --features "$FEATURES"; \
       fi
   ```

   Same change in `Dockerfile.ingestion_service` for `ingestion_service`.

2. Create `docker-compose.yml` (base - HTTP mode):

```yaml
services:
  event_ray_server:
    build:
      context: .
      dockerfile: Dockerfile.event_ray_server
    env_file: .env.docker
    ports:
      - "8081:8081"
    networks:
      - event_ray_network

  ingestion_service:
    build:
      context: .
      dockerfile: Dockerfile.ingestion_service
    env_file: .env.docker
    ports:
      - "8082:8082"
    depends_on:
      - event_ray_server
    networks:
      - event_ray_network

networks:
  event_ray_network:
```

3. Create `docker-compose.redis.yml` (override - adds Redis mode):

```yaml
services:
  event_ray_server:
    build:
      args:
        FEATURES: redis-pubsub
    depends_on:
      - redis

  ingestion_service:
    build:
      args:
        FEATURES: redis-pubsub
    depends_on:
      - redis

  redis:
    image: redis:alpine
    ports:
      - "6379:6379"
    networks:
      - event_ray_network
```

4. Usage:

```bash
# HTTP mode
docker-compose up --build

# Redis mode (merges base + override)
docker-compose -f docker-compose.yml -f docker-compose.redis.yml up --build
```
