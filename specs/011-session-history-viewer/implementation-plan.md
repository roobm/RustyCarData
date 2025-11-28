# Implementation Plan: Session History Viewer with Data Visualization

**Branch**: `011-session-history-viewer` | **Date**: 2025-11-27 | **Spec**: [spec.md](./spec.md)

## Summary

Build a complete session history viewer that allows users to browse, name, visualize, and manage their recorded OBD-II telemetry sessions from Supabase. The interface provides a list view for browsing sessions (with custom naming like "Roope's BMW M3"), a dedicated visualization page with multi-chart line graphs for all OBD-II parameters, zoom functionality for detailed analysis, and session management capabilities (delete). Future enhancement includes GPS route mapping.

**Key User Requirements (from discussion):**
- User selects which session to view from a list (~10 sessions per customer expected)
- Sessions can be named (e.g., "Roope's BMW M3" like in dyno mode)
- Filter sessions by name
- Display all OBD-II parameters as line charts
- Create separate charts for each parameter on dedicated page
- Zoom capability for deep analysis
- Delete unwanted sessions
- Future: Map view of GPS route

## Technical Context

**Language/Version**: Rust 1.75+ (backend), TypeScript 5.x (frontend)
**Primary Dependencies**:
  - Backend: Tauri 2.0, postgrest (Supabase client), serde, uuid
  - Frontend: React 18, Recharts (charting), React Router, Tailwind CSS, lucide-react (icons)
**Storage**: Supabase PostgreSQL (sessions, telemetry_points, session_stats tables)
**Testing**: cargo test (Rust), manual testing (React)
**Target Platform**: Desktop (macOS, Windows, Linux via Tauri)
**Project Type**: Desktop application (Tauri hybrid - Rust backend + React frontend)
**Performance Goals**:
  - List 50 sessions in <2 seconds
  - Render 5000 telemetry points across 12 charts in <3 seconds
  - Smooth zoom/pan interactions (<16ms frame time)
**Constraints**:
  - Must handle up to 10,000 telemetry points per session
  - Chart rendering must remain smooth during zoom
  - Supabase queries should be optimized (indexed columns)
**Scale/Scope**:
  - ~10 sessions per user (manageable in UI without pagination)
  - 12+ OBD-II parameters to visualize
  - 4 new Rust functions, 4 new Tauri commands
  - 5 new React components, 2 new pages

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Branch Management ✅
- **Status**: PASS
- Feature developed on dedicated `011-session-history-viewer` branch
- Isolated from main branch until complete and tested

### II. Documentation ✅
- **Status**: PASS
- Complete feature spec created: `specs/011-session-history-viewer/spec.md`
- Implementation plan (this document)
- Will update README with user instructions
- Code will include inline documentation

### III. Rust Idioms and Code Quality ✅
- **Status**: PASS (Pre-commit)
- All Rust code will pass `cargo clippy` with no warnings
- All code formatted with `cargo fmt`
- No `unwrap()` or `expect()` in production code
- Explicit error handling with `Result<T, E>` and `?` operator
- Clear, readable code suitable for Rust beginners

### IV. Architecture and Modularity ✅
- **Status**: PASS
- Strict backend/frontend separation maintained via Tauri commands
- Supabase operations encapsulated in `SupabaseClient` struct
- No business logic in React components (data fetching via Tauri only)
- Clean API boundary between Rust and TypeScript
- Reusable chart components

### V. Error Handling ✅
- **Status**: PASS (Pre-commit)
- All Supabase operations return `Result<T, Box<dyn std::error::Error>>`
- Tauri commands convert errors to `String` for frontend consumption
- Frontend displays user-friendly error messages
- No panics during normal operation
- Graceful degradation for missing data

**GATE RESULT**: ✅ ALL CHECKS PASS - Proceed to Phase 0

## Project Structure

### Documentation (this feature)

```text
specs/011-session-history-viewer/
├── spec.md                  # Feature specification (COMPLETE)
├── implementation-plan.md   # This file (COMPLETE)
├── research.md              # Phase 0 output (TO BE CREATED)
├── data-model.md            # Phase 1 output (TO BE CREATED)
├── contracts/               # Phase 1 output (TO BE CREATED)
│   ├── tauri-commands.md    # Tauri command specifications
│   └── data-structures.md   # TypeScript interfaces
├── quickstart.md            # Phase 1 output (TO BE CREATED)
└── tasks.md                 # Phase 2 output via /speckit.tasks (TO BE CREATED)
```

### Source Code (repository root)

```text
# Tauri 2.0 Desktop Application Structure
src-tauri/                           # Rust backend
├── src/
│   ├── lib.rs                       # Tauri app entry point
│   ├── commands.rs                  # Tauri command handlers [MODIFY]
│   ├── models.rs                    # Shared data structures [MODIFY]
│   └── storage/
│       ├── mod.rs                   # Storage module
│       ├── supabase.rs              # Supabase client [MODIFY - add 4 methods]
│       ├── manager.rs               # Storage manager
│       └── writer.rs                # CSV writer
└── Cargo.toml

src/                                 # React frontend
├── App.tsx                          # App routes [MODIFY - add 2 routes]
├── pages/
│   ├── DashboardPage.tsx            # Existing
│   ├── SessionHistoryPage.tsx       # [NEW] Session list page
│   └── SessionDetailPage.tsx        # [NEW] Charts page
├── components/
│   ├── SessionList.tsx              # [NEW] Session list component
│   ├── SessionCard.tsx              # [NEW] Individual session card
│   ├── TelemetryCharts.tsx          # [NEW] Multi-chart container
│   ├── ChartContainer.tsx           # [NEW] Individual chart wrapper
│   └── (existing components...)
└── store/
    └── useStore.ts                  # [MODIFY - add session state]

tests/
└── integration/
    └── session_history_test.rs      # [NEW] Backend integration tests
```

**Structure Decision**: Tauri 2.0 desktop application follows standard hybrid architecture:
- `src-tauri/` contains Rust backend (Tauri commands, Supabase client, business logic)
- `src/` contains React frontend (UI components, pages, routing)
- Backend exposes Tauri commands as IPC boundary
- Frontend consumes commands via `@tauri-apps/api/core` invoke

**Key Modifications**:
1. **Backend**: Extend `src-tauri/src/storage/supabase.rs` with 4 new methods
2. **Backend**: Add 4 new Tauri commands to `src-tauri/src/commands.rs`
3. **Frontend**: Create 2 new pages (`SessionHistoryPage`, `SessionDetailPage`)
4. **Frontend**: Create 4 new components (`SessionList`, `SessionCard`, `TelemetryCharts`, `ChartContainer`)
5. **Frontend**: Add 2 routes to `App.tsx`

## Complexity Tracking

> No Constitution violations - this section not required.

## Phase 0: Outline & Research

**Goal**: Resolve all technical unknowns and research best practices.

### Research Questions

1. **Recharts Multi-Chart Synchronization**
   - How to synchronize zoom across multiple `<LineChart>` components?
   - Best practice for shared X-axis state management?
   - Performance optimization for 12+ charts rendering simultaneously?

2. **Supabase Cascade Delete**
   - Does PostgreSQL cascade delete work with foreign key constraints?
   - Need to manually delete `telemetry_points` and `session_stats` before `sessions`?
   - Transaction support in postgrest client?

3. **Chart Performance**
   - Recharts performance with 5000-10000 data points per chart?
   - Need data downsampling/aggregation for large datasets?
   - Virtualization or lazy loading strategies?

4. **Session Naming UX**
   - Inline editing best practice (click-to-edit vs dedicated modal)?
   - Auto-save vs explicit save button?
   - Handle concurrent edits if multiple tabs open?

5. **Zoom Implementation**
   - Recharts built-in zoom vs custom implementation?
   - `react-zoom-pan-pinch` library vs native Recharts zoom?
   - Synchronized zoom state management pattern?

### Research Tasks

- **TASK R1**: Research Recharts synchronized zoom patterns
  - Review Recharts documentation on `syncId` prop
  - Test performance with 12 charts and 5000 points each
  - Identify optimal downsampling strategy if needed

- **TASK R2**: Test Supabase cascade delete behavior
  - Review PostgreSQL foreign key cascade settings
  - Test postgrest DELETE with relationships
  - Determine if manual cascade needed

- **TASK R3**: Prototype chart performance
  - Create test data with 10,000 points
  - Benchmark Recharts rendering time
  - Implement downsampling if >3 second render time

- **TASK R4**: Design session naming UX flow
  - Sketch wireframes for inline editing
  - Decide on auto-save vs manual save
  - Plan error handling for save failures

- **TASK R5**: Evaluate zoom libraries
  - Compare Recharts `brush` component vs custom zoom
  - Test synchronized zoom across multiple charts
  - Select implementation approach

**Output**: `research.md` documenting all decisions with rationale

## Phase 1: Design & Contracts

**Goal**: Define data models, API contracts, and component interfaces.

### Data Model

**Existing (No Changes Required)**:
- `SessionRow` - Session metadata (already includes `session_name`)
- `TelemetryRow` - Telemetry data points (all OBD-II parameters)
- `SessionStatsRow` - Aggregated statistics

**New Data Structures**:
```rust
// src-tauri/src/models.rs

/// Combined session data for frontend display
#[derive(Serialize)]
pub struct SessionData {
    pub session: SessionRow,
    pub telemetry: Vec<TelemetryRow>,
    pub stats: Option<SessionStatsRow>,
}

/// Search results for session filtering
#[derive(Serialize)]
pub struct SessionSearchResult {
    pub sessions: Vec<SessionRow>,
    pub total_count: usize,
}
```

```typescript
// src/types/session.ts (NEW FILE)

interface Session {
  id: string;
  created_at: string;
  started_at: string;
  ended_at?: string;
  session_name?: string;
  dataset_source?: 'playback' | 'random_walk' | 'imported' | 'live';
  vehicle_make?: string;
  vehicle_model?: string;
  vehicle_year?: number;
  duration_seconds?: number;
  distance_km?: number;
}

interface TelemetryPoint {
  session_id: string;
  timestamp: string;
  engine_rpm?: number;
  vehicle_speed?: number;
  coolant_temp?: number;
  throttle_position?: number;
  engine_load?: number;
  fuel_level?: number;
  intake_air_temp?: number;
  timing_advance?: number;
  battery_voltage?: number;
  maf_flow_rate?: number;
  o2_sensor_voltage?: number;
  fuel_pressure?: number;
}

interface SessionStats {
  session_id: string;
  avg_speed?: number;
  max_speed?: number;
  avg_rpm?: number;
  max_rpm?: number;
  avg_coolant_temp?: number;
  max_coolant_temp?: number;
  avg_throttle_position?: number;
  avg_engine_load?: number;
  fuel_consumed_liters?: number;
  total_points: number;
  missing_values_count: number;
}

interface SessionData {
  session: Session;
  telemetry: TelemetryPoint[];
  stats?: SessionStats;
}
```

### API Contracts (Tauri Commands)

**File**: `specs/011-session-history-viewer/contracts/tauri-commands.md`

```rust
/// List all sessions with optional limit
#[tauri::command]
pub async fn list_sessions(limit: Option<usize>) -> Result<Vec<SessionRow>, String>

/// Get full session data including telemetry and stats
#[tauri::command]
pub async fn get_session_data(session_id: String) -> Result<SessionData, String>

/// Update session name
#[tauri::command]
pub async fn update_session_name(
    session_id: String,
    name: String,
) -> Result<(), String>

/// Delete session and all related data
#[tauri::command]
pub async fn delete_session(session_id: String) -> Result<(), String>

/// Search sessions by name or vehicle
#[tauri::command]
pub async fn search_sessions(
    search_term: String,
    limit: Option<usize>,
) -> Result<Vec<SessionRow>, String>
```

### Component Contracts

**File**: `specs/011-session-history-viewer/contracts/components.md`

```typescript
// SessionHistoryPage.tsx
interface SessionHistoryPageProps {}
// Fetches and displays session list
// Handles navigation to detail page

// SessionList.tsx
interface SessionListProps {
  sessions: Session[];
  onSelectSession: (sessionId: string) => void;
  onDeleteSession: (sessionId: string) => void;
  onUpdateName: (sessionId: string, name: string) => void;
  isLoading: boolean;
}

// SessionCard.tsx
interface SessionCardProps {
  session: Session;
  onSelect: () => void;
  onDelete: () => void;
  onUpdateName: (name: string) => void;
}

// SessionDetailPage.tsx
interface SessionDetailPageProps {
  sessionId: string; // from URL params
}
// Fetches session data and displays charts

// TelemetryCharts.tsx
interface TelemetryChartsProps {
  telemetry: TelemetryPoint[];
  sessionName: string;
}
// Renders all available parameter charts
// Manages synchronized zoom state

// ChartContainer.tsx
interface ChartContainerProps {
  data: Array<{ timestamp: string; value: number }>;
  parameterName: string;
  unit: string;
  color: string;
  syncId: string; // for chart synchronization
}
```

**Output**:
- `data-model.md` - Complete data structure documentation
- `contracts/tauri-commands.md` - Tauri command specifications
- `contracts/components.md` - React component interfaces
- `quickstart.md` - Developer getting started guide

## Phase 2: Implementation Tasks

**Note**: Detailed task breakdown will be generated via `/speckit.tasks` command.

### Backend Tasks (src-tauri/)

**T1.1**: Extend `SupabaseClient` with session management methods
- Add `update_session_name()`
- Add `delete_session()` with cascade logic
- Add `search_sessions()` with ILIKE query
- Add `get_session_stats()`

**T1.2**: Add Tauri command handlers
- `list_sessions()` - wrapper for SupabaseClient
- `get_session_data()` - combine session + telemetry + stats
- `update_session_name()` - wrapper for SupabaseClient
- `delete_session()` - wrapper for SupabaseClient
- `search_sessions()` - wrapper for SupabaseClient

**T1.3**: Add data structures to `models.rs`
- `SessionData` struct
- `SessionSearchResult` struct
- Serde serialization support

**T1.4**: Write integration tests
- Test session CRUD operations
- Test search functionality
- Test cascade delete
- Test error handling

### Frontend Tasks (src/)

**T2.1**: Create TypeScript types
- Create `src/types/session.ts`
- Define all interfaces matching Rust structs

**T2.2**: Create SessionHistoryPage
- Page component with routing
- Fetch sessions on mount
- Search bar component
- Loading and empty states
- Navigation to detail page

**T2.3**: Create SessionList component
- Render session cards
- Handle click events
- Search/filter logic
- Infinite scroll or pagination (if needed)

**T2.4**: Create SessionCard component
- Display session metadata
- Inline name editing with auto-save
- Delete button with confirmation
- Click to navigate to detail

**T2.5**: Create SessionDetailPage
- Fetch session data by ID from URL params
- Display session metadata header
- Render TelemetryCharts component
- Back button to history page
- Loading and error states

**T2.6**: Create TelemetryCharts component
- Map telemetry data to chart data format
- Render ChartContainer for each OBD-II parameter
- Manage synchronized zoom state
- Reset zoom button
- Responsive grid layout

**T2.7**: Create ChartContainer component
- Recharts `<LineChart>` integration
- Parameter-specific styling (colors, units)
- Synchronized zoom via `syncId`
- Hover tooltips
- Axis formatting

**T2.8**: Add routing to App.tsx
- `/history` route → SessionHistoryPage
- `/history/:sessionId` route → SessionDetailPage

**T2.9**: Add navigation link
- Add "History" link to sidebar/header
- Icon and label

**T2.10**: Styling and polish
- Tailwind CSS styling matching existing design
- Responsive layout
- Loading skeletons
- Error boundaries
- Toast notifications for actions

### Testing Tasks

**T3.1**: Manual testing checklist
- Empty state
- Session list display
- Search/filter
- Name editing
- Session deletion
- Detail page charts
- Zoom functionality
- Navigation flow

**T3.2**: Performance testing
- Large session (10K points)
- Multiple chart rendering
- Zoom responsiveness

**T3.3**: Edge case testing
- No Supabase connection
- Empty telemetry data
- Partial data (missing PIDs)

## Success Metrics

### Backend ✅
- [ ] All Supabase methods pass integration tests
- [ ] All Tauri commands return correct data structures
- [ ] Error handling covers all failure modes
- [ ] Code passes `cargo clippy` and `cargo fmt`
- [ ] No `unwrap()` or `expect()` in production code

### Frontend ✅
- [ ] Session list displays all sessions correctly
- [ ] Session naming/editing works reliably
- [ ] Session deletion works with confirmation
- [ ] Detail page renders all 12+ charts
- [ ] Charts display correct data and labels
- [ ] Zoom synchronizes across all charts
- [ ] UI matches design system (Tailwind)
- [ ] All interactions are responsive (<100ms)

### User Experience ✅
- [ ] Users can browse ~10 sessions easily
- [ ] Users can identify sessions by custom names
- [ ] Users can filter sessions by name
- [ ] Users can visualize all telemetry parameters
- [ ] Users can zoom for detailed analysis
- [ ] Users can delete unwanted sessions
- [ ] Interface requires no training to use

### Performance ✅
- [ ] Session list loads in <2 seconds (50 sessions)
- [ ] Detail page renders in <3 seconds (5000 points, 12 charts)
- [ ] Zoom/pan interactions are smooth (<16ms frame time)
- [ ] Search/filter is instant (<100ms)

## Timeline Estimate

- Phase 0 (Research): 2-3 hours
- Phase 1 (Design): 2-3 hours
- Phase 2 Implementation:
  - Backend (T1.x): 3-4 hours
  - Frontend (T2.x): 6-8 hours
  - Testing (T3.x): 2-3 hours
- Integration & Polish: 2-3 hours

**Total: 17-24 hours of focused work**

## Risks & Mitigation

### Risk 1: Chart Performance with Large Datasets
**Impact**: High
**Probability**: Medium
**Mitigation**:
- Implement data downsampling for >5000 points
- Use Recharts optimization features (e.g., `isAnimationActive={false}`)
- Lazy load charts (only render visible charts)

### Risk 2: Supabase Query Performance
**Impact**: Medium
**Probability**: Low
**Mitigation**:
- Ensure `session_id` column is indexed in `telemetry_points` table
- Limit telemetry queries to 10,000 points
- Add query timeout handling

### Risk 3: Concurrent Session Name Updates
**Impact**: Low
**Probability**: Low
**Mitigation**:
- Optimistic UI updates with rollback on error
- Debounce name updates (500ms delay)
- Show save status indicator

### Risk 4: Missing GPS Data for Map View
**Impact**: Medium (for future feature)
**Probability**: High
**Mitigation**:
- Phase 5 (Map View) is explicitly marked as future enhancement
- Current implementation focuses on telemetry visualization
- GPS data collection can be added in parallel

## Next Steps

1. ✅ Complete spec.md (DONE)
2. ✅ Complete implementation-plan.md (THIS DOCUMENT)
3. ⏳ Execute Phase 0 research tasks → `research.md`
4. ⏳ Execute Phase 1 design tasks → `data-model.md`, `contracts/`, `quickstart.md`
5. ⏳ Run `/speckit.tasks` to generate detailed task breakdown → `tasks.md`
6. ⏳ Begin implementation following task sequence

## Questions for User (Already Answered)

✅ Q1: How should sessions be displayed? → **User selects from list**
✅ Q2: Expected session count? → **~10 per customer, need naming like "Roope's BMW M3"**
✅ Q3: Which OBD-II parameters to show? → **All of them as line charts**
✅ Q4: Single chart or multiple? → **Separate charts for each parameter on dedicated page**
✅ Q5: Zoom needed? → **Yes, for deep analysis**
✅ Q10: Delete sessions? → **Yes**
✅ Q10: Map view? → **Yes, future enhancement (Phase 5)**

All user requirements captured in spec.md and this implementation plan.
