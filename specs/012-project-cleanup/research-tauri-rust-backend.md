# Tauri 2.0 Rust Backend - Best Practices (2024)

## Project Structure

### Tauri 2.0 Architecture

A Tauri project consists of two parts:
1. **JavaScript/TypeScript frontend** (top level)
2. **Rust backend** (inside `src-tauri/` folder)

### Rust Backend Structure (`src-tauri/`)

```
src-tauri/
├── src/
│   ├── lib.rs              # Library entry point (mobile compatible)
│   ├── main.rs             # Desktop entry point (calls lib.rs)
│   ├── commands.rs         # Tauri command definitions
│   ├── state.rs            # Application state management
│   ├── models.rs           # Data models and types
│   ├── simulation/         # Feature modules
│   │   ├── mod.rs
│   │   ├── state.rs
│   │   └── playback.rs
│   ├── storage/            # Storage layer
│   │   ├── mod.rs
│   │   └── supabase.rs
│   └── bin/                # Utility binaries
│       └── import_data.rs
├── capabilities/           # Tauri capability definitions
├── icons/                  # Application icons
├── Cargo.toml             # Rust dependencies
└── tauri.conf.json        # Tauri configuration
```

### Key Files

**lib.rs** - Mobile-compatible entry point:
```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Setup code
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_simulation,
            commands::stop_simulation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**main.rs** - Desktop entry point:
```rust
fn main() {
    app_lib::run()
}
```

## Module Organization Best Practices

### Module Visibility

Use appropriate visibility modifiers:

```rust
// Public API
pub struct SimulationState { }

// Public within crate only
pub(crate) fn internal_helper() { }

// Private to module
fn private_impl() { }
```

### Feature-Based Organization

Organize by domain/feature, not by type:

```
✅ Good:
src/
├── simulation/
│   ├── mod.rs (exports)
│   ├── state.rs
│   ├── playback.rs
│   └── dataset.rs
└── storage/
    ├── mod.rs
    └── supabase.rs

❌ Avoid:
src/
├── models.rs (all models)
├── services.rs (all services)
└── utils.rs (everything else)
```

## Tauri Command Patterns

### Command Organization

Separate commands by domain:

```rust
// commands.rs or commands/simulation.rs
#[tauri::command]
pub fn start_simulation(
    state: State<'_, Arc<SimulationState>>,
    session_name: Option<String>
) -> Result<(), String> {
    // Implementation
    Ok(())
}

#[tauri::command]
pub async fn get_session_data(
    state: State<'_, Arc<SimulationState>>,
    session_id: String
) -> Result<SessionData, String> {
    // Async implementation
    Ok(data)
}
```

### Error Handling in Commands

Return `Result<T, String>` for simple errors, or use custom error types:

```rust
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommandError {
    message: String,
    code: String,
}

#[tauri::command]
pub async fn risky_operation() -> Result<Data, CommandError> {
    // ...
}
```

### State Management

Use `State` for shared state, wrap with `Arc<Mutex<T>>` for interior mutability:

```rust
use std::sync::{Arc, Mutex};
use tauri::State;

pub struct AppState {
    pub counter: Mutex<i32>,
    pub config: Mutex<Config>,
}

#[tauri::command]
pub fn increment(state: State<'_, Arc<AppState>>) -> i32 {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    *counter
}
```

## Error Handling

### Using `thiserror` and `anyhow`

**thiserror** for library errors (exposed to users):
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database connection failed: {0}")]
    ConnectionError(String),

    #[error("Session not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
```

**anyhow** for application errors (internal):
```rust
use anyhow::{Context, Result};

pub async fn load_session(id: &str) -> Result<Session> {
    let data = fetch_from_db(id)
        .await
        .context("Failed to fetch session from database")?;

    Ok(parse_session(data)?)
}
```

## Async Patterns

### Using Tokio

```rust
// Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }

// Code
use tokio::time::{sleep, Duration};

#[tauri::command]
pub async fn delayed_operation() -> String {
    sleep(Duration::from_secs(2)).await;
    "Done".to_string()
}
```

### Spawning Background Tasks

```rust
use tauri::AppHandle;
use tokio::spawn;

pub async fn start_background_task(app: AppHandle) {
    spawn(async move {
        loop {
            // Background work
            tokio::time::sleep(Duration::from_secs(1)).await;

            // Emit events to frontend
            app.emit("update", payload).ok();
        }
    });
}
```

## Testing Best Practices

### Unit Test Organization

Use `#[cfg(test)]` modules:

```rust
// simulation/state.rs
pub struct SimulationState {
    // ...
}

impl SimulationState {
    pub fn new() -> Self { /* ... */ }

    pub fn update(&mut self, value: f64) { /* ... */ }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let state = SimulationState::new();
        assert_eq!(state.rpm(), 0.0);
    }

    #[test]
    fn test_update() {
        let mut state = SimulationState::new();
        state.update(100.0);
        assert_eq!(state.rpm(), 100.0);
    }
}
```

### Testing with Mockall

For mocking traits and dependencies:

```rust
use mockall::{automock, predicate::*};

#[automock]
pub trait DataStore {
    fn save(&self, data: &str) -> Result<(), Error>;
    fn load(&self, id: &str) -> Result<String, Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_mock() {
        let mut mock = MockDataStore::new();
        mock.expect_save()
            .with(eq("test"))
            .times(1)
            .returning(|_| Ok(()));

        // Test code using mock
        let result = mock.save("test");
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Create `tests/` directory at crate root (sibling to `src/`):

```rust
// tests/integration_test.rs
use my_app::{SimulationState, start_simulation};

#[test]
fn test_simulation_lifecycle() {
    let state = SimulationState::new();
    // Test the full workflow
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_simulation_lifecycle

# Run with output
cargo test -- --nocapture

# Run tests for specific package
cargo test -p tauri-app
```

## Code Quality

### Clippy Configuration

**Cargo.toml:**
```toml
[lints.clippy]
# Enable pedantic lints
pedantic = "warn"

# Disable specific pedantic lints
module_name_repetitions = "allow"
missing_errors_doc = "allow"
```

### Running Clippy

```bash
# Run clippy
cargo clippy

# Fix automatically where possible
cargo clippy --fix

# Run with all features
cargo clippy --all-features
```

### Documentation Standards

Use doc comments for public APIs:

```rust
/// Starts the simulation with an optional session name.
///
/// # Arguments
///
/// * `session_name` - Optional name for the recording session
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error string if failed.
///
/// # Examples
///
/// ```
/// start_simulation(Some("Highway Test".to_string()));
/// ```
pub fn start_simulation(session_name: Option<String>) -> Result<(), String> {
    // Implementation
    Ok(())
}
```

## Security Best Practices

### 1. API Keys and Secrets

**Never expose secrets in frontend** - Handle API calls in Rust:

```rust
#[tauri::command]
pub async fn fetch_from_api() -> Result<Data, String> {
    let api_key = std::env::var("API_KEY")
        .expect("API_KEY must be set");

    // Make request with secret key
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.example.com/data")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // Return data to frontend
    Ok(response.json().await.map_err(|e| e.to_string())?)
}
```

### 2. Capability Configuration

Explicitly allow commands in `capabilities/`:

```json
{
  "permissions": [
    "core:default",
    "shell:allow-execute"
  ]
}
```

### 3. Input Validation

Always validate input from frontend:

```rust
#[tauri::command]
pub fn process_input(user_input: String) -> Result<(), String> {
    if user_input.is_empty() {
        return Err("Input cannot be empty".to_string());
    }

    if user_input.len() > 1000 {
        return Err("Input too long".to_string());
    }

    // Process validated input
    Ok(())
}
```

## Performance Patterns

### Avoid Blocking Main Thread

Use async or spawn for heavy operations:

```rust
use tokio::task::spawn_blocking;

#[tauri::command]
pub async fn cpu_intensive_task() -> Result<String, String> {
    spawn_blocking(|| {
        // CPU-intensive work
        heavy_computation()
    })
    .await
    .map_err(|e| e.to_string())
}
```

### Efficient State Access

Minimize lock duration:

```rust
// ❌ Bad: Long lock duration
let mut state = app_state.lock().unwrap();
let result = expensive_computation(&state);
state.value = result;

// ✅ Good: Short lock duration
let input = {
    let state = app_state.lock().unwrap();
    state.input.clone()
};
let result = expensive_computation(&input);
{
    let mut state = app_state.lock().unwrap();
    state.value = result;
}
```

## Application to Current Project

### Current Structure Assessment

**Good:**
- ✅ Feature-based modules (`simulation/`, `storage/`)
- ✅ Separate `commands.rs`
- ✅ Using `Arc<Mutex<T>>` for state
- ✅ Async Supabase operations

**Improvements Needed:**
- ❌ 46 compiler warnings (unused code)
- ❌ No unit tests yet
- ❌ Entire `agents/` module unused
- ❌ Config module never constructed
- ❌ Error handling could use `thiserror`

### Recommended Changes

1. **Remove unused modules**: Delete `agents/`, `config.rs` if not needed
2. **Add unit tests**: Start with `simulation/state.rs`, `storage/supabase.rs`
3. **Improve error handling**: Use `thiserror` for custom errors
4. **Add documentation**: Doc comments for public APIs
5. **Run clippy**: Fix all clippy warnings

## References

- [Tauri 2.0 Project Structure (Official)](https://v2.tauri.app/start/project-structure/)
- [Learn Tauri By Doing - Part 1 (DEV)](https://dev.to/giuliano1993/learn-tauri-by-doing-part-1-introduction-and-structure-1gde)
- [Tauri Best Practices and Coding Standards (Project Rules)](https://www.projectrules.ai/rules/tauri)
- [Rust Unit Testing with cargo test (Software Patterns Lexicon)](https://softwarepatternslexicon.com/patterns-rust/22/2/)
- [Complete Guide To Testing Code In Rust (Zero To Mastery)](https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/)
- [Rust Testing with Mocks and Stubs: mockall (Software Patterns Lexicon)](https://softwarepatternslexicon.com/patterns-rust/22/5/)
- [Test Organization - The Rust Programming Language](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
