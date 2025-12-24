# Event Ray

Event Ray is a high-performance, open-source event streaming server built with Rust. It provides a scalable and reliable infrastructure for Server-Sent Events (SSE), enabling backend services to push real-time updates to clients over HTTP.

## Project Details

For more detailed information on the project's design and architecture, please refer to the following documents:

*   [**Project Brief**](memory-bank/projectbrief.md): An overview of the project's goals, positioning, and technologies used.
*   [**Architecture**](memory-bank/architecture.md): A high-level overview of the event flow and system design.
*   [**Project Structure**](memory-bank/project_structure.md): A detailed breakdown of the workspace, crates, and modules.

## Getting Started

### Prerequisites

To build and run the project, you will need to have the following installed:

*   [Rust](https://www.rust-lang.org/tools/install)
*   [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)
*   [Just](https://github.com/casey/just) (optional, for convenient development commands)
*   [cargo-hack](https://github.com/taiki-e/cargo-hack) (optional, for testing all feature combinations)
*   [Redis](https://redis.io/docs/getting-started/installation/) (required only for Redis Pub/Sub mode)

## How to Run

1.  **Build the workspace:**

    ```bash
    cargo build --workspace
    ```

2.  **Run the `event_ray_server`:**

    This server handles SSE subscriptions and is responsible for streaming events to clients.

    ```bash
    cargo run --bin event_ray_server
    ```

    The server will start on port `8081`.

3.  **Run the `ingestion_service`:**

    This service provides an endpoint for ingesting events into the system. Open a new terminal window and run:

    ```bash
    cargo run --bin ingestion_service
    ```

    The ingestion service will start on port `8082`.

4.  **Subscribe to an SSE stream:**

    You can use `curl` to subscribe to an event stream for a specific "ray".

    *   **Subscribe to "ray_1":**

        ```bash
        curl -N http://localhost:8081/sse?ray=ray_1
        ```

    *   **Subscribe to "ray_2":**

        Open another terminal and run:

        ```bash
        curl -N http://localhost:8081/sse?ray=ray_2
        ```

5.  **Publish an event:**

    You can now publish events to the subscribed rays using the `ingestion_service`.

    *   **Publish to "ray_1":**

        ```bash
        curl -X POST http://localhost:8082/api/events -H "Content-Type: application/json" -d '''{
          "ray_id": "ray_1",
          "payload": "Hello from ray 1!"
        }'''
        ```

        You should see the event appear in the terminal subscribed to "ray_1".

    *   **Publish to "ray_2":**

        ```bash
        curl -X POST http://localhost:8082/api/events -H "Content-Type: application/json" -d '''{
          "ray_id": "ray_2",
          "payload": "Greetings from ray 2!"
        }'''
        ```

        You should see this event appear in the terminal subscribed to "ray_2".

## Testing

To run the integration test suite, use the following command:

```bash
cargo test --workspace
```

**Note:** The test suite manages its own server instances. Please ensure that any manually started servers are shut down before running the tests to avoid port conflicts.

## Work in Progress

This project is under active development. The APIs and architecture are subject to change.
