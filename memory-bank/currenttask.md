**Objective:**

Create a `README.md` file that provides a concise overview of the Event Ray project. The README should:
*   Briefly explain what Event Ray is, its purpose, and its core features, linking to `memory-bank/projectbrief.md` for a more detailed explanation.
*   Link to relevant documents in the `memory-bank/` directory (e.g., `memory-bank/architecture.md`, `memory-bank/project_structure.md`) for more detailed information on the project's design.
*   Provide clear, step-by-step instructions on how to set up and run the project locally.
*   Include example `curl` commands for subscribing to an event stream and publishing an event to demonstrate its functionality.

**Implementation Steps:**

1.  Create a new file named `README.md` in the project root.
2.  Add a main title and a brief introduction describing what Event Ray is.
3.  Add a "Project Details" section with links to the following files for more in-depth information:
    *   `memory-bank/projectbrief.md`
    *   `memory-bank/architecture.md`
    *   `memory-bank/project_structure.md`
4.  Add a "Getting Started" section that lists the prerequisites for building and running the project (e.g., Rust, Cargo).
5.  Add a "How to Run" section with step-by-step instructions for:
    *   Building the workspace.
    *   Running the `event_ray_server`.
    *   Running the `ingestion_service`.
    *   Using `curl` to subscribe to an SSE stream for "ray_1".
    *   Using `curl` to subscribe to an SSE stream for "ray_2".
    *   Using `curl` to publish an event to "ray_1".
    *   Using `curl` to publish an event to "ray_2".
6.  Add a "Testing" section explaining how to run the integration tests.
    *   Include a note that the test suite manages its own server instances, so any manually started servers should be shut down before running the tests to avoid port conflicts.
7.  Add a "Work in Progress" section to note that the project is under active development and subject to change.