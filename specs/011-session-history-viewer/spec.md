# Feature Specification: Session History Viewer with Data Visualization

**Feature ID**: 011-session-history-viewer
**Status**: Planning
**Priority**: P1 (User-facing feature, completes data recording workflow)
**Branch**: `011-session-history-viewer`

## Problem Statement

Users can record OBD-II telemetry sessions to Supabase cloud storage, but there is currently **no way to view or analyze this data**. The recorded sessions exist in the database but are completely inaccessible from the UI.

**Current Pain Points:**
- Cannot see list of past driving sessions
- Cannot select a specific session to review
- Cannot visualize telemetry data (RPM, speed, temperature, etc.)
- Cannot name/identify sessions (e.g., "Roope's BMW M3")
- Cannot filter or search sessions
- Cannot delete unwanted sessions
- No route/map visualization

**User Impact**: All recorded data is essentially "write-only" – it goes into the cloud but users can't access it for analysis or review.

## Solution Overview

Build a **Session History Viewer** that provides:

1. **Session Browser**: List view of all recorded sessions with metadata
2. **Session Naming**: Ability to name sessions (like "Roope's BMW M3" in dyno mode)
3. **Data Visualization**: Multi-chart view showing all OBD-II parameters over time
4. **Interactive Charts**: Zoom capability for detailed analysis
5. **Session Management**: Delete unwanted sessions
6. **Route Visualization**: Map view showing GPS-tracked route (future enhancement)

## User Stories

### US 011.1: View Session List with Vehicle Names

**As a user**, I want to see a list of all my recorded sessions with custom names so I can easily identify which car/session I'm looking at.

**Acceptance Criteria:**
- Display sessions in a list/table format
- Show session metadata:
  - **Custom name** (e.g., "Roope's BMW M3", "Highway Test Drive")
  - Date and time
  - Duration
  - Source type (playback/random_walk/imported)
  - Vehicle details (make, model, year) if available
- Sessions sorted by date (newest first)
- Filter sessions by vehicle name
- Loading state while fetching from Supabase
- Empty state when no sessions exist

### US 011.2: Name and Organize Sessions

**As a user**, I want to name my sessions so I can identify them later.

**Acceptance Criteria:**
- Ability to set session name during recording (like dyno mode)
- Ability to edit session name after recording
- Suggested format: "{Owner}'s {Vehicle}" (e.g., "Roope's BMW M3")
- Filter/search by session name

### US 011.3: Visualize All Telemetry Data

**As a user**, I want to see all OBD-II parameters as line charts on a dedicated page so I can analyze driving patterns.

**Acceptance Criteria:**
- Click a session to navigate to dedicated visualization page
- Display **all** available OBD-II parameters as separate synchronized charts:
  - Engine RPM
  - Vehicle Speed
  - Coolant Temperature
  - Throttle Position
  - Engine Load
  - Fuel Level
  - Intake Air Temperature
  - Timing Advance
  - Battery Voltage
  - MAF Flow Rate
  - O2 Sensor Voltage
  - Fuel Pressure
- Each chart shows parameter over time (X-axis: time, Y-axis: value)
- All charts share synchronized X-axis for easy correlation
- Charts stack vertically for easy comparison

### US 011.4: Deep Dive with Zoom

**As a user**, I want to zoom into specific time ranges on the charts so I can examine details closely.

**Acceptance Criteria:**
- Zoom controls on each chart
- Synchronized zoom across all charts (zoom on one = zoom on all)
- Reset zoom button
- Smooth zoom interaction
- Tooltips showing exact values on hover

### US 011.5: Delete Unwanted Sessions

**As a user**, I want to delete sessions I no longer need so my history stays organized.

**Acceptance Criteria:**
- Delete button on each session in the list
- Confirmation dialog before deletion
- Successful deletion removes session from list and database
- Error handling if deletion fails

### US 011.6: View Route on Map (Future)

**As a user**, I want to see the GPS route of my drive on a map so I can remember where I was driving.

**Acceptance Criteria (Future Implementation):**
- Display map showing GPS coordinates
- Route line connecting all points
- Markers for start/end locations
- Ability to click map point and see telemetry data at that location
- Integration with OpenStreetMap or similar
- *Note: Requires GPS data collection first*

## Technical Architecture

### Backend (Rust/Tauri)

#### 1. Extend Supabase Client

**Already Exists:**
- ✅ `get_session_telemetry()` - Fetches telemetry points
- ✅ `list_sessions()` - Lists all sessions
- ✅ `get_session()` - Gets single session

**Needs to Add:**
```rust
// src-tauri/src/storage/supabase.rs

impl SupabaseClient {
    /// Update session name
    pub async fn update_session_name(
        &self,
        session_id: Uuid,
        name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // UPDATE sessions SET session_name = ? WHERE id = ?
    }

    /// Delete session and all its telemetry points
    pub async fn delete_session(
        &self,
        session_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // DELETE FROM telemetry_points WHERE session_id = ?
        // DELETE FROM session_stats WHERE session_id = ?
        // DELETE FROM sessions WHERE id = ?
    }

    /// Get session statistics
    pub async fn get_session_stats(
        &self,
        session_id: Uuid,
    ) -> Result<Option<SessionStatsRow>, Box<dyn std::error::Error>> {
        // SELECT * FROM session_stats WHERE session_id = ?
    }

    /// Filter sessions by vehicle name/session name
    pub async fn search_sessions(
        &self,
        search_term: String,
        limit: usize,
    ) -> Result<Vec<SessionRow>, Box<dyn std::error::Error>> {
        // SELECT * FROM sessions
        // WHERE session_name ILIKE ? OR vehicle_model ILIKE ?
        // ORDER BY created_at DESC
        // LIMIT ?
    }
}
```

#### 2. Add Tauri Commands

```rust
// src-tauri/src/commands.rs

#[tauri::command]
pub async fn list_sessions(limit: Option<usize>) -> Result<Vec<SessionRow>, String> {
    // Fetch sessions from Supabase
    // Default limit: 50
}

#[tauri::command]
pub async fn get_session_data(session_id: String) -> Result<SessionData, String> {
    // 1. Parse UUID
    // 2. Fetch session metadata
    // 3. Fetch telemetry data (limit 10000 points for UI performance)
    // 4. Fetch session stats
    // 5. Return combined structure
}

#[tauri::command]
pub async fn update_session_name(
    session_id: String,
    name: String,
) -> Result<(), String> {
    // Update session name in Supabase
}

#[tauri::command]
pub async fn delete_session(session_id: String) -> Result<(), String> {
    // Delete session and all related data
}

#[tauri::command]
pub async fn search_sessions(
    search_term: String,
    limit: Option<usize>,
) -> Result<Vec<SessionRow>, String> {
    // Search sessions by name or vehicle
}
```

**Data Structures:**
```rust
#[derive(Serialize)]
pub struct SessionData {
    pub session: SessionRow,
    pub telemetry: Vec<TelemetryRow>,
    pub stats: Option<SessionStatsRow>,
}
```

### Frontend (React/TypeScript)

#### 1. New Page: SessionHistory

**File**: `src/pages/SessionHistoryPage.tsx`

**Layout:**
```
┌────────────────────────────────────────────────────────────┐
│  Session History                                 [Search 🔍]│
├────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────┐  │
│  │ Session List                                         │  │
│  │                                                      │  │
│  │ ┌─────────────────────────────────────────────┐    │  │
│  │ │ 📊 Roope's BMW M3                    [Edit]  │    │  │
│  │ │    2025-11-27 10:30 • 5 min • Playback      │    │  │
│  │ │    3,245 points • Max: 6500 RPM, 180 km/h   │    │  │
│  │ └─────────────────────────────────────────────┘    │  │
│  │                                                      │  │
│  │ ┌─────────────────────────────────────────────┐    │  │
│  │ │ 📊 Highway Test Drive            [Edit] [❌] │    │  │
│  │ │    2025-11-27 09:15 • 12 min • Random       │    │  │
│  │ │    6,821 points • Max: 4200 RPM, 120 km/h   │    │  │
│  │ └─────────────────────────────────────────────┘    │  │
│  │                                                      │  │
│  └──────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────┘
```

#### 2. New Page: SessionDetail

**File**: `src/pages/SessionDetailPage.tsx`

**Layout:**
```
┌────────────────────────────────────────────────────────────┐
│  ← Back to History      Roope's BMW M3                     │
│                         2025-11-27 10:30 • 5 min          │
├────────────────────────────────────────────────────────────┤
│  [🔍 Reset Zoom]                                           │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │ Engine RPM                                           │ │
│  │ 8000├─────────────▁▂▃▅▇█▇▅▃▂▁────────────────────   │ │
│  │     0                    Time                        │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │ Vehicle Speed (km/h)                                 │ │
│  │ 200├──────────▁▁▂▃▄▅▅▄▃▂▁─────────────────────────  │ │
│  │     0                    Time                        │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐ │
│  │ Coolant Temperature (°C)                             │ │
│  │ 100├────────────▁▂▃▃▃▃▃▃──────────────────────────  │ │
│  │     0                    Time                        │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                            │
│  ... (More charts for other parameters)                   │
└────────────────────────────────────────────────────────────┘
```

#### 3. Components

**SessionList.tsx**
- Fetches sessions via `list_sessions()` command
- Displays session cards with metadata
- Each card shows:
  - Session name (editable inline)
  - Date/time, duration, source type
  - Key stats (max RPM, max speed)
  - Edit and Delete buttons
- Click card → navigate to `/history/:sessionId`
- Handles loading/empty states

**SessionCard.tsx**
- Individual session card component
- Inline name editing
- Delete confirmation dialog
- Stats display

**TelemetryCharts.tsx**
- Receives telemetry data array
- Uses **Recharts** library (already installed)
- Creates synchronized `LineChart` for each OBD-II parameter
- Shared zoom/pan state across all charts
- Synchronized tooltips
- Responsive design

**ChartContainer.tsx**
- Wrapper for individual chart
- Handles zoom controls
- Parameter-specific formatting (units, colors)

#### 4. Navigation & Routing

Add routes to `src/App.tsx`:
```typescript
<Route path="/history" element={<SessionHistoryPage />} />
<Route path="/history/:sessionId" element={<SessionDetailPage />} />
```

Add navigation link in sidebar/header.

## Data Flow

### Viewing Session List
```
User clicks "History" tab
    ↓
SessionHistoryPage loads
    ↓
Calls list_sessions(50) Tauri command
    ↓
Backend queries Supabase sessions table
    ↓
Returns SessionRow[] to frontend
    ↓
SessionList renders session cards
    ↓
User can search/filter sessions
```

### Viewing Session Details
```
User clicks a session card
    ↓
Navigate to /history/:sessionId
    ↓
SessionDetailPage loads
    ↓
Calls get_session_data(sessionId) Tauri command
    ↓
Backend:
  1. Queries sessions table (metadata)
  2. Queries telemetry_points table (data, limit 10K points)
  3. Queries session_stats table (stats)
    ↓
Returns SessionData to frontend
    ↓
TelemetryCharts renders:
  - Separate chart for each OBD-II parameter
  - All charts synchronized on X-axis (time)
  - Zoom controls
  - Tooltips
```

### Naming a Session
```
User clicks [Edit] on session card
    ↓
Inline input appears
    ↓
User types name (e.g., "Roope's BMW M3")
    ↓
User presses Enter or clicks away
    ↓
Calls update_session_name(sessionId, name) Tauri command
    ↓
Backend updates Supabase
    ↓
Frontend updates local state
    ↓
Success notification
```

### Deleting a Session
```
User clicks [Delete] button
    ↓
Confirmation dialog appears
    ↓
User confirms deletion
    ↓
Calls delete_session(sessionId) Tauri command
    ↓
Backend:
  1. DELETE FROM telemetry_points WHERE session_id = ?
  2. DELETE FROM session_stats WHERE session_id = ?
  3. DELETE FROM sessions WHERE id = ?
    ↓
Frontend removes session from list
    ↓
Success notification
```

## Constitution Check

### I. Branch Management ✅
- Feature developed on `011-session-history-viewer` branch
- Isolated from main branch

### II. Documentation ✅
- This spec documents the feature completely
- Will update README with History feature instructions
- Code will include inline documentation

### III. Rust Idioms and Code Quality ✅
- All code will pass `cargo clippy` and `cargo fmt`
- No `unwrap()` or `expect()` in production code
- All errors propagated using `Result<T, E>` and `?` operator
- Clear, readable code suitable for Rust newbies

### IV. Architecture and Modularity ✅
- Backend/frontend separation maintained
- Supabase client properly abstracted
- Tauri commands as clean API boundary
- No business logic in frontend

### V. Error Handling ✅
- All Supabase operations return `Result<T, Box<dyn Error>>`
- Tauri commands convert errors to `String` for frontend
- Frontend displays user-friendly error messages
- No panics in normal operation

## Dependencies

### Already Available ✅
- `recharts` - Chart library for React
- `lucide-react` - Icon library
- `tailwindcss` - Styling
- `react-router-dom` - Routing
- Supabase PostgreSQL schema (sessions, telemetry_points, session_stats)
- SupabaseClient with core CRUD operations

### New Dependencies
- **None required!** All dependencies already installed.

### Future Dependencies (for Map View)
- `react-leaflet` or `mapbox-gl` - Map rendering
- GPS coordinate collection in telemetry schema

## Implementation Phases

### Phase 1: Backend - Session Management (2-3 hours)
- Add `update_session_name()` to SupabaseClient
- Add `delete_session()` to SupabaseClient (cascade delete)
- Add `search_sessions()` to SupabaseClient
- Add `get_session_stats()` to SupabaseClient
- Add Tauri commands for all operations
- Test all backend operations

### Phase 2: Frontend - Session List (2-3 hours)
- Create `SessionHistoryPage` component
- Create `SessionList` component
- Create `SessionCard` component with inline editing
- Implement session fetching and display
- Add search/filter functionality
- Add delete confirmation dialog
- Handle loading/empty states

### Phase 3: Frontend - Data Visualization (3-4 hours)
- Create `SessionDetailPage` component
- Create `TelemetryCharts` component
- Create `ChartContainer` for individual charts
- Implement Recharts integration for all OBD-II parameters:
  - Engine RPM
  - Vehicle Speed
  - Coolant Temperature
  - Throttle Position
  - Engine Load
  - Fuel Level
  - Intake Air Temperature
  - Timing Advance
  - Battery Voltage
  - MAF Flow Rate
  - O2 Sensor Voltage
  - Fuel Pressure
- Implement synchronized zoom across all charts
- Add hover tooltips with exact values
- Responsive design

### Phase 4: Integration & Polish (1-2 hours)
- Add routes to App.tsx
- Add "History" link to navigation
- Style with Tailwind CSS (match existing design)
- Add loading skeletons
- Add error boundaries
- Test with real Supabase data
- Handle edge cases (missing data, empty sessions)

### Phase 5: Future Enhancement - Map View (4-6 hours)
- Add GPS fields to telemetry schema
- Integrate map library (react-leaflet or mapbox-gl)
- Create MapView component
- Display route on map
- Link map points to telemetry charts
- Add location markers

## Testing Strategy

### Manual Testing Checklist

#### Session List
- [ ] Empty state displays correctly when no sessions exist
- [ ] Sessions load and display with correct metadata
- [ ] Search/filter works correctly
- [ ] Inline name editing saves successfully
- [ ] Delete confirmation dialog appears
- [ ] Session deletion removes from list and database

#### Session Detail
- [ ] Detail page loads when clicking session
- [ ] All available OBD-II parameters display as charts
- [ ] Charts show correct data (values and timeline)
- [ ] Zoom works on individual charts
- [ ] Zoom synchronizes across all charts
- [ ] Reset zoom button works
- [ ] Tooltips show exact values on hover
- [ ] Back button returns to list

#### Edge Cases
- [ ] No Supabase connection → Shows error message
- [ ] Session with no telemetry data → Shows "No data" message
- [ ] Session with partial data (some PIDs missing) → Shows available data only
- [ ] Very large session (10K+ points) → Charts render smoothly
- [ ] Very small session (<10 points) → Charts still display correctly

### Performance Testing
- [ ] List with 50 sessions loads in <2 seconds
- [ ] Detail page with 5000 data points renders in <3 seconds
- [ ] Chart zoom/pan is smooth (no lag)
- [ ] Search/filter is instant (<100ms)

## Success Criteria

### Backend ✅
- [ ] All Tauri commands work correctly
- [ ] Session CRUD operations complete successfully
- [ ] Errors are handled gracefully with meaningful messages
- [ ] Code passes `cargo clippy` and `cargo fmt`

### Frontend ✅
- [ ] History page displays list of sessions
- [ ] Users can name/rename sessions
- [ ] Users can delete sessions
- [ ] Detail page shows all OBD-II parameters as charts
- [ ] Charts are interactive with zoom and tooltips
- [ ] UI is responsive and matches design system
- [ ] Navigation works correctly
- [ ] Loading and error states are clear

### User Experience ✅
- [ ] Users can easily browse their session history
- [ ] Users can identify sessions by custom names
- [ ] Users can filter sessions to find specific drives
- [ ] Users can visualize all telemetry data clearly
- [ ] Users can zoom in for detailed analysis
- [ ] Users can delete unwanted sessions
- [ ] Interface is intuitive and requires no training

## Timeline Estimate

- Phase 1 (Backend): 2-3 hours
- Phase 2 (Session List): 2-3 hours
- Phase 3 (Visualization): 3-4 hours
- Phase 4 (Integration): 1-2 hours
- **Phase 5 (Map View): 4-6 hours (future)**

**Total (without map): 8-12 hours of focused work**
**Total (with map): 12-18 hours of focused work**

## Future Enhancements

1. **Map View** (Phase 5)
   - GPS-tracked route visualization
   - Click map point to see telemetry at that location
   - Route replay animation

2. **Session Comparison**
   - Select 2+ sessions
   - Overlay charts for comparison
   - Side-by-side stats

3. **Export & Sharing**
   - Export session data as CSV
   - Generate PDF report with charts
   - Share session link with others

4. **Advanced Analytics**
   - Acceleration/deceleration metrics
   - Fuel efficiency calculations
   - Driving score/rating
   - Anomaly detection

5. **Session Tags & Categories**
   - Tag sessions (e.g., "track day", "commute", "road trip")
   - Filter by tags
   - Organize into folders

## Notes

- This feature completes **User Story 2** (Data Recording & Retrieval)
- Builds on existing Supabase infrastructure
- Reuses existing UI components and styling patterns
- Recharts library already integrated in codebase
- Can be implemented incrementally (list first, then charts, then map)
- Session naming follows dyno mode pattern ("Roope's BMW M3")
- All ~10 sessions per customer easily manageable in UI
- Zoom functionality essential for detailed analysis
- Map view is highly desired but requires GPS data collection first
