# Research: Rust Car Data Project

## Decisions

### 1. UI Framework: Tauri
- **Decision**: Use Tauri with React/TypeScript.
- **Rationale**: Allows for a modern, flexible UI (Web Tech) while keeping the backend logic in high-performance Rust. The user is comfortable with Web UI.
- **Alternatives**: Iced, Egui (Pure Rust). Rejected because Web UI offers better styling/charting libraries (e.g., Recharts, D3) out of the box.

### 2. Data Storage: Parquet
- **Decision**: Use Apache Parquet format.
- **Rationale**: Highly efficient columnar storage, perfect for time-series sensor data. User is familiar with it. Excellent ecosystem support in Rust (`parquet`, `polars`).
- **Alternatives**: SQLite (good but row-based, larger file size), CSV (slow, no types).

### 3. CAN/OBD Simulation
- **Decision**: Simulate CAN frames at the byte level.
- **Rationale**: User wants to learn "deep" CAN bus details.
- **Implementation**:
    - Create a "Virtual Car" module in Rust that generates raw CAN frames (ID + 8 bytes data).
    - Simulate standard OBD-II PIDs (RPM, Speed) within these frames.
    - On macOS, we can't use native `SocketCAN`, so we will build a software-only simulation layer that the application reads from (as if it were a serial port or socket).

### 4. AI Analysis
- **Decision**: External API (OpenAI/Anthropic) + Local Pre-processing.
- **Rationale**: Easiest way to get "advanced" analysis without training custom models yet.
- **Flow**:
    1. Collect data session (e.g., 10 mins drive).
    2. Rust backend aggregates/summarizes data (Min/Max/Avg/Anomalies).
    3. Send summary JSON to AI API with prompt: "Analyze this driving session...".
    4. Display text/insight result in UI.

## Unknowns & Research Tasks

### 1. CAN Frame Structure for OBD-II
- **Task**: Research the exact CAN frame format for standard OBD-II requests/responses (Service 01).
- **Goal**: Correctly simulate the byte sequence for "Request RPM" and "Response RPM".

### 2. Tauri + Async Rust Loop
- **Task**: Best practice for running a high-frequency (10ms) simulation loop alongside the Tauri event loop.
- **Goal**: Ensure UI doesn't freeze while processing 100s of CAN frames/sec.

### 3. Parquet Writing in Rust
- **Task**: Verify `parquet` crate usage for appending data in real-time vs batch writing.
- **Goal**: Avoid data loss if app crashes. Likely need to buffer in memory and flush every N seconds.
