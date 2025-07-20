# AI Assistant: Standard Task Initiation Workflow

This document outlines the standard operating procedure for the AI assistant.

## General Workflow for Task Initiation

The assistant should follow these steps to ensure a thorough understanding of both the overall project and the specific task requirements:

0.  **Understanding Project Principles**:
    *   The assistant must first read `ai_assistant_guides/principles.md`. This file outlines the core guiding principles for the Event Ray project and should inform all subsequent understanding and actions.
    *   **Confirmation**: After reading, the assistant should confirm with the user that the principles have been read and processed.

1.  **Contextual Reading from `memory-bank/`**:
    *   **Initial File**: After understanding the principles, the assistant should read `memory-bank/projectbrief.md`. This file provides the foundational understanding of the project's purpose and high-level goals.
    *   **Subsequent Files**: After `projectbrief.md`, the assistant must read all other files present in the `memory-bank/` directory, **except for `currenttask.md`**. This includes files like `architecture.md`, `project_structure.md`, `previoustasks.md`, etc. This step is crucial for building comprehensive background knowledge.
    *   **Confirmation**: After reading each file (or a logical group of files), the assistant should confirm with the user that the file(s) have been read and processed.

2.  **Addressing `currenttask.md`**:
    *   **Do Not Read Prematurely**: The `memory-bank/currenttask.md` file should **not** be read during the initial contextual file reading phase.
    *   **Explicit Inquiry**: Once all other specified files in `memory-bank/` have been read and acknowledged, the assistant must then explicitly ask the user for permission to read `memory-bank/currenttask.md`.

3.  **Understanding `currenttask.md`**:
    *   The `memory-bank/currenttask.md` file contains the specific details for the task at hand. It typically includes:
        *   An **Objective**: Clearly stating what the task aims to achieve.
        *   **Implementation Plan/Steps**: A detailed breakdown of actions to be taken.
    *   By reading this file *after* absorbing the general project context, the assistant can more effectively understand and execute the specific instructions.

Adherence to this workflow is key for efficient and accurate task completion.

## Task Types: Planning and Implementation

Tasks assigned to the assistant can generally be categorized into two types:

1.  **Planning Tasks**:
    *   In a planning task, the assistant's primary role is to collaborate with the user to define or refine the objective and steps outlined in `memory-bank/currenttask.md`.
    *   This involves a discussion with the user about the current task's details.
    *   **Process for Planning:**
        1.  **Objective First**: The overall objective of the task in `currenttask.md` should be discussed and finalized with the user first.
        2.  **Step-by-Step Discussion**: Each implementation step within `currenttask.md` should then be discussed individually.
        3.  **User-Prompted Updates**: Modifications or additions to `currenttask.md` (for the objective or steps) should only be made after the assistant has discussed the proposed changes and the user explicitly prompts or approves the update to the file.

2.  **Implementation Tasks**:
    *   In an implementation task, the assistant actively works to execute the steps detailed in `memory-bank/currenttask.md`, using available tools and capabilities.
    *   Planning for these tasks should ideally be completed before switching to implementation mode.
    *   Each step outlined in `currenttask.md` must be executed individually; multiple steps should not be combined into a single action.
    *   After completing each step, the project should compile successfully (e.g., via `cargo check` or `cargo build`). Compiler warnings are permissible at these intermediate stages.
    *   Upon completion of all steps in `currenttask.md`, the following final state requirements must be met:
        *   The project must compile without any compiler warnings.
        *   The project must be free of `cargo clippy` warnings (e.g., run `cargo clippy -- -D warnings`).
        *   All automated tests (e.g., `cargo test`) must pass.
    *   Once the entire task is successfully completed and verified, an entry summarizing the task, its key changes, and impact must be added to `memory-bank/previoustasks.md`, following the established reverse chronological order.
    *   After task completion and logging, the assistant should, with the user's permission, evaluate if any other files in the `memory-bank/` directory (e.g., `architecture.md`, `project_structure.md`) require updates based on the changes made during the task.
