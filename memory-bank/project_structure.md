# Event Ray Project Structure

**Objective:** This document details the organization of modules and the general purpose of key data structures within the Event Ray application. For a high-level overview of the event flow, please refer to `architecture.md`.

## Module Overview:

The `src` directory contains the core logic of the application, organized into the following modules:

*   **`main.rs`:**
    *   The application's main entry point.
    *   Responsible for initializing the runtime, setting up shared state, configuring routes, and starting the HTTP server.
    *   Declares other top-level modules.

*   **`event.rs`:**
    *   Defines the primary internal event structure (`AppEvent`) used within the server.

*   **`api_models.rs`:**
    *   Defines data structures used for serializing and deserializing API request/response data and query parameters. These models facilitate communication with external clients.

*   **`app_state.rs`:**
    *   Defines the `AppState` struct, which encapsulates shared application state (like the event broadcast sender) accessible by request handlers.

*   **`handlers.rs`:**
    *   Contains Axum request handler functions. Each function implements the business logic for a specific API endpoint.

*   **`routes.rs`:**
    *   Contains the routing logic. It defines all HTTP routes and maps them to their respective handler functions in `handlers.rs`, also managing the injection of shared state into handlers.

## Key Data Structures:

*   **`AppEvent` (in `src/event.rs`):**
    *   **Purpose:** Represents a single, discrete event that circulates within the application.
    *   **Key Attributes:** Typically includes a unique event ID, an identifier for the event stream it belongs to (`ray_id`), a timestamp, and the event's actual data (`payload`).

*   **`AppState` (in `src/app_state.rs`):**
    *   **Purpose:** To provide a centralized way to share resources and state across different request handlers.
    *   **Key Attribute:** Contains the sender part of the Tokio broadcast channel, enabling handlers to publish events to the central event bus.

This modular structure aims to keep concerns separated, making the application more understandable, testable, and maintainable.
