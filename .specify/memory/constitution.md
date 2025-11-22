<!--
Sync Impact Report:
- Version change: 1.0.0 -> 1.1.0
- Modified Principles:
  - III. Code Quality and Readability -> III. Rust Idioms and Code Quality (Expanded to include Rust-specific tooling and safety)
- Added Principles:
  - IV. Architecture and Modularity (New: Enforces separation of concerns and hardware abstraction)
  - V. Error Handling (New: Enforces typed errors and forbids unwrap/panic)
- Templates requiring updates:
  - .specify/templates/plan-template.md (✅ "Constitution Check" section will dynamically reflect these)
-->

# carDataProject Constitution

## Core Principles

### I. Branch Management
Always open new branch for features. Feature development must be isolated from the main branch to maintain stability.

### II. Documentation
Always update documentation to be up-to-date. Documentation must reflect current codebase state at all times.

### III. Rust Idioms and Code Quality
Focus on code quality, readability, and Rust safety.
- **Tooling**: Code must pass `cargo clippy` (no warnings) and be formatted with `cargo fmt`.
- **Safety**: Avoid `unwrap()` and `expect()` in production code; handle `Option` and `Result` explicitly.
- **Readability**: Code must be readable even for a Rust newbie. Prioritize clarity over cleverness.

### IV. Architecture and Modularity
Maintain strict separation of concerns to ensure testability and portability.
- **Tauri Separation**: Keep the Rust backend (business logic, hardware access) decoupled from the Frontend UI.
- **Hardware Abstraction**: Use Traits to abstract hardware interfaces (e.g., `ObdConnection` trait) to allow seamless switching between Simulation and Real Hardware.
- **Dependency Injection**: Inject dependencies (like the hardware trait) rather than hardcoding them, enabling easier unit testing.

### V. Error Handling
Use robust, typed error handling throughout the application.
- **Typed Errors**: Use custom Error enums (e.g., via `thiserror`) for library code to provide meaningful context.
- **Propagation**: Propagate errors up the stack using `Result<T, E>` and the `?` operator.
- **No Panics**: The application should never panic during normal operation. Catch and handle all potential failure modes.

## Governance

This constitution supersedes all other practices. All development work must comply with these core principles.

**Version**: 1.1.0 | **Ratified**: 2025-11-22 | **Last Amended**: 2025-11-22
