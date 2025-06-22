# Event Ray Application Architecture: Event Flow

**Objective:** This document outlines the high-level event flow within the Event Ray application, a Server-Sent Events (SSE) streaming server. For details on specific data structures, modules, and handler responsibilities, please refer to `project_structure.md`.

## Core Architectural Concepts:

*   **Central Event Bus:** The system utilizes a central, in-memory broadcast mechanism (specifically, a Tokio broadcast channel) to distribute events.
*   **Decoupled Publishing and Subscribing:** Event producers publish events to this central bus without direct knowledge of the subscribers. Event consumers subscribe to the bus and receive events they are interested in.
*   **Ray-Based Filtering:** Clients subscribe to specific "rays" (event streams). The server filters events from the central bus, delivering only those matching a client's subscribed ray.

## High-Level Event Flow:

1.  **Initialization:**
    *   The application starts and initializes its core components, including the central event bus and the shared state that provides access to this bus.
    *   HTTP endpoints for publishing events and subscribing to SSE streams are set up.

2.  **Event Publishing:**
    *   An external client or internal process sends a request (typically `POST`) to a designated API endpoint to publish a new event. This request includes data for the event, notably an identifier for the target "ray" (event stream) and the event's content (payload).
    *   The server's request handler for this endpoint processes the incoming data, prepares it as an internal event, and sends this internal event into the central event bus.

3.  **Event Subscription (SSE):**
    *   Clients initiate an SSE connection by sending a request (typically `GET`) to a designated SSE endpoint. This request includes a parameter specifying the "ray" they wish to subscribe to.
    *   The server's request handler for SSE connections establishes a persistent connection with the client.
    *   This handler subscribes to the central event bus to receive all broadcasted internal events.

4.  **Event Filtering and Delivery:**
    *   As internal events are broadcast on the central bus, each active SSE handler (representing a connected client) receives them.
    *   Each SSE handler inspects the received internal event and compares its "ray" identifier with the "ray" its client subscribed to.
    *   If the identifiers match, the handler formats the event's content appropriately for SSE and streams it to its connected client.
    *   If the identifiers do not match, the handler discards the event for that particular client.

---
This flow ensures that events are efficiently broadcast and selectively delivered to clients based on their specific subscriptions, forming the core of Event Ray's real-time capabilities.
