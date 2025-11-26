# OBD-II Telemetry Dashboard

A real-time automotive telemetry dashboard built with Tauri 2.0, Rust, and React. Features real OBD-II data playback, AI-powered driving analysis, and live car visualization.

## Features

### 🚗 Real Data Playback
- **3 Bundled Datasets**: City commute, highway driving, and mixed driving scenarios from a Toyota Etios 2014
- **27 OBD-II Parameters**: Engine RPM, vehicle speed, coolant temperature, throttle position, engine load, fuel level, and more
- **Adjustable Playback Speed**: 0.5x to 10x speed control for testing and demonstration
- **Smooth 10Hz Updates**: Real-time telemetry updates at 10 frames per second

### 📊 Live Dashboard
- **Interactive Gauges**: RPM, speed, and temperature with color-coded ranges
- **Telemetry Charts**: Real-time line charts showing historical data
- **CAN Frame Monitor**: Raw protocol-level message inspection
- **System Status**: Simulation state and connection indicators

### 🎨 Car Animation Widget
- **Animated SVG Car**: Wheels spin based on vehicle speed
- **Dynamic Colors**: Car changes color based on driving state (idle/normal/accelerating/high-speed)
- **Visual Effects**: Exhaust smoke, high-RPM shake, speed lines, headlights, and tail lights
- **Status Badges**: Real-time indication of engine state and driving mode

### 🤖 AI Analysis (Coming Soon)
- **Driving Coaches**: Fuel efficiency, safety, and diagnostic agents
- **Real-Time Insights**: Personalized recommendations based on driving patterns
- **OpenAI Integration**: Powered by GPT-4 for intelligent analysis

## Tech Stack

**Frontend:**
- React 18 + TypeScript
- Vite 6
- Zustand (state management)
- Recharts (charting)
- Tailwind CSS

**Backend:**
- Rust (Tauri 2.0)
- CSV parsing with `csv` crate
- Flexible dataset loading with automatic PID detection
- Dual-mode simulation (real data playback + random walk fallback)

## Quick Start

### Prerequisites
- Node.js 18+ and npm
- Rust 1.70+ and Cargo
- macOS, Linux, or Windows

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd carDataProject

# Install dependencies
npm install

# Run development server
npm run tauri dev
```

The app will open at `http://localhost:1420/`

### Building for Production

```bash
npm run tauri build
```

## Usage

### Starting the Simulation

1. Click the **"Start Engine"** button in the dashboard
2. The green **"REAL DATA PLAYBACK"** badge indicates you're using real OBD-II data
3. Select a dataset from the dropdown to switch between driving scenarios
4. Adjust playback speed using the speed control (0.5x - 10x)

### Available Datasets

| Dataset | Vehicle | Duration | Samples | Description |
|---------|---------|----------|---------|-------------|
| **City Commute** | Toyota Etios 2014 | 64 min | 3,853 | Stop-and-go city traffic |
| **Highway Driving** | Toyota Etios 2014 | 45 min | 2,709 | Sustained high-speed cruising |
| **Extended Mixed** | Toyota Etios 2014 | 59 min | 3,528 | Mixed city and highway driving |

*Source: [carOBD Dataset](https://github.com/eron93br/carOBD) (Open Source)*

### Dataset Format

Datasets are stored in `src-tauri/data/datasets/` as CSV files with the following structure:
- **Catalog**: `catalog.json` contains metadata for all datasets
- **CSV Files**: One row per second with 27 OBD-II parameters
- **Auto-Detection**: Column names are automatically mapped to PIDs

## Development

### Project Structure

```
carDataProject/
├── src/                    # React frontend
│   ├── components/         # UI components (Gauges, Charts, CarAnimation)
│   ├── pages/              # Dashboard and AI Analysis pages
│   ├── services/           # API client
│   └── store/              # Zustand state management
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── agents/         # AI analysis agents
│   │   ├── simulation/     # Dataset loading and playback engine
│   │   ├── storage/        # CSV recording
│   │   └── commands.rs     # Tauri commands
│   └── data/datasets/      # Bundled OBD-II datasets
├── specs/                  # Feature specifications (speckit)
└── test/                   # Unit and integration tests
```

### Running Tests

```bash
# Rust backend tests
cd src-tauri
cargo test

# Test dataset loading
cargo test test_all_datasets_load -- --nocapture

# Frontend tests (coming soon)
npm test
```

### Adding New Datasets

1. Place CSV file in `src-tauri/data/datasets/<category>/`
2. Update `catalog.json` with dataset metadata
3. Ensure column names match expected PID format (e.g., `ENGINE_RPM ()`)
4. Restart the app to load the new dataset

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
