# Project Brief: Event Ray

**Event Ray** is an open-source, high-performance event streaming server. It provides a scalable and reliable infrastructure for **Server-Sent Events (SSE)**, enabling backend services to push real-time updates to clients over HTTP.

## Goals

- Deliver a production-grade SSE server optimized for low latency and high concurrency.
- Ensure the service is highly scalable to handle thousands of concurrent SSE connections efficiently.
- Provide a simple, extensible architecture that gets events from backend services using multiple technologies like APIs, Kafka, etc.
- Support fine-grained client subscriptions and message filtering over SSE.
- Support for authentication based event subscription.
- Offer an easy integration path for developers using modern frontends and microservices.

## Positioning

Unlike WebSocket-based systems, Event Ray is built for **unidirectional, push-only** scenarios — where simplicity, browser compatibility, and resource efficiency are paramount.

## Technologies Used

*   **Rust:** The core programming language, chosen for its performance, safety, and concurrency features.
*   **Tokio:** An asynchronous runtime for Rust, providing the foundation for non-blocking I/O operations essential for a high-concurrency server.
*   **Axum:** A web framework built on Tokio, used to handle HTTP requests, routing, and specifically the Server-Sent Events (SSE) endpoint.
*   **Docker:** Used for containerizing the application, ensuring consistent builds and simplifying deployment.
