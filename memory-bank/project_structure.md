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

## Crate-Specific Module Overview:

### 1. `event_ray_server` Crate (`event_ray_server/src/`)

*   **`main.rs`:**
    *   The application's main entry point for the SSE server.
    *   Initializes runtime, shared state (like `AppState`), configures routes, and starts the HTTP server.
*   **`lib.rs`:**
    *   Declares public modules of the `event_ray_server` library, making them accessible for `main.rs` and integration tests.
*   **`app_state.rs`:**
    *   Defines the `AppState` struct, encapsulating shared application state (like the event broadcast sender using `AppEvent` from `event_ray_core`) for the SSE server.
*   **`handlers.rs`:**
    *   Contains Axum request handler functions for the SSE server's API endpoints (e.g., event publishing, SSE connections, health check).
*   **`routes.rs`:**
    *   Defines HTTP routes for the SSE server and maps them to handlers in `handlers.rs`.

### 2. `ingestion_service` Crate (`ingestion_service/src/`)

*   **`main.rs`:**
    *   The main entry point for the Ingestion Service.
    *   Initializes runtime, configures routes, and starts its HTTP server.
*   **`handlers.rs`:**
    *   Contains Axum request handler functions for the Ingestion Service's API endpoints (e.g., event ingestion at `/api/events`, health check at `/health`).
*   **`routes.rs`:**
    *   Defines HTTP routes for the Ingestion Service and maps them to its handlers.

### 3. `event_ray_core` Crate (`event_ray_core/src/`)

*   **`lib.rs`:**
    *   The main library file, declaring and exporting shared modules.
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
    *   Contains shared utilities for testing (e.g., `TestServerHandle`, `SseTestClient`, `PublisherTestClient`).

## Key Data Structures:

*   **`AppEvent` (in `event_ray_core/src/app_event.rs`):**
    *   **Purpose:** Represents a single, discrete event that circulates within the application.
    *   **Key Attributes:** Includes a unique event ID, `ray_id`, timestamp, and payload.

*   **`PublishRequest`, `SseParams` (in `event_ray_core/src/api_models.rs`):**
    *   **Purpose:** Define the structure for API interactions (publishing events, SSE subscriptions).

*   **`AppState` (in `event_ray_server/src/app_state.rs`):**
    *   **Purpose:** To provide shared resources for the `event_ray_server`, like the Tokio broadcast sender for `AppEvent`s.

This workspace structure with distinct crates aims for better separation of concerns, improved build times for individual components, and clearer organization as the project grows.
