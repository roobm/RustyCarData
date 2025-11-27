# Tasks: Session History Viewer with Data Visualization

**Feature**: 011-session-history-viewer
**Branch**: `011-session-history-viewer`
**Spec**: [spec.md](./spec.md) | [Implementation Plan](./implementation-plan.md)

## Overview

This document provides a complete, dependency-ordered task breakdown for implementing the Session History Viewer feature. Following the user's request, this is divided into **two major phases**:

**Phase A (This Document)**: Basic Session History Viewer
- Session list display
- Session naming and editing
- Telemetry visualization with charts
- Zoom functionality
- Session deletion

**Phase B (Future Tasks Document)**: AI Integration
- Automatic insights generation
- Conversational "Ask AI" feature
- Predictive maintenance alerts
- (To be specified in separate tasks document)

## Task Summary

- **Total Tasks**: 35 tasks
- **Parallel Opportunities**: 18 tasks marked [P]
- **User Stories Covered**: US 011.1 → US 011.5 (US 011.6 Map View is future enhancement)
- **Estimated Completion**: 12-18 hours total

## Implementation Strategy

**MVP Scope**: US 011.1 + US 011.3 (Session List + Basic Charts)
- Provides immediate value: users can browse and visualize sessions
- Tests core infrastructure (Supabase integration, routing, chart rendering)
- Can be deployed independently

**Incremental Delivery**:
1. US 011.1: Session list (2-3 hours)
2. US 011.3: Telemetry charts (3-4 hours)
3. US 011.2: Session naming (1-2 hours)
4. US 011.4: Zoom functionality (2-3 hours)
5. US 011.5: Session deletion (1 hour)

## Dependency Graph

```
Phase 1 (Setup)
  ↓
Phase 2 (Foundational - Backend)
  ↓
  ├─→ US 011.1 (Session List) ────────────────┐
  ├─→ US 011.3 (Telemetry Charts) ────────────┤ [All Independent]
  ├─→ US 011.2 (Session Naming) ──────────────┤
  ├─→ US 011.4 (Zoom Functionality) ──────────┤
  └─→ US 011.5 (Session Deletion) ────────────┘
  ↓
Phase 7 (Polish & Integration)
```

**Note**: User stories US 011.1 through US 011.5 can be implemented in parallel after Phase 2 is complete. They have no dependencies on each other.

## Parallel Execution Opportunities

### By User Story

**US 011.1 (Session List)**:
- T008, T009, T010 can run in parallel (different files)

**US 011.3 (Telemetry Charts)**:
- T015, T016, T017 can run in parallel (different components)

**Backend Tasks**:
- T004, T005, T006 can run in parallel (different functions in same module)

**Frontend Pages**:
- SessionHistoryPage and SessionDetailPage can be developed in parallel

---

## Phase 1: Setup & Configuration

**Goal**: Initialize project structure and verify dependencies

- [x] T001 Verify branch is `011-session-history-viewer` and clean working tree
- [x] T002 Verify Supabase client already has `get_session_telemetry()` and `list_sessions()` methods in src-tauri/src/storage/supabase.rs
- [x] T003 Verify Recharts is installed in package.json (should already exist)

---

## Phase 2: Foundational - Backend Infrastructure

**Goal**: Implement core backend functionality needed by all user stories

**Note**: These tasks MUST be completed before starting any user story implementation.

### Supabase Client Extensions

- [x] T004 [P] Implement `update_session_name()` method in src-tauri/src/storage/supabase.rs
  ```rust
  pub async fn update_session_name(
      &self,
      session_id: Uuid,
      name: String,
  ) -> Result<(), Box<dyn std::error::Error>>
  ```
  - Use Supabase UPDATE query
  - Return error if session not found
  - Handle network failures gracefully

- [x] T005 [P] Implement `delete_session()` method in src-tauri/src/storage/supabase.rs
  ```rust
  pub async fn delete_session(
      &self,
      session_id: Uuid,
  ) -> Result<(), Box<dyn std::error::Error>>
  ```
  - DELETE FROM telemetry_points WHERE session_id = ?
  - DELETE FROM session_stats WHERE session_id = ?
  - DELETE FROM sessions WHERE id = ?
  - Use transaction if possible, fallback to sequential deletes
  - Handle cascade constraints

- [x] T006 [P] Implement `get_session_stats()` method in src-tauri/src/storage/supabase.rs
  ```rust
  pub async fn get_session_stats(
      &self,
      session_id: Uuid,
  ) -> Result<Option<SessionStatsRow>, Box<dyn std::error::Error>>
  ```
  - Query session_stats table
  - Return None if no stats calculated yet
  - Used for displaying session metadata

### Tauri Commands

- [x] T007 [P] Add Tauri commands to src-tauri/src/commands.rs:
  - `list_sessions(limit: Option<usize>) -> Result<Vec<SessionRow>, String>`
  - `get_session_data(session_id: String) -> Result<SessionData, String>`
  - `update_session_name(session_id: String, name: String) -> Result<(), String>`
  - `delete_session(session_id: String) -> Result<(), String>`
  - Register all commands in src-tauri/src/lib.rs `tauri::Builder`

---

## Phase 3: US 011.1 - View Session List with Vehicle Names

**Goal**: Users can see a list of all recorded sessions with metadata

**Independent Test Criteria**:
- [ ] Navigate to `/history` route
- [ ] Sessions load and display in list format
- [ ] Session metadata shown: name, date, duration, source type
- [ ] Sessions sorted by date (newest first)
- [ ] Empty state displays when no sessions exist
- [ ] Loading state shows while fetching

### TypeScript Types

- [x] T008 [P] [US1] Create src/types/session.ts with TypeScript interfaces:
  - `Session` interface (maps to SessionRow)
  - `TelemetryPoint` interface (maps to TelemetryRow)
  - `SessionStats` interface (maps to SessionStatsRow)
  - `SessionData` interface (combined structure)

### Pages

- [x] T009 [US1] Create src/pages/SessionHistoryPage.tsx:
  - Fetch sessions on mount using `invoke('list_sessions', { limit: 50 })`
  - Display SessionList component
  - Show loading skeleton while fetching
  - Show empty state if no sessions
  - Handle errors with error boundary

### Components

- [x] T010 [P] [US1] Create src/components/SessionList.tsx:
  - Props: `sessions: Session[], onSelectSession: (id: string) => void`
  - Render list of SessionCard components
  - Sort sessions by `created_at` descending
  - Responsive grid layout (Tailwind CSS)

- [x] T011 [P] [US1] Create src/components/SessionCard.tsx:
  - Props: `session: Session, onSelect: () => void`
  - Display session metadata:
    - Session name (or "Unnamed Session" if null)
    - Date/time formatted
    - Duration in minutes
    - Source type badge
    - Vehicle info if available
  - Click handler calls `onSelect()`
  - Hover effect, card styling with Tailwind

### Routing

- [x] T012 [US1] Add `/history` route to src/App.tsx:
  - `<Route path="/history" element={<SessionHistoryPage />} />`
  - Import SessionHistoryPage component

- [x] T013 [US1] Add "History" navigation link to sidebar/header:
  - Link to `/history` route
  - Icon: `<History />` from lucide-react
  - Label: "History"

---

## Phase 4: US 011.3 - Visualize All Telemetry Data

**Goal**: Users can view detailed telemetry charts for a session

**Independent Test Criteria**:
- [ ] Click a session in the list
- [ ] Navigate to `/history/:sessionId` detail page
- [ ] All available OBD-II parameters display as separate charts
- [ ] Charts stack vertically
- [ ] X-axis shows time, Y-axis shows parameter values
- [ ] Charts share synchronized X-axis
- [ ] Hover tooltips show exact values

### Pages

- [ ] T014 [US3] Create src/pages/SessionDetailPage.tsx:
  - Get `sessionId` from URL params (`useParams()`)
  - Fetch session data on mount: `invoke('get_session_data', { sessionId })`
  - Display session header (name, date, duration)
  - Render TelemetryCharts component with data
  - Back button to `/history`
  - Loading and error states

### Components

- [ ] T015 [P] [US3] Create src/components/TelemetryCharts.tsx:
  - Props: `telemetry: TelemetryPoint[], sessionName: string`
  - Transform telemetry data to chart-friendly format
  - Render ChartContainer for each OBD-II parameter:
    - engine_rpm (RPM)
    - vehicle_speed (km/h)
    - coolant_temp (°C)
    - throttle_position (%)
    - engine_load (%)
    - fuel_level (%)
    - intake_air_temp (°C)
    - timing_advance (°)
    - battery_voltage (V)
    - maf_flow_rate (g/s)
    - o2_sensor_voltage (V)
    - fuel_pressure (kPa)
  - Vertical stack layout
  - Only render charts for parameters with data (skip null values)

- [ ] T016 [P] [US3] Create src/components/ChartContainer.tsx:
  - Props: `data: ChartDataPoint[], parameterName: string, unit: string, color: string, syncId: string`
  - Render Recharts `<LineChart>` component
  - Configure axes (X: time, Y: value)
  - Add `<Tooltip />` for hover values
  - Add `<Line />` with specified color
  - Responsive container with fixed height (200px)
  - Parameter-specific formatting (decimals, units)

### Routing

- [ ] T017 [US3] Add `/history/:sessionId` route to src/App.tsx:
  - `<Route path="/history/:sessionId" element={<SessionDetailPage />} />`
  - Import SessionDetailPage component

---

## Phase 5: US 011.2 - Name and Organize Sessions

**Goal**: Users can name sessions and filter by name

**Independent Test Criteria**:
- [ ] Click "Edit" button on session card
- [ ] Inline input field appears
- [ ] Type new session name (e.g., "Roope's BMW M3")
- [ ] Press Enter or click away to save
- [ ] Session name updates in list
- [ ] Search/filter box filters sessions by name

### Components

- [ ] T018 [US2] Update src/components/SessionCard.tsx to add inline editing:
  - Add "Edit" button (pencil icon)
  - Click edit → show inline `<input>` field
  - Pre-fill with current session name
  - On blur or Enter key: call `invoke('update_session_name', { sessionId, name })`
  - Show save indicator (spinner) during update
  - Revert on error, show error message
  - Update local state optimistically

- [ ] T019 [US2] Add search/filter to src/pages/SessionHistoryPage.tsx:
  - Add search input field at top
  - Filter sessions by name (case-insensitive includes)
  - Debounce input (300ms)
  - Show "No matching sessions" when filter returns empty

---

## Phase 6: US 011.4 - Deep Dive with Zoom

**Goal**: Users can zoom into chart time ranges for detailed analysis

**Independent Test Criteria**:
- [ ] Charts display with zoom controls
- [ ] Brush selection on one chart zooms all charts
- [ ] Zoom is synchronized across all charts
- [ ] Reset zoom button restores original view
- [ ] Tooltips still work during zoom

### Components

- [ ] T020 [US4] Update src/components/TelemetryCharts.tsx to add zoom state:
  - Add `useState` for zoom domain (start/end timestamps)
  - Pass `zoomDomain` and `setZoomDomain` to all ChartContainer children
  - Add "Reset Zoom" button (only visible when zoomed)
  - Button calls `setZoomDomain(null)` to reset

- [ ] T021 [US4] Update src/components/ChartContainer.tsx to implement synchronized zoom:
  - Add Recharts `<Brush />` component to first chart only
  - Configure `syncId` prop on all LineCharts (same value = synchronized)
  - Use `domain` prop on `<XAxis>` to control zoom range
  - On brush change, call `setZoomDomain()` from parent
  - All charts re-render with new domain

---

## Phase 7: US 011.5 - Delete Unwanted Sessions

**Goal**: Users can delete sessions from the list

**Independent Test Criteria**:
- [ ] Click "Delete" button on session card
- [ ] Confirmation dialog appears
- [ ] Click "Confirm" to delete
- [ ] Session removed from Supabase
- [ ] Session removed from list (UI updates)
- [ ] Error message shown if deletion fails

### Components

- [ ] T022 [US5] Update src/components/SessionCard.tsx to add delete button:
  - Add "Delete" button (trash icon)
  - Click delete → show confirmation dialog
  - Props: `onDelete: (sessionId: string) => void`
  - Call parent `onDelete` handler when confirmed

- [ ] T023 [US5] Add delete handler to src/pages/SessionHistoryPage.tsx:
  - Create `handleDeleteSession(sessionId)` function
  - Show confirmation dialog (use window.confirm or custom modal)
  - Call `invoke('delete_session', { sessionId })`
  - On success: remove session from local state
  - On error: show error toast/message
  - Pass handler to SessionList → SessionCard

---

## Phase 8: Polish & Cross-Cutting Concerns

**Goal**: Final integration, styling, and quality improvements

### Styling & UX

- [ ] T024 [P] Style SessionHistoryPage with Tailwind CSS:
  - Match existing app design system
  - Responsive layout (mobile-friendly)
  - Loading skeletons for session cards
  - Empty state illustration/message

- [ ] T025 [P] Style SessionDetailPage with Tailwind CSS:
  - Header with session metadata card
  - Back button styling
  - Chart spacing and layout
  - Responsive grid for charts

- [ ] T026 [P] Add loading states across all components:
  - Skeleton loaders for SessionList
  - Spinner for chart loading
  - Button loading states during save/delete

- [ ] T027 [P] Add error boundaries to catch React errors:
  - Wrap SessionHistoryPage in error boundary
  - Wrap SessionDetailPage in error boundary
  - Display user-friendly error messages

### Data Handling

- [ ] T028 [P] Handle missing/null telemetry values gracefully:
  - Skip charts for parameters with all null values
  - Show "No data" message for empty sessions
  - Handle timestamp parsing errors

- [ ] T029 [P] Optimize chart rendering for large datasets:
  - Limit telemetry query to 10,000 points
  - Add downsampling if >5,000 points (optional)
  - Test with large session (benchmark render time)

### Integration

- [ ] T030 Test end-to-end flow:
  - List sessions → click session → view charts → zoom → back to list
  - Edit session name → verify persistence
  - Delete session → verify removal from list and database
  - Test with empty state (no sessions)
  - Test error handling (network failure, invalid session ID)

- [ ] T031 Verify Supabase integration:
  - All CRUD operations work correctly
  - Errors handled gracefully
  - No unwrap/expect in production code (Rust)
  - All queries return proper Result types

### Code Quality

- [ ] T032 Run `cargo clippy` on backend code:
  - Fix all warnings
  - Ensure no unwrap() or expect() in production code
  - Verify error propagation with `?` operator

- [ ] T033 Run `cargo fmt` on all Rust code:
  - Format src-tauri/src/storage/supabase.rs
  - Format src-tauri/src/commands.rs

- [ ] T034 Review TypeScript types:
  - Ensure all interfaces match Rust structs
  - No `any` types
  - Proper null handling

### Documentation

- [ ] T035 Update README.md with Session History Viewer usage:
  - How to access history page
  - How to name sessions
  - How to zoom charts
  - How to delete sessions

---

## MVP Checklist (Minimum Viable Product)

**Goal**: Get basic functionality working ASAP for user testing

**Scope**: US 011.1 + US 011.3 (Session List + Charts)

### MVP Tasks (Execute in Order):
1. ✅ Phase 1: Setup (T001-T003)
2. ✅ Phase 2: Foundational Backend (T004-T007)
3. ✅ US 011.1: Session List (T008-T013)
4. ✅ US 011.3: Telemetry Charts (T014-T017)
5. ✅ Polish: Basic styling and error handling (T024, T025, T027, T030)

**Time Estimate**: 8-10 hours
**Deliverable**: Users can browse sessions and view detailed charts

---

## Testing Checklist (Per User Story)

### US 011.1 - Session List
- [ ] Navigate to `/history` route successfully
- [ ] Sessions load from Supabase
- [ ] Sessions display with correct metadata
- [ ] Empty state works when no sessions
- [ ] Loading state shows during fetch
- [ ] Click session navigates to detail page

### US 011.2 - Session Naming
- [ ] Edit button shows on session card
- [ ] Click edit shows input field
- [ ] Type new name saves to Supabase
- [ ] Name updates in UI optimistically
- [ ] Error handling works if save fails
- [ ] Search/filter finds sessions by name

### US 011.3 - Telemetry Charts
- [ ] Session detail page loads with session ID
- [ ] All available OBD-II parameters display as charts
- [ ] Charts stack vertically
- [ ] X-axis shows time correctly
- [ ] Y-axis shows parameter values with units
- [ ] Hover tooltips show exact values
- [ ] Back button returns to list

### US 011.4 - Zoom Functionality
- [ ] Brush selection zooms chart
- [ ] Zoom synchronizes across all charts
- [ ] Reset zoom button appears when zoomed
- [ ] Reset zoom restores original view
- [ ] Tooltips still work during zoom

### US 011.5 - Session Deletion
- [ ] Delete button shows on session card
- [ ] Click delete shows confirmation dialog
- [ ] Confirm deletes session from Supabase
- [ ] Session removed from UI
- [ ] Error message shows if deletion fails

---

## Performance Targets

- Session list load: < 2 seconds (50 sessions)
- Session detail load: < 3 seconds (5000 telemetry points, 12 charts)
- Chart zoom interaction: < 16ms frame time (smooth)
- Session name update: < 1 second
- Session deletion: < 2 seconds

---

## Known Limitations (Current Phase)

1. **No AI insights** - Phase B will add automatic analysis
2. **No session comparison** - Single session view only
3. **No map view** - US 011.6 is future enhancement
4. **No export functionality** - CSV/PDF export planned for Phase B
5. **Basic filtering only** - Advanced search planned for future

---

## Next Steps After Completion

Once Phase A (this tasks document) is complete:

1. **Deploy MVP** - Get user feedback on basic viewer
2. **Create Phase B Tasks** - AI integration (separate tasks.md)
3. **Implement AI features**:
   - Automatic insights panel
   - Conversational "Ask AI" feature
   - Predictive maintenance alerts
4. **Advanced features**:
   - Session comparison overlays
   - Map view (US 011.6)
   - CSV/PDF export
   - Advanced analytics

---

## Task Execution Commands

### Start Implementation
```bash
# Ensure on correct branch
git checkout 011-session-history-viewer

# Verify dependencies
cd src-tauri && cargo check
cd .. && npm install

# Start working on tasks in order (T001 → T035)
```

### During Implementation
```bash
# Backend: Run Rust checks
cargo clippy
cargo fmt

# Frontend: Type checking
npm run type-check

# Test application
npm run dev
```

### Task Completion Tracking
Update this file as you complete tasks:
- Change `- [ ]` to `- [x]` for completed tasks
- Commit after each phase completion
- Use task IDs in commit messages (e.g., "feat: T008-T011 - Implement session list UI")

---

**Document Version**: 1.0
**Last Updated**: 2025-11-27
**Total Estimated Time**: 12-18 hours
**MVP Time**: 8-10 hours
