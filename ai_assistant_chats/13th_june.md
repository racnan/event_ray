# Summary of Planning Discussion: Event Ray Load Testing & Horizontal Scalability (June 13th)

This document summarizes the key points and decisions from our planning session regarding load testing and implementing horizontal scalability for the Event Ray project.

## 1. Initial Objective & Context

*   The primary goal was to devise a strategy for load testing Event Ray and to explore methods for testing its horizontal scalability.
*   It was noted that `memory-bank/currenttask.md` was empty, leading to a planning-focused discussion rather than direct implementation.

## 2. Load Testing Strategy Formulation

*   **Key Performance Indicators (KPIs):** We identified the need to define critical metrics for load testing, including:
    *   Maximum concurrent Server-Sent Events (SSE) connections.
    *   Event throughput (events per second, both per-ray and system-wide).
    *   End-to-end event latency (from API publish to SSE client reception).
    *   Server resource utilization (CPU, memory, network I/O) under various load conditions.
    *   Error rates during high load.
*   **Tool Selection:** Discussed potential tools for generating load:
    *   For SSE clients: `k6` (with SSE extensions), `Artillery`, or enhancing existing custom Rust test clients.
    *   For API event publishers: `k6`, `Artillery`, `wrk`, `ab`, or custom scripts.
*   **Test Scenarios:** Outlined various scenarios to simulate different load patterns:
    *   Gradual ramp-up of SSE connections and/or event publishing rates.
    *   Sustained high load (soak tests) to check for stability and memory leaks.
    *   Spike tests to observe behavior under sudden bursts of traffic.
    *   Tests varying the number of unique `ray_id`s versus subscribers per `ray_id`.
*   **Environment & Monitoring:** Emphasized the need for a dedicated testing environment and robust server-side monitoring.

## 3. Horizontal Scalability: Initial Architectural Considerations

*   **Current Limitation:** Recognized that Event Ray's current architecture, using an in-memory Tokio broadcast channel (`memory-bank/architecture.md`), inherently limits true horizontal scaling. Events published to one instance are not visible to clients connected to other independent instances.
*   **Need for Shared Backplane:** Concluded that a shared event backplane (e.g., Kafka, Redis Streams, NATS JetStream) is essential for instances to share event information and for clients connected to any instance to receive relevant events.
*   **Prioritization:** A critical discussion point was whether to focus on comprehensive load testing of the current single-instance architecture first or to prioritize implementing the horizontal scaling mechanism.
    *   **Decision:** It was agreed that building the architecture for horizontal scaling (i.e., integrating a shared message broker) should take precedence. Exhaustively testing the current single-instance model would provide limited insight into the performance of the intended scalable system.
    *   **Hybrid Approach:** Proposed to start designing and implementing the scaling architecture while concurrently setting up foundational load testing tools and conducting lightweight tests on the single instance to gain initial insights and mature the testing framework.

## 4. Local Testing of Horizontal Scalability

*   **Methodology:** Proposed using Docker Compose to simulate a multi-instance environment locally. This setup would include:
    *   Multiple Event Ray application instances.
    *   An instance of the chosen message broker (e.g., Kafka).
    *   A load balancer (e.g., Nginx) to distribute requests.
*   **Verification:** This local setup would allow testing of the core event distribution logic across instances.
*   **Limitations:** Acknowledged that local testing has limitations regarding resource constraints and realistic network conditions but is invaluable for functional verification.

## 5. Responsibility for Scaling Operations

*   **Event Ray's Role:** The Event Ray application itself would not be responsible for initiating scale-up or scale-down actions.
*   **External Orchestration:** Scaling operations would be managed by orchestration platforms like Kubernetes (using Horizontal Pod Autoscaler), AWS Auto Scaling Groups, etc.
*   **Scaling Triggers:** These platforms would make scaling decisions based on metrics such as CPU/memory utilization per instance, active SSE connection counts, event throughput, message broker consumer lag, and request/event latency.
*   **Design Implications for Event Ray:** Must be designed to be stateless (as much as possible regarding client sessions, with events state in the broker), support graceful startup/shutdown, and expose necessary metrics.

## 6. Selection of a Shared Message Broker

*   **Key Requirements for Event Ray:** Publish-subscribe mechanism, event persistence, scalability, low latency, robust Rust client libraries, and efficient handling of numerous "rays" (event streams).
*   **Candidates Compared:**
    *   **Apache Kafka:** Extremely scalable, high throughput, durable, mature ecosystem, good Rust client (`rust-rdkafka`). Operationally more complex and resource-intensive.
    *   **Redis Streams:** Simpler operationally, low latency, persistent, good Rust client (`redis-rs`). Scalability might be less than Kafka for extreme loads; requires Redis Cluster for HA/scalability.
    *   **NATS JetStream:** High performance, aims for simpler operations than Kafka, persistent, good Rust client (`async-nats`). Newer than Kafka, smaller ecosystem.
*   **Initial User Leaning & Critical Discussion:**
    *   User initially leaned towards Kafka for its "future-proofing" and "production-ready" appeal.
    *   Critically discussed the implications: mandating Kafka as Event Ray's *internal* scaling backplane could impose a significant operational burden on users, potentially conflicting with goals of simplicity and ease of integration.
    *   Clarified that the `projectbrief.md` mention of Kafka ("gets events from... Kafka") refers to Event Ray *consuming from* external Kafka sources, not necessarily *using* Kafka internally. This opened up the choice for the internal backplane.
*   **Clustering for Reliability:** Acknowledged that Redis Streams (via Redis Cluster) also requires a clustered setup for production-grade reliability and scalability, similar to Kafka. The difference often lies in the perceived complexity and resource footprint of managing these clusters.

## 7. Architectural Pattern: Separate Ingestion Service

*   **Proposal:** To enhance separation of concerns and scalability, discussed splitting Event Ray into two main services:
    1.  **Ingestion Service:** Responsible for receiving events from all external sources (e.g., API POSTs, consuming from external Kafka topics, webhooks), validating them, and publishing them to the internal shared message broker.
    2.  **Event Ray SSE Service:** Consumes events from the internal shared message broker and manages SSE connections to clients, fanning out the events.
*   **Pros:** Clearer separation, independent scaling of ingestion and SSE tiers, increased resilience, simplified core SSE logic, flexibility in adding new event sources.
*   **Cons:** More services to manage, potential for a slight increase in latency due to an extra hop, increased deployment complexity if not handled well.
*   **Addressing Deployment Complexity:** The user suggested that deployment complexity for a multi-service setup could be effectively managed using tools like Docker Compose, well-written deployment scripts, and Kubernetes (e.g., Helm charts). This was agreed upon as a viable strategy to maintain ease of deployment despite a more complex internal architecture.

## 8. Project Context & Path Forward

*   **Personal Project & Learning Goals:** The user confirmed this is a personal project, allowing for a more ambitious scope focused on learning, including creating Helm charts and robust Docker Compose setups. This made the separate ingestion service model with Kafka more attractive.
*   **Solidified Architectural Vision:**
    *   **Core Services:** Dedicated Ingestion Service and Event Ray SSE Service.
    *   **Shared Message Broker:** Proceed with **Apache Kafka** as the initial choice, utilizing the `rust-rdkafka` client. Design with potential abstraction for other brokers in mind.
    *   **Deployment Strategy:** Containerize with Docker, use Docker Compose for local/dev, and aim for Helm charts for Kubernetes.
*   **High-Level Phased Implementation Plan:**
    1.  **Phase 1 (Kafka Integration & PoC):**
        *   Integrate Kafka producer into the (future) Ingestion Service.
        *   Integrate Kafka consumer into the (future) Event Ray SSE Service.
        *   Dockerize services and create a basic Docker Compose setup for multi-instance testing with Kafka, including a load balancer.
    2.  **Phase 2 (Load Testing Framework):**
        *   Select/develop load testing tools.
        *   Define and run basic load test scenarios against the Docker Compose environment.
    3.  **Phase 3 (Refinement & Advanced Deployment):**
        *   Optimize Kafka interactions.
        *   Conduct advanced load testing.
        *   Develop Helm charts.
        *   Explore autoscaling on Kubernetes.
*   **Immediate Key Decisions Pending:**
    1.  **Binary Structure:** Single vs. multiple Rust binaries for the services.
    2.  **Kafka Topic Strategy:** E.g., one topic per `ray_id`, sharded topics, or a single global topic.
    3.  **Configuration Management:** How services will receive configurations (e.g., Kafka broker details).

This summary captures the detailed planning and decisions made, setting a clear direction for enhancing Event Ray's scalability and testability.
