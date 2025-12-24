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
