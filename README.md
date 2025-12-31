# Event Ray

Event Ray is a high-performance, open-source event streaming server built with Rust. It provides a scalable and reliable infrastructure for Server-Sent Events (SSE), enabling backend services to push real-time updates to clients over HTTP.

## Project Details

For more detailed information on the project's design and architecture, please refer to the following documents:

*   [**Project Brief**](memory-bank/projectbrief.md): An overview of the project's goals, positioning, and technologies used.
*   [**Architecture**](memory-bank/architecture.md): A high-level overview of the event flow and system design.
*   [**Project Structure**](memory-bank/project_structure.md): A detailed breakdown of the workspace, crates, and modules.

## Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) (with Cargo)
*   [Just](https://github.com/casey/just) (for development commands)
*   [cargo-hack](https://github.com/taiki-e/cargo-hack) (for testing all feature combinations)
*   [Docker](https://docs.docker.com/get-docker/) (for containerized deployment)
*   [Redis](https://redis.io/docs/getting-started/installation/) (only for Redis Pub/Sub mode)

## How to Run

### Local Development

**HTTP Mode (default):**
```bash
just run-http
```

**Redis Pub/Sub Mode:**
```bash
# Ensure Redis is running on localhost:6379
just run-redis
```

### Docker Compose

**HTTP Mode:**
```bash
docker-compose up --build
```

**Redis Pub/Sub Mode:**
```bash
docker-compose -f docker-compose.yml -f docker-compose.redis.yml up --build
```

## Usage

Once the services are running:

**Subscribe to an SSE stream:**

*   **Subscribe to "ray_1":**

    ```bash
    curl -N http://localhost:8081/sse?ray=ray_1
    ```

*   **Subscribe to "ray_2":**

    Open another terminal and run:

    ```bash
    curl -N http://localhost:8081/sse?ray=ray_2
    ```

**Publish an event:**

*   **Publish to "ray_1":**

    ```bash
    curl -X POST http://localhost:8082/api/events -H "Content-Type: application/json" -d '{
      "ray_id": "ray_1",
      "payload": "Hello from ray 1!"
    }'
    ```

    You should see the event appear in the terminal subscribed to "ray_1".

*   **Publish to "ray_2":**

    ```bash
    curl -X POST http://localhost:8082/api/events -H "Content-Type: application/json" -d '{
      "ray_id": "ray_2",
      "payload": "Greetings from ray 2!"
    }'
    ```

    You should see this event appear in the terminal subscribed to "ray_2".

## Testing

```bash
just test    # Run all tests
just check   # Check compilation for all feature combinations
just lint    # Run clippy for all feature combinations
```

## Work in Progress

This project is under active development. The APIs and architecture are subject to change.
