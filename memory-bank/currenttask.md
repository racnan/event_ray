**Objective:** Overhaul the project's error handling by implementing a layered and scalable system using the `error-stack` and `thiserror` libraries.

The foundation of this system will be established in the `event_ray_core` crate. We will introduce a new `ApiError` type that will serve as a high-level `error-stack::Context`, responsible for categorizing errors (e.g., `BadRequest`, `InternalServerError`) to ensure consistent HTTP responses across all services.

Each service will then define its own specific error contexts, using `thiserror` for clear definitions. These specific errors will be attached to an `error_stack::Report`, preserving the full, typed error chain for rich, traceable logging and debugging, before being categorized by the central `ApiError` at the application boundary.

---
**Implementation Plan**

**Step 1: Set up `event_ray_core` dependencies.**
*   Run the following shell commands from the workspace root to add the latest versions of the required dependencies to the `event_ray_core` crate:
    *   `cargo add thiserror --package event_ray_core`
    *   `cargo add error-stack --package event_ray_core`
    *   `cargo add axum --package event_ray_core --no-default-features --features "json"`

**Step 2: Define the core `ApiError` context.**
*   Create a new file at `event_ray_core/src/error.rs`.
*   Inside this file, define a public `ApiError` enum with `BadRequest` and `InternalServerError` variants.
*   Use `thiserror` (`#[derive(Error)]` and `#[error("...")]`) to automatically implement the `Display` trait for `ApiError`.
*   Implement the `error_stack::Context` marker trait for `ApiError`.
*   In `event_ray_core/src/lib.rs`, add `pub mod error;` to make the new module public.

**Step 3: Implement Axum integration for `ApiError`.**
*   In `event_ray_core/src/error.rs`, implement the `axum::response::IntoResponse` trait for `error_stack::Report<ApiError>`. This implementation will contain the logic to convert an error report into a final HTTP `StatusCode` and response body.
*   In the same file, implement the `From<axum::extract::rejection::JsonRejection>` trait for `error_stack::Report<ApiError>`. This will be the bridge that automatically converts Axum's JSON parsing errors into our `ApiError::BadRequest`.

**Step 4: Set up `event_ray_server` error enum.**
*   Run `cargo add error-stack --package event_ray_server` and `cargo add thiserror --package event_ray_server` to add the dependencies.
*   Create a new file at `event_ray_server/src/error.rs`.
*   Inside this file, define a single, public `Error` enum for the service. This enum will have variants to wrap **recoverable, request-time failures**, including:
    *   `BroadcastSend(#[from] tokio::sync::broadcast::error::SendError<AppEvent>)`
    *   `BroadcastRecv(#[from] tokio::sync::broadcast::error::RecvError)`
*   Implement the `Display` (via `thiserror`) and `error_stack::Context` traits for this `Error` enum.
*   In `event_ray_server/src/lib.rs`, add `pub mod error;` to make the new module public.

**Step 5: Refactor `event_ray_server` handlers and enable error reporting.**
*   In `event_ray_server/src/handlers.rs`, refactor the `publish_event_handler` function:
    *   Change its return type to `Result<impl IntoResponse, error_stack::Report<event_ray_core::error::ApiError>>`.
    *   Replace the existing error handling for the `state.tx.send()` call. When an error occurs, create an error report with the `event_ray_server::error::Error::BroadcastSend` context, and then attach the high-level `event_ray_core::error::ApiError::InternalServerError` context before returning.
*   In the `sse_handler` function, refactor the error handling inside the `loop`:
    *   In the `Err` case of the `rx.recv()` match, create a report using the `event_ray_server::error::Error::BroadcastRecv` context and **print the full error report to the console.**
*   In `event_ray_core/src/error.rs`, update the `IntoResponse` implementation for `Report<ApiError>` to **print the full error report to the console** before returning the final HTTP response.

**Step 6: Set up `ingestion_service` error enum.**
*   Run `cargo add error-stack --package ingestion_service` and `cargo add thiserror --package ingestion_service` to add the dependencies.
*   Create a new file at `ingestion_service/src/error.rs`.
*   Inside this file, define a single, public `Error` enum for the service. The primary variant will be to wrap errors from the `reqwest` HTTP client used by the publisher: `PublisherRequest(#[from] reqwest::Error)`.
*   Implement the `Display` (via `thiserror`) and `error_stack::Context` traits for this `Error` enum.
*   In `ingestion_service/src/lib.rs`, add `pub mod error;` to make the new module public.

**Step 7: Refactor `ingestion_service` logic.**
*   In `ingestion_service/src/publisher.rs`, update the `EventPublisher` trait's `publish` method to return a `Result` that contains a `Report` of the new `ingestion_service::error::Error` on failure.
*   In `ingestion_service/src/publisher/http.rs`, update the `HttpPublisher::publish` implementation to match the new trait signature. It should create and return a `Report` on failure, using the new `Error` context variants.
*   In `ingestion_service/src/handlers.rs`, refactor the `ingest_event_handler` to return `Result<impl IntoResponse, error_stack::Report<event_ray_core::error::ApiError>>`. It should now use the `?` operator on the `publish_event` call and attach the appropriate `ApiError` context on failure.

**Step 8: Refactor test utilities and update existing test.**
*   In the `tests/src/common/` module, create a new error module to define a `TestUtilError` enum using `thiserror`. It should have variants for I/O, `reqwest`, and `eventsource_client` errors.
*   Update the function signatures in `server_manager.rs` and `sse_client.rs` to return this new `TestUtilError` instead of `Result<..., Box<dyn Error>>`.
*   In `single_ray_test.rs`, update the `test_publish_and_receive_single_event_on_ray` test case.
*   Modify the `.expect()` calls to properly handle the new `TestUtilError` returned by the refactored test utility functions, ensuring the test continues to pass.

**Step 9: Add integration test for `400 Bad Request`.**
*   In `single_ray_test.rs`, create a new test case.
*   This test will call the `ingestion_service`'s `/api/events` endpoint with a deliberately malformed JSON body.
*   The test will assert that the HTTP response has a `400 Bad Request` status code.
