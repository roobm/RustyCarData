# Implementation Plan: Rust Car Data Project

**Branch**: `001-rust-car-data` | **Date**: 2025-11-22 | **Spec**: [specs/system-spec.md](specs/system-spec.md)
**Input**: Feature specification from `/specs/system-spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

A Rust-based desktop application to read, simulate, and analyze OBD-II car data. The project starts with data simulation and aims to support real hardware (ELM327 etc.) in the future. It features an advanced UI for visualization and uses AI for data analysis.

## Technical Context

**Language/Version**: Rust (Latest Stable)
**Primary Dependencies**: 
- Async Runtime: `tokio`
- GUI Framework: `Tauri` (v2 if stable, else v1) + React/TypeScript (Web UI)
- OBD-II/CAN: `socketcan` (linux/sim), `tokio-serial` (ELM327), custom CAN frame parser
- AI/ML: Mock API initially (for UI dev), External API (OpenAI/Anthropic) future
**Storage**: `Parquet` (via `parquet` crate or `polars`)
**Testing**: `cargo test`
**Target Platform**: Desktop (macOS dev, Linux/Windows target)
**Project Type**: Desktop Application (Tauri)
**Performance Goals**: Real-time CAN frame processing, 60fps UI
**Constraints**: macOS dev environment requires CAN simulation (no native CAN), Offline data collection
**Scale/Scope**: Single user, local analysis initially.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Branch Management**: Feature branch `001-rust-car-data` created.
- [x] **Documentation**: Plan and Spec files initialized.
- [x] **Code Quality**: Rust enforces strict typing; will use `clippy` and `rustfmt`.

## Project Structure

### Documentation (this feature)

```text
specs/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── ai-strategy.md       # AI Strategy
├── tasks.md             # Global tasks
└── 001-rust-car-data/   # Legacy feature specs
    └── contracts/
```

### Source Code (repository root)

```text
src-tauri/
├── src/
│   ├── main.rs          # Entry point
│   ├── commands.rs      # Tauri commands (Frontend <-> Backend)
│   ├── simulation/      # CAN/OBD simulation logic
│   ├── hardware/        # Real hardware interface (Serial/CAN)
│   ├── analysis/        # AI/Data analysis logic
│   └── storage/         # Parquet writing logic
├── Cargo.toml
└── tauri.conf.json

src/                     # Frontend (React + TypeScript)
├── components/
│   ├── Dashboard.tsx
│   └── AnalysisView.tsx
├── App.tsx
└── main.tsx
```

**Structure Decision**: Tauri App Structure (Rust Backend + Web Frontend). This separates the high-performance Rust logic (CAN/OBD, Storage) from the UI.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
