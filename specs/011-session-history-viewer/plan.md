# Implementation Plan: Session History and Data Visualization

**Feature**: 011-session-history-viewer
**Branch**: `011-session-history-viewer`
**Status**: Planning
**Priority**: P2 (Completes User Story 2 - Data Recording)

## Problem Statement

Users can record sessions to Supabase, but:
- No way to **view** past sessions
- No way to **select** a specific session
- No way to **visualize** telemetry data in graphs
- Can't compare sessions or review driving patterns

**Result**: Data is being collected but not accessible for review or analysis.

## Solution: Session History Viewer with Data Visualization

Build a complete history interface that allows users to:
1. **Browse** all recorded sessions (from Supabase)
2. **Select** a session to view details
3. **Visualize** telemetry data in interactive time-series graphs
4. **Filter** sessions by date, source type, duration

## User Stories

### US2.1: View Session List
**As a user**, I want to see a list of all my recorded sessions so I can browse my driving history.

**Acceptance Criteria:**
- Display sessions in a table/list format
- Show session metadata: name, date, duration, source type (playback/random_walk/imported)
- Sort by date (newest first by default)
- Show loading state while fetching from Supabase
- Handle empty state (no sessions yet)

### US2.2: View Session Details with Graphs
**As a user**, I want to click on a session and see detailed telemetry graphs so I can analyze my driving patterns.

**Acceptance Criteria:**
- Click a session row to load its telemetry data
- Display multiple synchronized time-series graphs:
  - RPM over time
  - Speed over time
  - Coolant Temperature over time
  - Throttle Position over time
  - Engine Load over time
- Graphs are interactive (hover to see values)
- All graphs share the same X-axis (time) for easy correlation
- Show session summary stats (max speed, max RPM, duration)

## Technical Approach

### Backend (Rust/Tauri)

#### 1. Extend Supabase Client

Already have `list_sessions()` method. Need to add:

```rust
// src-tauri/src/storage/supabase.rs

impl SupabaseClient {
    /// Get telemetry data for a specific session
    pub async fn get_session_telemetry(
        &self,
        session_id: Uuid,
    ) -> Result<Vec<TelemetryRow>, Box<dyn std::error::Error>> {
        // Query telemetry_points WHERE session_id = ?
        // ORDER BY timestamp ASC
        // LIMIT 10000 (reasonable limit for UI)
    }
}
```

#### 2. Add Tauri Commands

```rust
// src-tauri/src/commands.rs

#[tauri::command]
pub async fn list_sessions() -> Result<Vec<SessionRow>, String> {
    // Use SupabaseClient::list_sessions()
    // Return list ordered by started_at DESC
}

#[tauri::command]
pub async fn read_session(session_id: String) -> Result<SessionData, String> {
    // 1. Parse UUID from string
    // 2. Get session metadata
    // 3. Get telemetry data
    // 4. Return combined structure
}
```

**Data Structure:**
```rust
#[derive(Serialize)]
pub struct SessionData {
    pub session: SessionRow,
    pub telemetry: Vec<TelemetryRow>,
    pub stats: SessionStats,
}

#[derive(Serialize)]
pub struct SessionStats {
    pub max_speed: f64,
    pub avg_speed: f64,
    pub max_rpm: f64,
    pub avg_rpm: f64,
    pub point_count: usize,
}
```

### Frontend (React/TypeScript)

#### 1. New Page: SessionHistory

```
src/pages/SessionHistory.tsx
```

**Layout:**
```
┌─────────────────────────────────────────────┐
│  Session History                            │
├─────────────────────────────────────────────┤
│  ┌─────────────────────────────────────┐   │
│  │ Session List (Left Panel - 30%)     │   │
│  │                                      │   │
│  │ [ ] Session 2025-11-27 10:30        │   │
│  │     Playback | 5min | Toyota Etios  │   │
│  │                                      │   │
│  │ [x] Session 2025-11-27 09:15   ◄───┼───│ Selected
│  │     Random Walk | 2min              │   │
│  │                                      │   │
│  │ [ ] Imported: session_20251122      │   │
│  │     Imported | 10min                │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │ Telemetry Graphs (Right Panel - 70%)│   │
│  │                                      │   │
│  │  RPM Chart  ▁▂▃▅▇█▇▅▃▂▁             │   │
│  │  Speed Chart ▁▁▂▃▄▅▅▄▃▂▁            │   │
│  │  Temp Chart  ──▁▂▃▃▃▃▃▃             │   │
│  │                                      │   │
│  └─────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

#### 2. Components

**SessionList.tsx**
- Fetches sessions via `list_sessions()` command
- Displays list with metadata
- Handles click → emits `onSelectSession(session_id)`
- Shows loading/empty states

**SessionDetails.tsx**
- Receives `session_id` prop
- Fetches telemetry via `read_session()` command
- Displays session metadata
- Renders charts

**TelemetryCharts.tsx**
- Receives telemetry data array
- Uses Recharts library (already installed)
- Creates multiple synchronized `LineChart` components
- Responsive design

#### 3. Navigation

Add route to `src/App.tsx`:
```typescript
<Route path="/history" element={<SessionHistory />} />
```

Add navigation link to sidebar/header.

## Data Flow

```
User clicks "History" tab
    ↓
SessionHistory page loads
    ↓
Calls list_sessions() Tauri command
    ↓
Backend queries Supabase sessions table
    ↓
Returns SessionRow[] to frontend
    ↓
SessionList component renders rows
    ↓
User clicks a session row
    ↓
Calls read_session(session_id) Tauri command
    ↓
Backend:
  1. Queries sessions table (metadata)
  2. Queries telemetry_points table (data)
  3. Calculates stats (max/avg)
    ↓
Returns SessionData to frontend
    ↓
SessionDetails renders:
  - Metadata card
  - Stats summary
  - TelemetryCharts component
    ↓
Charts display interactive time-series graphs
```

## Implementation Steps

### Phase 1: Backend - Data Access
- [ ] **T020.1** Add `get_session_telemetry()` to `SupabaseClient`
- [ ] **T020.2** Add `list_sessions()` Tauri command
- [ ] **T020.3** Add `read_session()` Tauri command
- [ ] **T020.4** Add `SessionStats` calculation helper

### Phase 2: Frontend - Session List
- [ ] **T021.1** Create `SessionHistory` page component
- [ ] **T021.2** Create `SessionList` component
- [ ] **T021.3** Implement session fetching and display
- [ ] **T021.4** Add loading/empty states
- [ ] **T021.5** Add click handler for selection

### Phase 3: Frontend - Data Visualization
- [ ] **T022.1** Create `SessionDetails` component
- [ ] **T022.2** Create `TelemetryCharts` component
- [ ] **T022.3** Implement Recharts integration
- [ ] **T022.4** Add multiple synchronized charts (RPM, Speed, Temp, etc.)
- [ ] **T022.5** Add hover tooltips and interactivity

### Phase 4: Integration & Polish
- [ ] **T023.1** Add navigation route to App.tsx
- [ ] **T023.2** Add "History" link to navigation
- [ ] **T023.3** Style with Tailwind CSS (match existing design)
- [ ] **T023.4** Add responsive layout
- [ ] **T023.5** Test with real Supabase data

## Testing Strategy

### Manual Testing
1. **Empty State**: No sessions exist
   - Should show "No sessions yet" message
2. **List Display**: Multiple sessions exist
   - Should display all sessions ordered by date
3. **Session Selection**: Click a session
   - Should load and display telemetry graphs
4. **Large Session**: Session with 1000+ telemetry points
   - Charts should render smoothly
5. **Different Sources**: Test playback, random_walk, imported sessions
   - All should display correctly

### Edge Cases
- No Supabase connection → Show error message
- Empty telemetry data → Show "No data" message
- Session with missing PIDs → Display available data only

## Dependencies

### Already Available
- ✅ `recharts` - Chart library (installed)
- ✅ `lucide-react` - Icons (installed)
- ✅ Tailwind CSS - Styling
- ✅ Supabase schema and client
- ✅ Session recording working

### New Additions
- None required! Everything is already installed.

## Success Criteria

**Backend:**
- [ ] `list_sessions()` command returns all sessions from Supabase
- [ ] `read_session()` command returns session + telemetry data
- [ ] Commands handle errors gracefully

**Frontend:**
- [ ] History page displays list of sessions
- [ ] Clicking a session loads its data
- [ ] Charts display telemetry data clearly
- [ ] UI is responsive and matches design
- [ ] Navigation works correctly

**User Experience:**
- [ ] User can browse all their sessions
- [ ] User can visualize any session's data
- [ ] Graphs are interactive and informative
- [ ] Loading states are clear
- [ ] Empty states are handled well

## Timeline Estimate

- Phase 1 (Backend): ~1-2 hours
- Phase 2 (Session List): ~1-2 hours
- Phase 3 (Visualization): ~2-3 hours
- Phase 4 (Integration): ~1 hour

**Total: ~5-8 hours of focused work**

## Notes

- This completes **User Story 2** from the original tasks
- Builds on existing Supabase infrastructure
- Reuses existing UI components and styling
- Chart library (Recharts) is already integrated in codebase
- Can be implemented incrementally (list first, then graphs)
