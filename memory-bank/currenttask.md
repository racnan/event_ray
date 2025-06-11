# Current Task: Implement End-to-End Testing for Event Ray

## Objective
Develop an end-to-end testing framework for Event Ray that validates the core SSE functionality by:
1. Starting the server programmatically
2. Establishing SSE connections to different rays
3. Publishing events to specific rays
4. Verifying events are delivered to the correct subscribers

## Approach
Implement an external test client that interacts with the Event Ray server through its HTTP interfaces, simulating real-world usage patterns. The test framework will use HTTP clients to make API calls and establish SSE connections, allowing for comprehensive validation of the event filtering and delivery mechanisms.

## Implementation Plan

### Step 1: Set up test directory structure and dependencies
1. Create a `tests` directory at the project root to house all integration tests
2. Create a single initial test file `tests/integration_test.rs` that will contain all test code to start with
   - We can refactor into multiple files as needed in later steps
3. Add required dependencies using `cargo add`:
   - `reqwest` - HTTP client for making API calls and establishing SSE connections
   - `tokio-test` - For testing async code
   - `futures` - For working with async streams
   - `uuid` - For generating unique test identifiers
   - `eventsource-client` or similar - For SSE client functionality
4. Modify Cargo.toml to enable necessary features for the added dependencies:
   - Enable `json` and `stream` features for reqwest
   - Enable any other required features for the dependencies

### Step 2: Create server management module
This step involves creating utilities in `tests/server_manager.rs` to programmatically start and stop the Event Ray server for integration tests.

1.  **Define `TestServerHandle` Struct:**
    *   Location: `tests/server_manager.rs`.
    *   Fields:
        *   `join_handle: tokio::task::JoinHandle<Result<(), std::io::Error>>` (for the server task).
        *   `shutdown_tx: tokio::sync::oneshot::Sender<()>` (to signal server shutdown).
        *   `base_url: String` (e.g., "http://127.0.0.1:PORT", for clients to connect to).

2.  **Implement `TestServerHandle::new(app_state: AppState)` Asynchronous Constructor:**
    *   Accepts an `AppState` instance (created by the calling test).
    *   Binds a `TcpListener` to a dynamically assigned free port on `127.0.0.1:0`.
    *   Constructs the `base_url` from the listener's actual address.
    *   Uses the application's `event_ray::routes::create_router(app_state)` to get the router.
    *   Spawns the Axum server in a Tokio task, configured with graceful shutdown via a `oneshot` channel.
    *   Performs a readiness check by repeatedly pinging the server's `/health` endpoint until it responds successfully or a timeout occurs.
    *   Returns an instance of `TestServerHandle`.

3.  **Implement `TestServerHandle::stop()` Asynchronous Method:**
    *   Sends a signal via `shutdown_tx`.
    *   Awaits the `join_handle` to ensure the server task completes.

4.  **Module Visibility for Tests:**
    *   The necessary components from the main application (e.g., `event_ray::routes::create_router`, `event_ray::app_state::AppState`, `event_ray::event::AppEvent`) must be made `pub` so they are accessible from the integration tests in the `tests` directory.

5.  **Usage in Test Files (e.g., `tests/single_ray_test.rs`):**
    *   Individual test functions will be responsible for:
        1.  Creating a `tokio::sync::broadcast::channel` for `AppEvent`.
        2.  Creating an `AppState` instance with the sender from this channel.
        3.  Calling `TestServerHandle::new(app_state)` to start the server.
        4.  Verifying the server is accessible (e.g., via its `base_url`).
        5.  Calling `server.stop().await` in a teardown phase to shut down the server.
    *   The primary goal for this step's usage is to confirm the server can be started and stopped programmatically. Actual event publishing and SSE interaction tests will be covered in subsequent steps.

This approach allows each test to run an isolated, fully functional instance of the Event Ray server.

### Step 3: Implement SSE client
This step focuses on creating a reusable asynchronous SSE client in `tests/sse_client.rs` using the `eventsource_client` crate.

1.  **Add `eventsource_client` Crate Dependency:**
    *   Add `eventsource_client` to `[dev-dependencies]` in `Cargo.toml`.
    *   Also ensure `futures` is available (likely already a transitive or direct dev-dependency).

2.  **Define `SseTestClient` Struct in `tests/sse_client.rs`:**
    *   Fields:
        *   `stream: Pin<Box<dyn Stream<Item = Result<eventsource_client::SSE<eventsource_client::Event>, eventsource_client::Error>> + Send>>` (The asynchronous stream of SSEs from the library).
        *   `_client_keep_alive: eventsource_client::Client` (The client instance, kept to ensure the connection remains active as long as the `SseTestClient` exists. The stream itself might not keep the connection alive if the client is dropped).

3.  **Implement `SseTestClient::connect(base_url: &str, ray_id: &str) -> Result<Self, TestClientError>` Asynchronous Constructor:**
    *   Construct the full SSE URL: `format!("{}/sse?ray={}", base_url, ray_id)`.
    *   Use `eventsource_client::ClientBuilder::for_url(&sse_url)?.build()` to create the client instance.
    *   Get the event stream: `let stream = Box::pin(client_instance.stream());`.
    *   Store the `stream` and the `client_instance` (as `_client_keep_alive`) in the `SseTestClient`.
    *   Return `Ok(Self)`.

4.  **Implement `async fn receive_event(&mut self, timeout: Duration) -> Result<AppEvent, TestClientError>`:**
    *   Use `tokio::time::timeout(timeout, self.stream.try_next()).await`.
    *   Handle the `Result` from `timeout`:
        *   If `Ok(Ok(Some(sse_enum)))`:
            *   Match on `sse_enum`:
                *   `eventsource_client::SSE::Event(event)`:
                    *   Check `event.event_type` (e.g., if it's "message" or the default used by Event Ray).
                    *   Deserialize `event.data` (String) from JSON into an `event_ray::event::AppEvent`.
                    *   Return `Ok(deserialized_app_event)`.
                *   `eventsource_client::SSE::Comment(_)`: Ignore or log.
                *   `eventsource_client::SSE::Connected(_)`: Can be used for an initial readiness check if needed, or ignored.
            *   If the event type is not what's expected, return `Err(TestClientError::UnexpectedEventType)`.
        *   If `Ok(Ok(None))`, the stream ended: return `Err(TestClientError::StreamEnded)`.
        *   If `Ok(Err(e))`, an error from the stream: return `Err(TestClientError::from_eventsource_error(e))`.
        *   If `Err(_timeout_error)`, a timeout occurred: return `Err(TestClientError::Timeout)`.

5.  **Cleanup (Rely on `Drop`):**
    *   When `SseTestClient` is dropped, its `_client_keep_alive` (the `eventsource_client::Client` instance) and `stream` will be dropped. The `eventsource_client` library should handle closing the connection gracefully. No explicit `close()` method should be necessary unless the library documentation specifically recommends one for its `Client` type.

6.  **Define `TestClientError` Enum in `tests/sse_client.rs`:**
    *   To consolidate errors from `eventsource_client::Error`, `serde_json::Error`, timeouts, stream closure, and unexpected event types.

7.  **`AppEvent` Requirements:**
    *   Ensure `event_ray::event::AppEvent` is `pub` and derives `serde::Deserialize`, `Debug`, and `PartialEq`.

### Step 4: Implement event publisher client
This step focuses on creating a reusable asynchronous client in `tests/publisher_client.rs` to send events to the Event Ray server's `/api/events` endpoint.

1.  **Create `publisher_client.rs`:**
    *   A new file `tests/publisher_client.rs` will house the publisher client logic.

2.  **Dependencies (ensure they are in `[dev-dependencies]` in `Cargo.toml`):**
    *   `reqwest` (with `json` feature): For making HTTP POST requests.
    *   `event_ray::api_models::PublishRequest`: The request body structure from `src/api_models.rs`.

3.  **Define `PublisherTestClient` Struct in `tests/publisher_client.rs`:**
    *   Fields:
        *   `http_client: reqwest::Client`
        *   `base_url: String`

4.  **Implement `PublisherTestClient::new(base_url: String) -> Self` Constructor:**
    *   Creates a new `reqwest::Client`.
    *   Stores the `http_client` and `base_url`.

5.  **Implement `async fn publish_event(&self, ray_id: &str, payload: String) -> Result<(), reqwest::Error>`:**
    *   Construct the full publish URL: `format!("{}/api/events", self.base_url)`.
    *   Create an instance of `event_ray::api_models::PublishRequest`:
        ```rust
        let request_body = event_ray::api_models::PublishRequest {
            ray_id: ray_id.to_string(),
            payload, // This is a String as per src/api_models.rs
        };
        ```
    *   Send an HTTP POST request using `self.http_client`.
    *   Call `response.error_for_status()?` on the `reqwest::Response`. This will return `Ok(response)` if the status is a success (2xx), or `Err(reqwest::Error)` if it's a client or server error status (4xx, 5xx).
    *   If `error_for_status()` returns `Ok(_)`, then return `Ok(())`. The `reqwest::Error` will be propagated otherwise.

6.  **No Custom Error Enum:** Errors will be propagated as `reqwest::Error`. Tests will use `.expect()` or match on this error type.

7.  **`PublishRequest` Requirements:**
    *   `event_ray::api_models::PublishRequest` is already `pub` and derives `serde::Serialize` and `serde::Deserialize`.

### Step 5: Create Basic Single Ray Test Scenario
This step involves writing the first integration test, specifically `test_publish_and_receive_single_event_on_ray()`, which will reside in a new file `tests/single_ray_test.rs` (or a similarly named file like `single_ray_tests.rs`). This test will validate the fundamental behavior of publishing to and receiving from a single event stream ("ray"). It will use the `TestServerHandle`, `SseTestClient`, and `PublisherTestClient` developed in previous steps.

**Test File:** `tests/single_ray_test.rs` (or `tests/single_ray_tests.rs`)

**Core Test Scenario:**

1.  **`test_publish_and_receive_single_event_on_ray()`**
    *   **Objective:** Verify that an event published to a specific ray is received by a client subscribed to that same ray.
    *   **Setup:**
        1.  Start the Event Ray server using `TestServerHandle::new()`. This involves creating an `AppState` with a `broadcast::channel`.
        2.  Generate a unique `ray_id` (e.g., using `uuid::Uuid::new_v4().to_string()`).
        3.  Create an `SseTestClient` and connect it to this `ray_id` using `SseTestClient::connect()`. Expect success.
        4.  Create a `PublisherTestClient` using the server's `base_url`.
    *   **Action:**
        1.  Define a sample event payload (e.g., a JSON string like `r#"{"message": "hello ray"}"#`).
        2.  Use `PublisherTestClient::publish_event()` to send this payload to the generated `ray_id`. Expect success.
    *   **Verification:**
        1.  Use `SseTestClient::receive_event()` with a reasonable timeout (e.g., 5 seconds).
        2.  Assert that the result is `Ok(Some(app_event))`.
        3.  Verify that `app_event.ray_id` matches the published `ray_id`.
        4.  Verify that `app_event.payload` matches the published payload string.
        5.  (Optional: Check that `app_event.id` is a valid UUID string, if `AppEvent` includes it).
    *   **Teardown:**
        1.  Call `TestServerHandle::stop()` to shut down the server. Expect success.

**General Test Structure (within `tests/single_ray_test.rs`):**

```rust
// In tests/single_ray_test.rs
#[cfg(test)]
mod tests { // The module name inside the file can be just 'tests'
    use event_ray::app_state::AppState;
    use event_ray::event::AppEvent;
    // Assuming helper modules (server_manager, sse_client, publisher_client)
    // are declared in a common place like tests/integration_test.rs or tests/common/mod.rs
    // and are accessible via crate::tests::... or similar path.
    // For example:
    // use crate::tests::server_manager::TestServerHandle;
    // use crate::tests::sse_client::SseTestClient;
    // use crate::tests::publisher_client::PublisherTestClient;

    use std::time::Duration;
    use tokio::sync::broadcast;
    use uuid::Uuid;

    // Placeholder for actual import paths if helper modules are structured:
    // This assumes TestServerHandle, SseTestClient, PublisherTestClient are made available.
    // If they are in files like tests/server_manager.rs, tests/sse_client.rs etc.,
    // those files need to be declared as modules in tests/integration_test.rs (acting as lib.rs for tests)
    // e.g. in tests/integration_test.rs:
    // pub mod server_manager;
    // pub mod sse_client;
    // pub mod publisher_client;
    // Then here:
    // use crate::integration_test::{server_manager::TestServerHandle, sse_client::SseTestClient, publisher_client::PublisherTestClient};


    async fn setup_test_environment() -> (TestServerHandle, PublisherTestClient, String /* base_url */) {
        let (event_tx, _) = broadcast::channel::<AppEvent>(1024);
        let app_state = AppState::new(event_tx);
        // The actual types for TestServerHandle etc. would be resolved by the imports above
        let server = TestServerHandle::new(app_state).await.expect("Failed to start server");
        let publisher_client = PublisherTestClient::new(server.base_url.clone());
        (server, publisher_client, server.base_url.clone())
    }

    #[tokio::test]
    async fn test_publish_and_receive_single_event_on_ray() {
        let (server_handle, publisher, base_url) = setup_test_environment().await;
        let ray_id = Uuid::new_v4().to_string();
        let payload_str = r#"{"message":"event_payload_content"}"#.to_string();

        let mut sse_client = SseTestClient::connect(&base_url, &ray_id)
            .await
            .expect("Failed to connect SSE client");

        publisher.publish_event(&ray_id, payload_str.clone())
            .await
            .expect("Failed to publish event");

        match sse_client.receive_event(Duration::from_secs(5)).await {
            Ok(Some(event)) => {
                assert_eq!(event.ray_id, ray_id);
                assert_eq!(event.payload, payload_str);
                // If AppEvent has an 'id' field:
                // assert!(!event.id.is_empty(), "Event ID should not be empty");
            }
            Ok(None) => panic!("Stream ended prematurely, expected an event"),
            Err(e) => panic!("Failed to receive event: {:?}", e),
        }

        server_handle.stop().await.expect("Failed to stop server");
    }
}
```

**Key Considerations for Step 5:**
*   **Focus:** This step now exclusively covers the implementation of the `test_publish_and_receive_single_event_on_ray` scenario in `tests/single_ray_test.rs`. Other scenarios will be deferred.
*   **File Structure:** The test utilities (`server_manager.rs`, `sse_client.rs`, `publisher_client.rs`) will be created as separate files. `tests/integration_test.rs` might serve as the integration test crate root (like a `lib.rs` for tests) to declare these utility modules and the `single_ray_test.rs` module.
*   **Error Handling:** Continues to use `.expect()` and matching on `Result<Option<AppEvent>, Box<dyn Error>>` from `SseTestClient::receive_event()`.
*   **`AppEvent` Structure:** Assumes `AppEvent` is `pub`, derives `serde::Deserialize`, `Debug`, `PartialEq`, and its `payload` is a `String`.
