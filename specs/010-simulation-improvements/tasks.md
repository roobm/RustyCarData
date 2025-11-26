# Implementation Tasks: Real OBD-II Data Integration

**Feature**: 010-simulation-improvements | **Branch**: `010-simulation-improvements`
**Generated**: 2025-11-26

This document provides a task breakdown for implementing real OBD-II data playback with Supabase storage.

---

## Phase 1: Foundation & Supabase Setup

**Objective**: Set up infrastructure for Supabase integration and prepare for data playback

- [ ] **T001** [P1] [Setup] Add Supabase dependencies to Cargo.toml
      Dependencies: uuid, postgrest, dotenv
      File: src-tauri/Cargo.toml

- [ ] **T002** [P1] [Setup] Create .env.example file with Supabase configuration
      File: src-tauri/.env.example
      Variables: SUPABASE_URL, SUPABASE_ANON_KEY

- [ ] **T003** [P1] [Schema] Create Supabase migration SQL file
      File: src-tauri/migrations/001_initial_schema.sql
      Tables: sessions, telemetry_points, session_stats

- [ ] **T004** [P1] [Code] Create Supabase client module
      File: src-tauri/src/storage/supabase.rs
      Implements: SupabaseClient struct with CRUD operations

- [ ] **T005** [P1] [Code] Update storage/mod.rs to export supabase module
      File: src-tauri/src/storage/mod.rs

- [ ] **T006** [P1] [Code] Create configuration loader for environment variables
      File: src-tauri/src/config.rs
      Loads: SUPABASE_URL, SUPABASE_ANON_KEY from .env

---

## Phase 2: Dataset Integration

**Objective**: Download and prepare real OBD-II datasets for bundling

- [ ] **T007** [P1] [Data] Create datasets directory
      Directory: src-tauri/data/datasets/

- [ ] **T008** [P1] [Data] Download carOBD dataset (Toyota Etios)
      Source: https://github.com/eron93br/carOBD
      Target: src-tauri/data/datasets/city_commute/
      PIDs: 27 parameters

- [ ] **T009** [P1] [Data] Download Kaggle OBD-II highway dataset
      Source: https://www.kaggle.com/datasets/anwarmehmoodsohail/driving-datasets-obd-iican-bus
      Target: src-tauri/data/datasets/highway/
      Note: Requires Kaggle account

- [ ] **T010** [P2] [Data] Download KIT Automotive mixed driving dataset
      Source: https://publikationen.bibliothek.kit.edu/1000085073
      Target: src-tauri/data/datasets/mixed/

- [ ] **T011** [P2] [Data] Create dataset catalog/registry
      File: src-tauri/data/datasets/catalog.json
      Schema: Dataset metadata (vehicle info, PIDs, duration, style)

- [ ] **T012** [P1] [Code] Create dataset loader module
      File: src-tauri/src/simulation/dataset.rs
      Functions: load_csv(), parse_dataset(), validate_data()

- [ ] **T013** [P1] [Code] Implement column auto-detection
      File: src-tauri/src/simulation/dataset.rs
      Detects: rpm, speed, coolant_temp, throttle, etc.

---

## Phase 3: Playback Engine

**Objective**: Replace simulation with real data playback

- [ ] **T014** [P1] [Code] Create playback module
      File: src-tauri/src/simulation/playback.rs
      Struct: DatasetPlayback with play/pause/speed control

- [ ] **T015** [P1] [Code] Implement playback state machine
      File: src-tauri/src/simulation/playback.rs
      States: Idle, Playing, Paused, Ended

- [ ] **T016** [P1] [Code] Update SimulationState with new PIDs
      File: src-tauri/src/simulation/state.rs
      Add: throttle_position, fuel_rate, acceleration, brake_pressure, etc.

- [ ] **T017** [P1] [Code] Refactor simulation loop for playback
      File: src-tauri/src/simulation/mod.rs
      Replace: Random walk physics with dataset playback

- [ ] **T018** [P1] [Code] Implement playback controls
      File: src-tauri/src/commands.rs
      Commands: start_playback(), pause_playback(), set_playback_speed()

- [ ] **T019** [P1] [Code] Handle dataset end (loop or stop)
      File: src-tauri/src/simulation/playback.rs
      Options: Loop, stop, trigger event

- [ ] **T020** [P1] [Test] Test playback with city commute dataset
      Validate: Correct PID values, 10Hz update rate, smooth playback

---

## Phase 4: Supabase Integration

**Objective**: Store telemetry and sessions in Supabase cloud database

- [ ] **T021** [P1] [Infra] Create Supabase project
      Platform: https://supabase.com
      Tier: Free (up to 500MB)

- [ ] **T022** [P1] [Infra] Run migration SQL on Supabase
      File: migrations/001_initial_schema.sql
      Tables: Create sessions, telemetry_points, session_stats

- [ ] **T023** [P1] [Config] Add Supabase URL and API key to .env
      Variables: SUPABASE_URL, SUPABASE_ANON_KEY
      Note: Don't commit .env file

- [ ] **T024** [P1] [Code] Implement session creation in Supabase
      File: src-tauri/src/storage/supabase.rs
      Method: create_session()

- [ ] **T025** [P1] [Code] Implement batch telemetry upload
      File: src-tauri/src/storage/supabase.rs
      Method: insert_telemetry_batch()
      Batch size: 100 points per request

- [ ] **T026** [P1] [Code] Implement session stats storage
      File: src-tauri/src/storage/supabase.rs
      Method: update_session_stats()

- [ ] **T027** [P1] [Code] Implement session retrieval
      File: src-tauri/src/storage/supabase.rs
      Methods: get_session(), list_sessions()

- [ ] **T028** [P1] [Code] Update simulation loop to write to Supabase
      File: src-tauri/src/simulation/mod.rs
      Add: Parallel writes to Supabase + local CSV

- [ ] **T029** [P1] [Test] Validate Supabase writes (check table contents)

---

## Phase 5: CSV Import

**Objective**: Allow users to import their own OBD-II data

- [ ] **T030** [P2] [Code] Enhance import_csv() for user data
      File: src-tauri/src/storage/manager.rs
      Add: Column mapping dialog, validation

- [ ] **T031** [P2] [Code] Create import command
      File: src-tauri/src/commands.rs
      Command: import_user_csv(path: String)

- [ ] **T032** [P2] [Code] Implement data validation
      File: src-tauri/src/simulation/dataset.rs
      Check: PID ranges, timestamp monotonicity, missing values

- [ ] **T033** [P2] [Code] Store imported sessions in Supabase
      Mark dataset_source as 'imported'

- [ ] **T034** [P2] [Test] Test import with various CSV formats
      Formats: Torque, OBD Fusion, BMWLogger

---

## Phase 6: Frontend Integration

**Objective**: Update UI for dataset selection, playback controls, and import

- [ ] **T035** [P2] [UI] Create dataset selection dropdown
      File: src/components/DatasetSelector.tsx
      Lists: Bundled datasets from catalog.json

- [ ] **T036** [P2] [UI] Create playback controls component
      File: src/components/PlaybackControls.tsx
      Controls: Play/Pause, Speed (1x/2x/5x), Progress bar

- [ ] **T037** [P2] [UI] Add import CSV button
      File: src/pages/DashboardPage.tsx
      Opens: File dialog, calls import_user_csv()

- [ ] **T038** [P2] [UI] Create session history view
      File: src/components/SessionHistory.tsx
      Fetches: Sessions from Supabase, displays list

- [ ] **T039** [P2] [Code] Update telemetry store for new PIDs
      File: src/store/useStore.ts
      Add: throttle, fuelRate, acceleration, etc.

- [ ] **T040** [P2] [UI] Update dashboard to display new PIDs
      File: src/pages/DashboardPage.tsx
      Add: Gauges/charts for throttle, fuel rate, etc.

---

## Phase 7: Car Visualization Widget

**Objective**: Fun animated car widget showing driving state

- [ ] **T041** [P3] [Design] Create car sprite SVG
      File: src/assets/car-sprite.svg
      Size: 100x50px, side view

- [ ] **T042** [P3] [Design] Create speed line graphics
      File: src/assets/speed-lines.svg

- [ ] **T043** [P3] [UI] Create CarAnimation component
      File: src/components/CarAnimation.tsx
      Framework: React Spring or Framer Motion

- [ ] **T044** [P3] [Code] Implement animation state machine
      States: Idle, Accelerating, Cruising, Braking, HighRPM

- [ ] **T045** [P3] [Code] Bind telemetry data to animations
      Subscribe to: speed, rpm, acceleration from store

- [ ] **T046** [P3] [UI] Add toggle control for widget
      Position: Bottom-right corner, draggable

- [ ] **T047** [P3] [Polish] Test at different playback speeds
      Speeds: 1x, 2x, 5x

---

## Phase 8: Testing & Polish

**Objective**: Ensure quality and reliability

- [ ] **T048** [P1] [Test] Test playback with all 4 bundled datasets
      Validate: Correct playback, no crashes, realistic values

- [ ] **T049** [P1] [Test] Test Supabase integration under load
      Load: 1000+ telemetry points, multiple sessions

- [ ] **T050** [P1] [Test] Test CSV import with edge cases
      Cases: Missing columns, invalid values, large files

- [ ] **T051** [P2] [Test] Verify agent analysis on real data
      Check: Scores differ between city/highway/aggressive

- [ ] **T052** [P2] [Polish] Add error handling and user feedback
      Messages: Dataset load failures, Supabase errors

- [ ] **T053** [P2] [Polish] Add loading indicators
      Components: Dataset loading, import progress

- [ ] **T054** [P2] [Docs] Update README with Supabase setup instructions

---

## Summary

- **Total Tasks**: 54
- **Completed**: 0
- **Priority 1 (Critical)**: 35 tasks
- **Priority 2 (Important)**: 16 tasks
- **Priority 3 (Nice-to-have)**: 7 tasks

## Execution Order

Tasks should be executed in phase order:
1. **Phase 1** (T001-T006): Foundation - 2-3 hours
2. **Phase 2** (T007-T013): Datasets - 2-3 hours
3. **Phase 3** (T014-T020): Playback - 3-4 hours
4. **Phase 4** (T021-T029): Supabase - 3-4 hours
5. **Phase 5** (T030-T034): Import - 2-3 hours
6. **Phase 6** (T035-T040): Frontend - 2-3 hours
7. **Phase 7** (T041-T047): Car Widget - 2-3 hours
8. **Phase 8** (T048-T054): Testing - 2-3 hours

**Total Estimated Time**: 18-24 hours

## Critical Path

The following tasks are blockers and must be completed first:
1. T001-T006 (Foundation)
2. T021-T023 (Supabase project setup)
3. T007-T013 (Dataset download and loading)
4. T014-T020 (Playback engine)

All other phases can proceed in parallel after Phase 1-3 are complete.

## Dependencies

**External:**
- Supabase account (free tier)
- Kaggle account (for dataset download)
- Dataset licenses verified

**Internal:**
- Rust dependencies installed (T001)
- Supabase tables created (T022)
- At least 1 dataset available (T008)

## Success Criteria

✅ **Minimum Viable:**
1. 3+ real datasets bundled and playable
2. Playback engine works smoothly at 10Hz
3. Data stored in both Supabase and local CSV
4. Can import user's CSV files
5. Agents analyze real data meaningfully

✅ **Quality Metrics:**
- Playback feels realistic (not random)
- Supabase queries complete < 500ms
- Import succeeds for 90%+ common formats
- Car animation runs at 60fps
- No data loss during playback

## Risk Mitigation

| Task | Risk | Mitigation |
|------|------|------------|
| T008-T010 | Dataset licenses | Use only verified open-source datasets |
| T021 | Supabase costs | Monitor usage, use free tier limits |
| T025 | Large batch uploads | Implement retry logic, chunking |
| T030 | CSV format variations | Extensive testing, fallback to manual mapping |
| T041-T047 | Performance issues | Profile animations, optimize rendering |

## Next Steps

After feature completion:
- Merge to main
- Create demo video showing real data playback
- Document dataset sources in README
- Consider Phase 9: Real-time OBD-II hardware connection
