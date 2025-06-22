# Task: Create Independent Ingestion Service and Refactor to Cargo Workspace

## Objective:
To restructure the Event Ray project into a Cargo workspace and create a new, independent 'Ingestion Service' as a separate binary with its own API endpoint for receiving events. This new service will be containerized. The existing `event_ray_server` binary will retain its current event publishing API and SSE functionalities to ensure ongoing operational stability and test compatibility. The new Ingestion Service will not yet be integrated into the primary event delivery flow.

## Implementation Plan/Steps:

**Step 1: Restructure Project into a Cargo Workspace.**
    *   Create a new root `Cargo.toml` file to define the workspace.
    *   Specify the initial members of the workspace in the root `Cargo.toml` (this will initially be just the existing application, which we'll rename).
    *   Create a new directory, for example, `event_ray_server/`.
    *   Move the existing `src/` directory, the existing `Cargo.toml` file, and any other project-specific root files (like `Dockerfile`, `.dockerignore` if they are specific to this binary) into the `event_ray_server/` subdirectory.
    *   Modify the `event_ray_server/Cargo.toml` (the moved one):
        *   Change its `[package]` name to `event_ray_server` (or a similar distinct name).
        *   Ensure its `[[bin]]` or `[lib]` sections are correctly defined if they exist.
        *   Adjust any relative paths in this `Cargo.toml` if necessary (though usually not needed for `path` dependencies if they are also moved or become workspace members).
    *   Update path references in `event_ray_server/src/main.rs` or `event_ray_server/src/lib.rs` if they refer to modules using the old crate name (e.g., if `main.rs` used `use event_ray::some_module;`, it might need to become `use event_ray_server::some_module;` or just `use crate::some_module;` depending on structure).
    *   Verify that the project compiles successfully as a workspace using `cargo check --workspace` or `cargo check -p event_ray_server`.
    *   Ensure existing tests still pass by running `cargo test --workspace` or `cargo test -p event_ray_server`.

**Step 2: Create a Shared Library Crate (e.g., `event_ray_core`).**
    *   Create a new directory, for example, `event_ray_core/`.
    *   Initialize it as a new Rust library crate with its own `event_ray_core/Cargo.toml` and `event_ray_core/src/lib.rs`.
    *   Add `event_ray_core` as a member to the root workspace `Cargo.toml`.
    *   Identify common data structures currently in `event_ray_server/src/` (previously `src/`) that will be needed by both the `event_ray_server` and the future `ingestion_service`. This primarily includes:
        *   `AppEvent` struct (likely from `event_ray_server/src/event.rs`).
        *   API model structs like `PublishRequest`, `SseParams` (likely from `event_ray_server/src/api_models.rs`).
    *   Move the definitions of these shared structures into the `event_ray_core/src/lib.rs` (or into appropriate modules within `event_ray_core/src/`).
    *   Add any necessary dependencies for these shared structures (e.g., `serde`, `uuid`, `chrono`) to `event_ray_core/Cargo.toml`.
    *   Update the `event_ray_server` crate:
        *   Add `event_ray_core` as a path dependency in `event_ray_server/Cargo.toml`.
        *   Modify `use` statements in `event_ray_server` code to refer to the shared items via the `event_ray_core` crate (e.g., `use event_ray_core::AppEvent;`).
    *   Verify that the project still compiles (`cargo check --workspace`) and existing tests pass (`cargo test --workspace`).

**Step 3: Create the New `ingestion_service` Crate.**
    *   Create a new directory, for example, `ingestion_service/`.
    *   Initialize it as a new Rust binary crate with its own `ingestion_service/Cargo.toml` and `ingestion_service/src/main.rs`.
    *   Add `ingestion_service` as a member to the root workspace `Cargo.toml`.
    *   In `ingestion_service/Cargo.toml`, add dependencies on:
        *   The shared `event_ray_core` crate (as a path dependency).
        *   Axum, Tokio (with features like `macros`, `rt-multi-thread`), Serde, Serde JSON, and any other foundational libraries needed for a basic web service.
    *   Verify that the empty `ingestion_service` crate compiles as part of the workspace (`cargo check -p ingestion_service` and `cargo check --workspace`).

**Step 4: Implement Basic `ingestion_service` Structure with Health Check.**
    *   In `ingestion_service/src/main.rs`, set up a basic Axum server listening on a configurable port (e.g., read from an environment variable, defaulting to a port different from `event_ray_server`, like 8081).
    *   Define a health check API route, for example, `GET /health`.
    *   Implement an Axum handler function for this `/health` route. This handler should:
        *   Simply return an HTTP `200 OK` response, possibly with a minimal JSON body like `{"status": "ok"}` or just plain text "OK".
    *   Ensure the `ingestion_service` compiles (`cargo check -p ingestion_service`) and can be run (`cargo run -p ingestion_service`).
    *   Manually test the new `/health` endpoint using a tool like `curl` or a web browser. Verify that the service returns the correct HTTP status and body.

**Step 5: Implement Event Ingestion Endpoint in `ingestion_service`.**
    *   In `ingestion_service/src/main.rs`, define a new API route for event ingestion, for example, `POST /api/ingest/events`.
    *   Implement an Axum handler function for this route. This handler should:
        *   Accept a JSON request body, expected to conform to the `PublishRequest` struct (from `event_ray_core::api_models`).
        *   Deserialize the request body into a `PublishRequest` object.
        *   If deserialization is successful, construct an `AppEvent` (from `event_ray_core::event`) using data from the request (like `ray_id`, `payload`) and generate a new `id` (e.g., UUID) and `timestamp`.
        *   For this version, the handler should log the successfully created `AppEvent` (e.g., using `println!` or a logging crate).
        *   Return an appropriate HTTP response (e.g., `202 Accepted` or `201 Created` on success, or `400 Bad Request` / `500 Internal Server Error` on failure).
    *   Ensure the `ingestion_service` compiles (`cargo check -p ingestion_service`) and can be run (`cargo run -p ingestion_service`).
    *   Manually test the new `/api/ingest/events` endpoint using a tool like `curl` or Postman. Verify that the service logs the event details and returns the correct HTTP status.

**Step 6: Create/Update Dockerfiles for Both Services.**

    *   **A. Update Existing Dockerfile for `event_ray_server`:**
        *   Rename the existing root `Dockerfile` to `Dockerfile.event_ray_server` (or move it to `event_ray_server/Dockerfile` and adjust paths if preferred).
        *   Modify this Dockerfile:
            *   The `WORKDIR` might remain `/usr/src/event_ray` or change if desired.
            *   The `COPY . .` command will copy the entire workspace.
            *   The `RUN cargo build --release` command needs to be changed to specify the `event_ray_server` package: `RUN cargo build --release -p event_ray_server`.
            *   The `CMD` needs to be updated to point to the correct binary path: `CMD ["./target/release/event_ray_server"]`.
            *   The `EXPOSE` port should remain as it is for the `event_ray_server` (e.g., 8080 or its current port).
        *   Build the Docker image for `event_ray_server` (e.g., `docker build -t event-ray-server -f Dockerfile.event_ray_server .`).
        *   Run the container and test its existing functionality.

    *   **B. Create New Dockerfile for `ingestion_service`:**
        *   Create a new Dockerfile named `Dockerfile.ingestion_service` in the project root.
        *   This Dockerfile will be similar to `Dockerfile.event_ray_server`:
            *   `FROM rust`
            *   `WORKDIR /usr/src/event_ray` (or a new workdir, but sharing can be fine for builds)
            *   `COPY . .` (copies the entire workspace)
            *   `RUN cargo build --release -p ingestion_service` (builds only the ingestion_service binary)
            *   `EXPOSE 8081` (or the configured port for the ingestion service)
            *   `CMD ["./target/release/ingestion_service"]`
        *   Build the Docker image for `ingestion_service` (e.g., `docker build -t ingestion-service -f Dockerfile.ingestion_service .`).
        *   Run the containerized `ingestion_service` (`docker run -p 8081:8081 ingestion-service`).
        *   Test both the `/health` and `/api/ingest/events` endpoints of the containerized service.
