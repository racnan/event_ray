# Current Task: Implement Scalable Event Propagation with Redis Pub/Sub

## Objective

Implement a scalable event propagation mechanism using Redis Pub/Sub to decouple the `ingestion_service` from the `event_ray_server`. This will allow multiple instances of each service to communicate effectively. The entire implementation will be controlled by a `redis-pubsub` feature flag.

## Implementation Steps

### Step 1: Add Dependencies

**Objective:** Add the `redis` crate as an optional dependency to the relevant services, controlled by a `redis-pubsub` feature flag. This will be done using the `cargo add` command to ensure the latest versions are used. Note: `serde_json` is already a regular dependency in both services, so it does not need to be added.

**Execution Plan:**

1.  **Update `ingestion_service/Cargo.toml`**:
    *   Run the following command to add `redis` as an optional dependency with the `tokio-comp` feature:
        ```sh
        cargo add redis --optional --features tokio-comp -p ingestion_service
        ```
    *   Manually add the `[features]` section to define the `redis-pubsub` feature, which will enable the `redis` dependency:
        ```toml
        [features]
        redis-pubsub = ["dep:redis"]
        ```

2.  **Update `event_ray_server/Cargo.toml`**:
    *   Run the following command to add `redis` as an optional dependency with the `tokio-comp` feature:
        ```sh
        cargo add redis --optional --features tokio-comp -p event_ray_server
        ```
    *   Manually add the `[features]` section to define the `redis-pubsub` feature, which will enable the `redis` dependency:
        ```toml
        [features]
        redis-pubsub = ["dep:redis"]
        ```

### Step 2: Create `RedisPublisher`

**Objective:** Create a new publisher in `ingestion_service` that implements the `EventPublisher` trait and sends events to a Redis channel.

**Execution Plan:**

1.  **Create New File:** Create `ingestion_service/src/publisher/redis.rs`.
2.  **Conditional Compilation:** Guard the entire module with `#[cfg(feature = "redis-pubsub")]` to ensure it is only compiled when the feature is enabled.
3.  **Define `RedisPublisher` Struct:** The struct will hold a `redis::Client` for connecting to Redis and a `String` for the channel name.
4.  **Implement `new` Function:** The constructor will take a Redis URL, initialize the `redis::Client`, and return a `RedisPublisher` instance.
5.  **Implement `EventPublisher` Trait:** Implement the `async fn publish(&self, event: &AppEvent)` method. This method will:
    a.  Get a connection from the Redis client.
    b.  Serialize the `AppEvent` to a JSON string using `serde_json`.
    c.  Use the Redis `PUBLISH` command to send the JSON string to the configured channel.
    d.  Handle and report any errors using `error-stack`.
6.  **Expose Module:** Add `#[cfg(feature = "redis-pubsub")] pub mod redis;` to `ingestion_service/src/publisher.rs` to make the new module available.

### Step 3: Create Redis Subscriber

**Objective:** Create a background task in `event_ray_server` that subscribes to a Redis channel and forwards received events to the internal Tokio broadcast channel.

**Execution Plan:**

1.  **Update Error Types:** Add Redis-related error variants to `event_ray_server/src/error.rs`, guarded by `#[cfg(feature = "redis-pubsub")]`:
    *   `RedisConnection` - for connection/subscription failures (wraps `redis::RedisError`)
    *   `Deserialization` - for JSON parsing failures (wraps `serde_json::Error`)

2.  **Create New File:** Create `event_ray_server/src/redis_subscriber.rs`.

3.  **Conditional Compilation:** Guard the entire module with `#[cfg(feature = "redis-pubsub")]`.

4.  **Implement Subscriber Function:** Create an async function `run_redis_subscriber` with the following signature:
    ```rust
    pub async fn run_redis_subscriber(
        redis_url: &str,
        channel: &str,
        event_sender: broadcast::Sender<AppEvent>,
    ) -> Result<(), Report<Error>>
    ```
    The function will:
    a.  Connect to Redis using `redis::Client`.
    b.  Get a Pub/Sub connection and subscribe to the specified channel.
    c.  Loop to receive messages from the Redis subscription.
    d.  Deserialize each message from JSON into `AppEvent`.
    e.  Send the `AppEvent` to the broadcast channel.

5.  **Error Handling Strategy:**
    *   **Connection errors:** Return `Err` and exit the function (fatal).
    *   **Deserialization errors:** Log the error with `eprintln!` and continue listening (non-fatal).
    *   **Broadcast send errors:** Log the error and continue (non-fatal, indicates no subscribers).

6.  **Expose Module:** Add `#[cfg(feature = "redis-pubsub")] pub mod redis_subscriber;` to `event_ray_server/src/lib.rs`.

### Step 4: Update `ingestion_service` Initialization

**Objective:** Conditionally initialize `RedisPublisher` or `HttpPublisher` in `ingestion_service/src/main.rs` based on the `redis-pubsub` feature flag.

**Execution Plan:**

1.  **Add Conditional Imports:** Update imports in `main.rs` to be feature-gated:
    ```rust
    #[cfg(feature = "redis-pubsub")]
    use ingestion_service::publisher::redis::RedisPublisher;

    #[cfg(not(feature = "redis-pubsub"))]
    use ingestion_service::publisher::http::HttpPublisher;
    ```

2.  **Conditional Publisher Creation:** Replace the current publisher initialization with feature-gated code:
    ```rust
    #[cfg(feature = "redis-pubsub")]
    let publisher: Arc<dyn EventPublisher> = Arc::new(RedisPublisher::new(
        "redis://127.0.0.1:6379".to_string(),
        "event_ray:events".to_string(),
    ));

    #[cfg(not(feature = "redis-pubsub"))]
    let publisher: Arc<dyn EventPublisher> = Arc::new(HttpPublisher::new(
        "http://localhost:8081/api/events".to_string(),
    ));
    ```

3.  **Add Startup Log:** Print which publisher mode is active for visibility:
    ```rust
    #[cfg(feature = "redis-pubsub")]
    println!("Using Redis publisher (channel: event_ray:events)");

    #[cfg(not(feature = "redis-pubsub"))]
    println!("Using HTTP publisher");
    ```

4.  **Add Required Import:** Add `use crate::publisher::EventPublisher;` to bring the trait into scope for the `Arc<dyn EventPublisher>` type annotation.

### Step 5: Update `event_ray_server` Initialization

**Objective:** Conditionally spawn the Redis subscriber task in `event_ray_server/src/main.rs` when the `redis-pubsub` feature is enabled. The server should verify Redis connectivity before starting.

**Execution Plan:**

1.  **Add Conditional Import:**
    ```rust
    #[cfg(feature = "redis-pubsub")]
    use event_ray_server::redis_subscriber;
    ```

2.  **Verify Redis Connection Before Starting:** Before spawning the subscriber, test the Redis connection. If it fails, the server should exit with an error rather than starting in a broken state:
    ```rust
    #[cfg(feature = "redis-pubsub")]
    {
        // Verify Redis connectivity before starting
        let client = redis::Client::open("redis://127.0.0.1:6379")?;
        let mut conn = client.get_multiplexed_async_connection().await?;
        redis::cmd("PING").query_async::<String>(&mut conn).await?;
        println!("Connected to Redis successfully");
    }
    ```

3.  **Spawn Redis Subscriber:** After verifying connectivity, spawn the subscriber task:
    ```rust
    #[cfg(feature = "redis-pubsub")]
    {
        let sender_clone = event_sender.clone();
        tokio::spawn(async move {
            if let Err(e) = redis_subscriber::run_redis_subscriber(
                "redis://127.0.0.1:6379",
                "event_ray:events",
                sender_clone,
            ).await {
                eprintln!("Redis subscriber error: {:?}", e);
            }
        });
        println!("Redis subscriber started (channel: event_ray:events)");
    }
    ```

4.  **Add Startup Log for Non-Redis Mode:**
    ```rust
    #[cfg(not(feature = "redis-pubsub"))]
    println!("Running in HTTP mode (no Redis subscriber)");
    ```

5.  **Add Conditional Redis Import:** Add feature-gated import for the `redis` crate:
    ```rust
    #[cfg(feature = "redis-pubsub")]
    use redis::AsyncCommands;
    ```

### Step 6: Add Justfile

**Objective:** Add a `justfile` to the project root for convenient development commands, including running both services together and checking all feature combinations.

**Execution Plan:**

1.  **Create `justfile`** in the project root with the following content:

    ```just
    # Default recipe - show available commands
    default:
        @just --list

    # Run both services in HTTP mode (default)
    run-http:
        #!/usr/bin/env bash
        set -e
        echo "Starting services in HTTP mode..."
        cargo run -p event_ray_server &
        SERVER_PID=$!
        cargo run -p ingestion_service &
        INGESTION_PID=$!
        trap "kill $SERVER_PID $INGESTION_PID 2>/dev/null" EXIT
        wait

    # Run both services with Redis Pub/Sub
    run-redis:
        #!/usr/bin/env bash
        set -e
        echo "Starting services with Redis Pub/Sub..."
        echo "Make sure Redis is running on localhost:6379"
        cargo run -p event_ray_server --features redis-pubsub &
        SERVER_PID=$!
        cargo run -p ingestion_service --features redis-pubsub &
        INGESTION_PID=$!
        trap "kill $SERVER_PID $INGESTION_PID 2>/dev/null" EXIT
        wait

    # Run tests
    test:
        cargo test --workspace

    # Run clippy for all feature combinations
    lint:
        cargo hack --feature-powerset clippy --workspace -- -D warnings

    # Check compilation for all feature combinations
    check:
        cargo hack --feature-powerset check --workspace
    ```

2.  **Prerequisites:** Developers will need to install `just` and `cargo-hack`:
    ```sh
    cargo install just
    cargo install cargo-hack
    ```

### Step 7: Update Documentation

**Objective:** Update project documentation to reflect the new Redis Pub/Sub feature.

**Execution Plan:**

1.  **Review and update the following documents as needed:**
    *   `README.md` - Document the Redis feature, justfile commands, and prerequisites
    *   `memory-bank/architecture.md` - Update event flow to reflect the Redis Pub/Sub path
    *   `memory-bank/project_structure.md` - Document new files and feature flags

2.  **Add entry to `memory-bank/previoustasks.md`** summarizing the completed task (per SOP).