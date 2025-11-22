# Tasks: Rust Car Data Project

**Input**: Design documents from `/specs/`
**Prerequisites**: plan.md (required), system-spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `src-tauri/src/`
- **Frontend**: `src/`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create Tauri project structure using `npm create tauri-app`
- [x] T002 [P] Install Rust dependencies (`tokio`, `serde`, `parquet`, `chrono`) in `src-tauri/Cargo.toml`
- [x] T003 [P] Install Frontend dependencies (`recharts`, `lucide-react`, `axios`) in `package.json`
- [x] T004 Configure Tauri permissions and capabilities in `src-tauri/tauri.conf.json`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Create `CanFrame` and `ObdPid` structs in `src-tauri/src/models.rs`
- [x] T006 [P] Create `TelemetryPoint` struct in `src-tauri/src/models.rs`
- [x] T007 Setup basic Tauri command structure in `src-tauri/src/lib.rs` (or `main.rs` for v1)
- [x] T008 [P] Create frontend API client wrapper in `src/services/api.ts` matching `contracts/tauri-api.ts`
- [x] T009 Setup global state management (Context or Zustand) in `src/store/useStore.ts`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Simulation & Visualization (Priority: P1) 🎯 MVP

**Goal**: As a user, I want to see simulated car data (RPM, Speed) on a dashboard so I can verify the UI works without a real car.

**Independent Test**: Click "Start Engine", see RPM gauge move.

### Implementation for User Story 1

- [x] T010 [US1] Implement `SimulationState` struct to hold current RPM/Speed in `src-tauri/src/simulation/state.rs`
- [x] T011 [US1] Implement `run_simulation_loop` function in `src-tauri/src/simulation/mod.rs` to generate random CAN frames
- [x] T012 [US1] Implement `start_simulation` and `stop_simulation` Tauri commands in `src-tauri/src/commands.rs`
- [x] T013 [US1] Implement `telemetry-update` event emission in `src-tauri/src/simulation/mod.rs`
- [x] T014 [P] [US1] Create `Gauge` component in `src/components/Gauge.tsx`
- [x] T015 [P] [US1] Create `Dashboard` page in `src/components/Dashboard.tsx`
- [x] T016 [US1] Connect Dashboard to `telemetry-update` event in `src/components/Dashboard.tsx`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Data Recording (Priority: P2)

**Goal**: As a user, I want my driving session to be saved to a file so I can analyze it later.

**Independent Test**: Run simulation for 10s, check `data/` folder for new `.parquet` file.

### Implementation for User Story 2

- [ ] T017 [US2] Create `SessionManager` struct to handle file paths and IDs in `src-tauri/src/storage/manager.rs`
- [ ] T018 [US2] Implement Parquet writer logic in `src-tauri/src/storage/writer.rs`
- [ ] T019 [US2] Integrate storage writing into the simulation loop in `src-tauri/src/simulation/mod.rs`
- [ ] T020 [US2] Implement `get_session_history` Tauri command in `src-tauri/src/commands.rs`
- [ ] T021 [P] [US2] Create `SessionList` component in `src/components/SessionList.tsx`
- [ ] T022 [US2] Add "History" tab to main UI in `src/App.tsx`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - AI Analysis (Priority: P3)

**Goal**: As a user, I want to get an AI-generated summary of my driving session so I can understand my driving habits.

**Independent Test**: Select a past session, click "Analyze", see text summary (from Mock).

### Implementation for User Story 3

- [ ] T023 [US3] Create `AnalysisResult` struct in `src-tauri/src/models.rs`
- [ ] T024 [US3] Implement data aggregation logic (min/max/avg) in `src-tauri/src/analysis/aggregator.rs`
- [ ] T025 [US3] Implement `analyze_session` Tauri command (returning Mock data initially) in `src-tauri/src/commands.rs`
- [ ] T026 [P] [US3] Create `AnalysisView` component in `src/components/AnalysisView.tsx`
- [ ] T027 [US3] Integrate Analysis view with Session List in `src/components/SessionList.tsx`

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: User Story 4 - Real OBD-II Connection (Priority: P4)

**Goal**: As a user, I want to connect to a real car via ELM327 (USB/Serial) and see live data.

**Independent Test**: Connect ELM327 simulator or device, select serial port, see live RPM.

### Implementation for User Story 4

- [ ] T028 [US4] Add `tokio-serial` dependency to `src-tauri/Cargo.toml`
- [ ] T029 [US4] Create `SerialConnection` struct in `src-tauri/src/hardware/serial.rs`
- [ ] T030 [US4] Implement ELM327 initialization sequence (ATZ, ATSP0, etc.) in `src-tauri/src/hardware/elm327.rs`
- [ ] T031 [US4] Implement `list_serial_ports` Tauri command in `src-tauri/src/commands.rs`
- [ ] T032 [US4] Create `ConnectionSettings` component in `src/components/ConnectionSettings.tsx`
- [ ] T033 [US4] Update `simulation/mod.rs` to accept a "Source" trait (Simulation vs Real Hardware)

**Checkpoint**: App can now switch between "Virtual Car" and "Real Car" modes

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T034 [P] Update `README.md` with run instructions
- [ ] T035 Code cleanup: Run `cargo clippy` and `cargo fmt`
- [ ] T036 [P] Add error handling toasts in Frontend
- [ ] T037 Verify Parquet file compatibility with external tools (e.g. Python/Pandas)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2)
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Depends on Simulation loop existing (US1)
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Depends on Data Recording (US2)
- **User Story 4 (P4)**: Can start after Foundational (Phase 2) - Independent of US1/US2/US3, but easier to build after US1 (Simulation) is working

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- Frontend components (Gauge, Dashboard, SessionList) can be built in parallel with Backend logic
- Analysis logic (US3) can be developed independently of the Simulation logic (US1) if using mock data

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
