# Quickstart

## Prerequisites
1. **Rust**: Install via `rustup` (https://rustup.rs/)
2. **Node.js**: Required for the frontend build (v18+)
3. **Tauri CLI**: `cargo install tauri-cli`

## Setup
1. Clone the repo.
2. Install frontend dependencies:
   ```bash
   cd src
   npm install
   ```
3. Run the app in development mode:
   ```bash
   cargo tauri dev
   ```

## Usage
1. The app will launch a desktop window.
2. Click **"Start Engine"** to begin the simulation.
3. You will see RPM and Speed gauges reacting to the simulated "virtual driver".
4. Data is automatically recorded to `data/session_[timestamp].parquet`.
5. Click **"Analyze"** to send the session summary to the AI (requires API Key).
