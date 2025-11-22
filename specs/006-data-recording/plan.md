# Feature Plan: Data Recording (Parquet)

## 1. Goal
Implement **User Story 2**: "As a user, I want my driving session to be saved to a file so I can analyze it later."
Enable the application to persist telemetry data to local storage using the Apache Parquet format for efficiency and compatibility.

## 2. Scope
-   **Backend**:
    -   Implement `SessionManager` to handle file paths and session lifecycle.
    -   Implement `ParquetWriter` to write `TelemetryPoint` structs to disk.
    -   Integrate writing into the main simulation loop.
    -   Add a Tauri command `get_session_history` to list past sessions.
    -   **Import Feature**: Implement `import_session` command to convert external CSV/JSON logs into the app's Parquet format for AI analysis.
-   **Frontend**:
    -   Create a `SessionList` component to view past recordings.
    -   Add a "History" page (or tab) to the UI.
    -   Add "Import" button to upload external logs.

## 3. Tasks
1.  [x] **Storage Logic**:
    -   Create `src-tauri/src/storage/mod.rs`, `manager.rs`, `writer.rs`.
    -   Implement schema definition and row writing.
2.  [ ] **Integration**:
    -   Initialize `SessionManager` in `lib.rs`.
    -   Update `simulation/mod.rs` to write data points when simulation is running.
3.  [ ] **Import Logic**:
    -   Add `csv` dependency to `src-tauri/Cargo.toml`.
    -   Implement `import_from_csv` in `storage/manager.rs`.
    -   Expose `import_file` Tauri command.
4.  [ ] **API**:
    -   Implement `get_session_history` command.
5.  [ ] **UI**:
    -   Create `HistoryPage.tsx`.
    -   Add route `/history`.
    -   Display list of parquet files with metadata (timestamp, size).
    -   Implement Import file picker.

## 4. Constitution Check
-   **Performance**: Parquet writing should happen in the background or be buffered to avoid blocking the simulation loop.
-   **Data Integrity**: Ensure files are properly closed when simulation stops.
-   **Schema**: Use the defined `TelemetryPoint` schema (Timestamp, PID, Value, Unit).
