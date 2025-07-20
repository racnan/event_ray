# Task: Abstract Event Publishing with an HTTP-Based Implementation

## Objective:
Create a generic `EventPublisher` trait to abstract the event propagation mechanism from the `ingestion_service`. Provide an initial, default implementation of this trait that uses an HTTP client to forward events to the `event_ray_server`'s existing `/api/events` endpoint.

This task will achieve:
*   A clean architectural boundary for event publishing.
*   A default, simple mechanism for inter-service communication suitable for single-node deployments.
*   The groundwork for introducing alternative backplanes (like Redis) in the future.

## Implementation Plan

**Step 1: Define the `EventPublisher` Trait and Module Structure.**
    *   Create a new `publisher.rs` file in `ingestion_service/src/`. In this file, define the `EventPublisher` trait.
    *   Create a corresponding `publisher/` directory in `ingestion_service/src/` to hold the implementations.
    *   Add `async-trait` as a dependency to `ingestion_service/Cargo.toml`.

**Step 2: Implement an HTTP-Based Publisher.**
    *   Create an `http.rs` file inside the `ingestion_service/src/publisher/` directory.
    *   In this file, define an `HttpPublisher` struct and implement the `EventPublisher` trait for it using `reqwest`.
    *   Add the `reqwest` crate with the `json` feature to `ingestion_service/Cargo.toml`.

**Step 3: Integrate the Publisher into the `ingestion_service` Application.**
    *   Create a shared `AppState` for the `ingestion_service` in a new `app_state.rs` file to hold the publisher.
    *   In `main.rs`, initialize the `HttpPublisher` and the `AppState`, then pass it to the Axum router.
    *   Update the `ingest_event_handler` to use the publisher from the shared state to forward the event.

**Step 4: Update Integration Tests to Validate the New Flow.**
    *   In `tests/src/common/server_manager.rs`, create a `ServerConfig` enum to define the startup configuration for each server type.
    *   Generalize the `TestServerHandle::new` function to accept the `ServerConfig` and be responsible for creating the correct `AppState` and router for the specified server.
    *   Update the `setup_test_environment` function in `tests/src/single_ray_test.rs` to launch both services and return their handles.
    *   Modify the test logic to use the `ingestion_service` for publishing and the `event_ray_server` for subscribing, confirming the end-to-end flow.
