# Current Task: Implement Internal "Ray" Event System

**Overall Goal:** Refactor Event Ray to handle event streams based on a "Ray" identifier provided by clients, using an internal broadcast channel.

**Step 1: Define Core Event Structure and Add Dependencies**

1.  **Add Dependencies:** Add `uuid`, `chrono`, `serde`, `serde_json` to `Cargo.toml`. (Interrupted before execution)
2.  **Create `src/event.rs`:** Define the internal `AppEvent` struct.
3.  **Create `src/api_models.rs`:** Define the incoming `PublishRequest` struct.
4.  **Declare Modules:** Add `mod event;` and `mod api_models;` to `src/main.rs`.
5.  **Add `use` Statements:** Add `use event::AppEvent;` and `use api_models::PublishRequest;` to `src/main.rs`.

---

**Step 2: Implement Event Publishing API (Structured)**

1.  **Initialize Broadcast Channel in `main()`:** Create `tokio::sync::broadcast::channel::<AppEvent>(100)` in `src/main.rs`.
2.  **Create `src/app_state.rs`:** Define `AppState` struct containing `broadcast::Sender<AppEvent>`.
3.  **Create `src/handlers.rs`:** Define `health_check` and `publish_event_handler` functions.
4.  **Create `src/routes.rs`:** Define `create_router` function taking `AppState` and routing `/health` and `/api/events`.
5.  **Modify `src/main.rs`:**
    *   Declare new modules (`app_state`, `handlers`, `routes`).
    *   Remove old `mpsc` channel setup, type aliases, handler functions, and router logic.
    *   Instantiate `AppState`.
    *   Call `routes::create_router` to get the router.

---

**Step 3: Implement SSE Handler with Ray-based Filtering (`GET /sse?ray=...`)**

1.  **Define `SseParams` Struct:** In `src/api_models.rs` (or new `src/sse_models.rs`), define `SseParams { ray: String }` for query parameter extraction.
2.  **Create `sse_handler` in `src/handlers.rs`:**
    *   Takes `State<AppState>` and `Query<SseParams>`.
    *   Subscribes to `state.event_sender`.
    *   Loops, receiving events from the broadcast channel.
    *   If `event.ray_id` matches `params.ray`, serializes `event.payload` and yields it as an SSE `Event`.
    *   Handles `Lagged` and `Closed` receiver errors.
    *   Includes SSE keep-alive.
3.  **Update `src/routes.rs`:** Add `GET /sse` route pointing to `handlers::sse_handler` in `create_router`.
4.  **Modify `src/main.rs` (and other files as needed):**
    *   Ensure `SseParams` is correctly defined and imported.
    *   Update `create_router` call in `main.rs` if its signature changed (e.g., removal of temporary parameters).
    *   Ensure all necessary `use` statements for SSE functionality are present in `src/handlers.rs`.
