# Feature Specification: Real OBD-II Data Integration

**Feature ID**: 010-simulation-improvements
**Branch**: `010-simulation-improvements`
**Priority**: P1 (Critical - makes app realistic and usable)
**Status**: Planning

## Overview

Replace the fake random-walk simulation with **real OBD-II data playback**. Ship with actual driving datasets, allow users to import their own data, and store sessions in Supabase for cloud access and analysis.

## Motivation

**Current State:**
- Fake random walk simulation (unrealistic)
- Only 3 PIDs with random values
- No real driving patterns
- Agent analysis on fake data is meaningless
- Parquet files only stored locally

**Problem:**
- App feels like a tech demo, not a real tool
- Can't showcase actual capabilities with real data
- Users can't trust the agent analysis
- No way to share or backup sessions

**Solution:**
- Use real OBD-II datasets from actual cars
- Play back real driving sessions
- Store in Supabase for cloud access
- Allow import of user's own data

## Goals

1. **Ship with Real Data**: Include 4-5 real driving session datasets
2. **Data Import**: Users can upload CSV files from their OBD-II logger
3. **Supabase Storage**: Store sessions in cloud database
4. **Realistic Playback**: Play real data through the system
5. **Agent Validation**: Agents analyze real driving patterns

## Non-Goals

- Real-time OBD-II hardware connection (future feature)
- Live vehicle simulation (replaced with playback)
- Video recording of drives
- GPS route mapping (can add with dataset metadata)

## User Stories

### US1: Realistic Demo Sessions
**As a** user or demonstrator
**I want** to play back real driving sessions from actual cars
**So that** the analysis feels authentic and trustworthy

**Acceptance Criteria:**
- ✅ App ships with 4+ real OBD-II datasets
- ✅ Each dataset represents different driving style (city, highway, aggressive, eco)
- ✅ Datasets include 10+ PIDs with real correlations
- ✅ Playback shows realistic sensor behavior

### US2: Import Custom Data
**As a** user with OBD-II data
**I want** to import my own driving logs
**So that** I can analyze my actual driving behavior

**Acceptance Criteria:**
- ✅ Import CSV files with OBD-II data
- ✅ Auto-detect column mapping (RPM, Speed, etc.)
- ✅ Validate data format and ranges
- ✅ Preview data before importing
- ✅ Store imported data in Supabase

### US3: Cloud Storage
**As a** user
**I want** my driving sessions stored in the cloud
**So that** I can access them from any device and share with others

**Acceptance Criteria:**
- ✅ Sessions stored in Supabase (not just local Parquet)
- ✅ Can view session history from cloud
- ✅ Can download sessions
- ✅ Can share session links
- ✅ Local Parquet as backup/offline mode

### US4: Agent Analysis on Real Data
**As a** developer
**I want** agents to analyze real driving data
**So that** I can validate agent accuracy and improve algorithms

**Acceptance Criteria:**
- ✅ Agents produce meaningful scores on real data
- ✅ Different driving styles get different scores
- ✅ Insights match actual driving events in data
- ✅ Recommendations are relevant to real behavior

## Data Sources

### Bundled Datasets (Ship with App)

**Dataset 1: City Commute (Toyota Etios)**
- Source: [carOBD GitHub](https://github.com/eron93br/carOBD)
- PIDs: 27 parameters
- Duration: Multiple sessions
- Style: City driving, stop-and-go
- License: Open source

**Dataset 2: Highway Driving**
- Source: [Kaggle OBD-II datasets](https://www.kaggle.com/datasets/anwarmehmoodsohail/driving-datasets-obd-iican-bus)
- PIDs: CAN-BUS data
- Duration: ~30-60 min
- Style: Highway cruise
- License: Kaggle dataset license

**Dataset 3: Mixed Driving**
- Source: [KIT Automotive Dataset](https://publikationen.bibliothek.kit.edu/1000085073)
- PIDs: 10 signals (RPM, Speed, Temps, etc.)
- Duration: Multiple sessions
- Style: Mixed city/highway
- License: Academic/research

**Dataset 4: Aggressive Driving** (if available)
- Source: To be sourced or synthesized from real data
- PIDs: Full OBD-II complement
- Duration: 20-30 min
- Style: Hard acceleration, braking, high RPM

### User-Provided Data

Users can import:
- CSV files from OBD-II loggers (Torque, OBD Fusion, etc.)
- BMWLogger exports
- BimmerLink logs
- Any CSV with timestamp + PID columns

## Technical Architecture

### Data Pipeline

```
Real OBD Dataset (CSV)
    ↓
Dataset Loader
    ↓
Playback Engine (100Hz)
    ↓
State Update
    ↓
┌─────────────┬──────────────┐
│ Supabase    │ Parquet      │
│ (Cloud)     │ (Local Cache)│
└─────────────┴──────────────┘
    ↓
Frontend Display + Agent Analysis
```

### Module Structure

```
src-tauri/src/
├── simulation/
│   ├── mod.rs              # Playback loop
│   ├── state.rs            # Enhanced with all PIDs
│   ├── playback.rs         # Play real data
│   └── dataset.rs          # Load & parse CSV
├── storage/
│   ├── supabase.rs         # NEW: Supabase client
│   ├── parquet.rs          # Keep for local cache
│   └── importer.rs         # NEW: Import user CSV
└── commands.rs             # Add import/playback commands
```

### Supabase Schema

```sql
-- Sessions table
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES auth.users,
    session_name TEXT,
    vehicle_info JSONB,
    start_time TIMESTAMPTZ,
    end_time TIMESTAMPTZ,
    duration_seconds INTEGER,
    distance_miles FLOAT,
    dataset_source TEXT,  -- 'bundled', 'imported', 'live'
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Telemetry data
CREATE TABLE telemetry_points (
    id BIGSERIAL PRIMARY KEY,
    session_id UUID REFERENCES sessions(id) ON DELETE CASCADE,
    timestamp BIGINT,  -- microseconds since epoch
    pid TEXT,          -- 'RPM', 'Speed', etc.
    value FLOAT,
    unit TEXT,
    INDEX idx_session_time (session_id, timestamp)
);

-- Session summary stats
CREATE TABLE session_stats (
    session_id UUID PRIMARY KEY REFERENCES sessions(id) ON DELETE CASCADE,
    max_rpm FLOAT,
    avg_rpm FLOAT,
    max_speed FLOAT,
    avg_speed FLOAT,
    fuel_consumed FLOAT,
    avg_fuel_efficiency FLOAT,
    hard_braking_events INTEGER,
    rapid_accel_events INTEGER,
    error_codes JSONB
);
```

### CSV Import Format

Support common OBD-II CSV formats:

```csv
timestamp,rpm,speed,coolant_temp,throttle_pos,maf
1638360000,2500,45.3,89.2,35.0,12.5
1638360100,2600,46.1,89.5,38.0,13.2
...
```

Auto-detect columns:
- Timestamp: `timestamp`, `time`, `t`
- RPM: `rpm`, `engine_rpm`, `engineRPM`
- Speed: `speed`, `vehicle_speed`, `vss`
- Etc.

## Implementation Phases

### Phase 1: Dataset Integration (P1)
- [x] Download 3-4 real OBD-II datasets
- [x] Create dataset loader (CSV parser)
- [x] Map CSV columns to PID types
- [x] Add datasets to `src-tauri/data/` directory
- [x] Create dataset catalog/registry

**Estimated: 2-3 hours**

### Phase 2: Playback Engine (P1)
- [x] Replace simulation loop with playback
- [x] Read dataset sequentially
- [x] Emit telemetry at realistic intervals
- [x] Handle dataset end (loop or stop)
- [x] Add playback controls (play/pause/speed)

**Estimated: 2-3 hours**

### Phase 3: Supabase Integration (P1)
- [x] Set up Supabase project
- [x] Create schema (sessions, telemetry, stats)
- [x] Implement Supabase client in Rust
- [x] Write telemetry to Supabase
- [x] Read sessions from Supabase
- [x] Keep Parquet as local cache

**Estimated: 3-4 hours**

### Phase 4: CSV Import (P2)
- [x] Add import command
- [x] Parse user-uploaded CSV
- [x] Auto-detect column mapping
- [x] Validate data quality
- [x] Convert to internal format
- [x] Store in Supabase

**Estimated: 2-3 hours**

### Phase 5: Frontend Integration (P2)
- [x] Dataset selection UI
- [x] Import CSV button & dialog
- [x] Playback controls (play/pause/speed)
- [x] Session history from Supabase
- [x] Share session links

**Estimated: 3-4 hours**

### Phase 6: Agent Integration (P2)
- [x] Analyze sessions from Supabase
- [x] Store analysis results in Supabase
- [x] Display agent insights for real data
- [x] Validate agent accuracy

**Estimated: 2 hours**

**Total: 14-19 hours**

## Data Quality Considerations

### Validation
- Check PID value ranges (RPM: 0-7000, Speed: 0-200, etc.)
- Verify timestamp monotonicity
- Detect missing/corrupt data
- Warn about incomplete PIDs

### Preprocessing
- Interpolate missing values (linear)
- Smooth noisy sensors
- Calculate derived values (acceleration from speed)
- Detect and label events (hard braking, etc.)

### Metadata
Store with each dataset:
```json
{
  "vehicle": {
    "make": "Toyota",
    "model": "Etios",
    "year": 2014,
    "engine": "Gasoline 1.5L"
  },
  "session": {
    "date": "2023-05-15",
    "location": "City Commute",
    "driver": "Driver A",
    "conditions": "Sunny, dry roads"
  },
  "quality": {
    "pids_available": 27,
    "sample_rate_hz": 10,
    "duration_minutes": 45
  }
}
```

## Success Criteria

✅ **Minimum Viable:**
1. Ship with 3 real driving datasets
2. Can play back real data through system
3. Data stored in Supabase
4. Agents analyze real sessions
5. Import works for common CSV formats

✅ **Quality Metrics:**
- Real data feels authentic (not random)
- Agent scores vary meaningfully across datasets
- Import succeeds for 90%+ of common formats
- Supabase queries < 500ms
- Playback smooth at 10Hz

## Dependencies

**External:**
- Supabase account (free tier is fine)
- Datasets downloaded from Kaggle/GitHub
- Supabase Rust client: `postgrest` or `supabase-rs`

**Internal:**
- Existing Parquet writer (keep as backup)
- Frontend for import UI
- Agent framework for analysis

## Risks & Mitigation

| Risk | Mitigation |
|------|------------|
| Dataset licenses unclear | Use clearly open-source datasets |
| Supabase costs | Use free tier, local Parquet fallback |
| CSV format variations | Auto-detect + manual mapping UI |
| Large datasets slow import | Stream processing, progress bar |
| Missing PIDs in datasets | Fill with defaults, warn user |

## Car Visualization Widget

**Fun Feature**: Small animated car widget showing real-time driving state

### Purpose
Add visual appeal and make the app more engaging by showing a small car animation that reacts to the driving data being played back.

### Design
```
┌─────────────────────┐
│  🚗 → → →           │  ← Accelerating (car tilts forward)
│  Speed: 45 mph      │
│  RPM: 2500          │
└─────────────────────┘
```

### Visual States
- **Idle**: Car stationary, slight idle bounce
- **Accelerating**: Car tilts forward, speed lines appear
- **Cruising**: Smooth animation, steady motion
- **Braking**: Car tilts back, brake lights glow red
- **High RPM**: Engine shakes, red indicator
- **Hard Braking**: Strong tilt, skid marks appear

### Implementation Options
1. **CSS Animations** (Simple, lightweight)
   - SVG car sprite with CSS transforms
   - Rotate/translate based on acceleration/braking
   - Color changes for states (red for braking, etc.)

2. **Canvas 2D** (Moderate complexity)
   - Draw car sprite on canvas
   - Smoother animations with requestAnimationFrame
   - Particle effects for speed/brake dust

3. **React Spring** (Best UX)
   - Physics-based animations
   - Natural transitions between states
   - Easy to bind to telemetry data

### Placement
- **Option 1**: Corner widget (bottom-right)
- **Option 2**: Embedded in telemetry panel
- **Option 3**: Floating draggable widget
- **Preferred**: Bottom-right corner, toggleable

### Data Bindings
```typescript
interface CarAnimationState {
  speed: number;        // 0-120 mph
  rpm: number;          // 0-7000
  acceleration: number; // m/s^2 (positive/negative)
  braking: boolean;     // brake event detected
  throttle: number;     // 0-100%
}
```

### Animation Logic
```typescript
// Tilt angle based on acceleration
const tiltAngle = acceleration * 5; // degrees

// Speed lines based on speed
const speedLineCount = Math.floor(speed / 10);

// Engine shake based on RPM
const shakeIntensity = rpm > 5000 ? (rpm - 5000) / 2000 : 0;

// Brake light intensity
const brakeLightOpacity = braking ? 1.0 : 0.2;
```

### Assets Needed
- Car sprite (side view, ~100x50px)
- Speed line graphics
- Brake light glow effect
- Background road texture (optional)

### Success Criteria
- Widget responds in real-time to playback data (<100ms latency)
- Animations feel natural and fun
- Performance: <5% CPU usage
- Works at all playback speeds (1x, 2x, 5x)
- Can be toggled on/off
- Mobile responsive

### Phase 7: Car Visualization (P3 - Fun!)
- [ ] Design car sprite assets
- [ ] Implement animation state machine
- [ ] Bind telemetry data to animations
- [ ] Add playback speed support
- [ ] Polish transitions and effects
- [ ] Add toggle control

**Estimated: 2-3 hours**

---

## Future Enhancements

- Real-time OBD-II hardware connection
- GPS visualization on maps (car drives on actual route)
- Dataset marketplace (share sessions)
- AI-powered driving coach live mode
- Export sessions to standard formats
- Session comparison tools
- Privacy-preserving data sharing
- 3D car model with WebGL (if 2D widget is popular)
