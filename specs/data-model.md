# Data Model

## Entities

### 1. CanFrame
Represents a single raw CAN bus message.
- `timestamp`: u64 (Unix timestamp in microseconds)
- `id`: u32 (Standard 11-bit or Extended 29-bit CAN ID)
- `data`: [u8; 8] (8 bytes of payload)
- `is_extended`: bool

### 2. ObdPid (Enum)
Supported OBD-II Parameter IDs.
- `EngineRPM` (0x0C)
- `VehicleSpeed` (0x0D)
- `CoolantTemp` (0x05)
- `ThrottlePos` (0x11)

### 3. TelemetryPoint
Processed value from a CAN frame, ready for UI/Storage.
- `timestamp`: u64
- `pid`: ObdPid
- `value`: f64 (Physical value, e.g., 2500.0 for RPM)
- `unit`: String (e.g., "rpm", "km/h")

### 4. DriveSession
A collection of telemetry data.
- `id`: UUID
- `start_time`: DateTime
- `end_time`: DateTime
- `file_path`: String (Path to .parquet file)
- `summary`: String (AI Analysis result)

## Storage Schema (Parquet)

| Column | Type | Description |
|--------|------|-------------|
| timestamp_us | Int64 | Microseconds since epoch |
| pid_id | Int32 | ID of the PID (e.g., 12 for RPM) |
| value | Double | The decoded value |
| raw_bytes | Binary | The original 8 bytes (optional, for debugging) |

## API Contracts (Tauri Commands)

### Frontend -> Backend

- `start_simulation()`: Starts the virtual car loop.
- `stop_simulation()`: Stops the loop.
- `get_session_history()`: Returns list of past DriveSessions.
- `analyze_session(session_id)`: Triggers AI analysis for a specific session.

### Backend -> Frontend (Events)

- `telemetry_update`: Emitted every X ms (e.g., 100ms) with a batch of `TelemetryPoint`s.
- `simulation_status`: Emitted when state changes (Running/Stopped).
