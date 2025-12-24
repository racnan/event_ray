# Event Ray Project Structure

**Objective:** This document details the organization of crates and modules, and the general purpose of key data structures within the Event Ray application. For a high-level overview of the event flow, please refer to `architecture.md`.

## Workspace Overview:

The Event Ray project is structured as a Cargo workspace, consisting of several crates:

*   **`event_ray_server` (Binary Crate):**
    *   The main SSE event streaming server.
    *   Handles client subscriptions via SSE and the original event publishing API.
    *   Located in `event_ray_server/`.

*   **`ingestion_service` (Binary Crate):**
    *   An independent service for ingesting events.
    *   Provides a dedicated API endpoint for event submission.
    *   Located in `ingestion_service/`.

*   **`event_ray_core` (Library Crate):**
    *   A shared library containing common data structures and types used by other crates in the workspace.
    *   Located in `event_ray_core/`.

*   **`tests` (Library Crate):**
    *   Contains integration tests for the workspace, primarily focusing on `event_ray_server` functionality.
    *   Located in the root `tests/` directory.

## Feature Flags:

Both `event_ray_server` and `ingestion_service` support the `redis-pubsub` feature flag:

*   **`redis-pubsub`:** Enables Redis Pub/Sub communication mode for scalable event propagation. When enabled:
    *   `ingestion_service` uses `RedisPublisher` to publish events to a Redis channel
    *   `event_ray_server` spawns a background subscriber task to receive events from Redis
    *   When disabled (default), services communicate via direct HTTP requests

## Development Tools:

*   **`justfile`:** Located in the project root, provides convenient commands for running services in different modes, testing, and linting across all feature combinations.

## Crate-Specific Module Overview:

### 1. `event_ray_server` Crate (`event_ray_server/src/`)

*   **`main.rs`:**
    *   The application's main entry point for the SSE server.
    *   Initializes runtime, shared state (like `AppState`), configures routes, and starts the HTTP server.
    *   When the `redis-pubsub` feature is enabled, verifies Redis connectivity and spawns the Redis subscriber task.
*   **`lib.rs`:**
    *   Declares public modules of the `event_ray_server` library, making them accessible for `main.rs` and integration tests.
*   **`app_state.rs`:**
    *   Defines the `AppState` struct, encapsulating shared application state (like the event broadcast sender using `AppEvent` from `event_ray_core`) for the SSE server.
*   **`error.rs`:**
    *   Defines the service-specific `Error` enum for `event_ray_server`, wrapping errors that can occur within its handlers (e.g., broadcast channel failures).
    *   Contains feature-gated error variants for Redis operations (`RedisConnection`, `Deserialization`, `RedisStreamEnded`).
*   **`handlers.rs`:**
    *   Contains Axum request handler functions for the SSE server's API endpoints (e.g., event publishing, SSE connections, health check).
*   **`routes.rs`:**
    *   Defines HTTP routes for the SSE server and maps them to handlers in `handlers.rs`.
*   **`redis_subscriber.rs` (feature-gated with `redis-pubsub`):**
    *   Contains the `run_redis_subscriber` function that subscribes to a Redis Pub/Sub channel and forwards received events to the internal broadcast channel.

### 2. `ingestion_service` Crate (`ingestion_service/src/`)

*   **`main.rs`:**
    *   The main entry point for the Ingestion Service.
    *   Initializes runtime, configures routes, and starts its HTTP server.
    *   Conditionally initializes either `HttpPublisher` (default) or `RedisPublisher` (with `redis-pubsub` feature) based on feature flags.
*   **`app_state.rs`:**
    *   Manages shared application state, including a trait object for the event publisher.
*   **`error.rs`:**
    *   Defines the service-specific `Error` enum for `ingestion_service`, providing generic contexts for failures (e.g., `PublishFailed`).
*   **`handlers.rs`:**
    *   Contains Axum request handler functions for the Ingestion Service's API endpoints (e.g., event ingestion at `/api/events`, health check at `/health`).
*   **`routes.rs`:**
    *   Defines HTTP routes for the Ingestion Service and maps them to its handlers.
*   **`publisher.rs`:**
    *   Defines the `EventPublisher` trait for abstracting event propagation mechanisms.
*   **`publisher/http.rs`:**
    *   Implements `HttpPublisher`, which forwards events to the event_ray_server via HTTP POST.
*   **`publisher/redis.rs` (feature-gated with `redis-pubsub`):**
    *   Implements `RedisPublisher`, which publishes events to a Redis Pub/Sub channel.

### 3. `event_ray_core` Crate (`event_ray_core/src/`)

*   **`lib.rs`:**
    *   The main library file, declaring and exporting shared modules.
*   **`error.rs`:**
    *   Defines the workspace-wide `ApiError` enum, which provides high-level error contexts (`BadRequest`, `InternalServerError`) for creating consistent HTTP responses.
    *   Contains the `ApiErrorResponse` newtype wrapper for converting `error-stack::Report<ApiError>` into an `axum::response::IntoResponse`.
*   **`app_event.rs`:**
    *   Defines the primary internal event structure (`AppEvent`) used across the workspace.
*   **`api_models.rs`:**
    *   Defines data structures for API requests/responses (e.g., `PublishRequest`, `SseParams`) shared across services.

### 4. `tests` Crate (`tests/src/`)

*   **`lib.rs`:**
    *   The main library file for the integration test crate, declaring test modules.
*   **`single_ray_test.rs`:**
    *   Contains integration tests, primarily for the `event_ray_server`.
*   **`common/` (directory with `mod.rs`):**
    *   Contains shared utilities for testing, including a `TestUtilError` enum for unified error handling (`error.rs`), and clients for the server (`TestServerHandle`, `SseTestClient`, `PublisherTestClient`).

## Key Data Structures:

*   **`AppEvent` (in `event_ray_core/src/app_event.rs`):**
    *   **Purpose:** Represents a single, discrete event that circulates within the application.
    *   **Key Attributes:** Includes a unique event ID, `ray_id`, timestamp, and payload.

*   **`PublishRequest`, `SseParams` (in `event_ray_core/src/api_models.rs`):**
    *   **Purpose:** Define the structure for API interactions (publishing events, SSE subscriptions).

*   **`AppState` (in `event_ray_server/src/app_state.rs`):**
    *   **Purpose:** To provide shared resources for the `event_ray_server`, like the Tokio broadcast sender for `AppEvent`s.

*   **`ApiError`, `ApiErrorResponse` (in `event_ray_core/src/error.rs`):**
    *   **Purpose:** `ApiError` provides high-level classification of errors for HTTP responses. `ApiErrorResponse` wraps an `error-stack::Report` to provide a consistent, centralized `IntoResponse` implementation for all services.

This workspace structure with distinct crates aims for better separation of concerns, improved build times for individual components, and clearer organization as the project grows.
