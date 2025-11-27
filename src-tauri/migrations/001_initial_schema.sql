-- Supabase Database Schema for OBD-II Telemetry Storage
-- Run this in your Supabase SQL Editor: https://app.supabase.com/project/_/sql

-- ============================================================================
-- Table: sessions
-- Stores driving session metadata
-- ============================================================================
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ,
    
    -- Session identification
    session_name VARCHAR(255),
    dataset_source VARCHAR(100), -- 'playback', 'random_walk', 'imported', 'live'
    
    -- Vehicle info (optional)
    vehicle_make VARCHAR(100),
    vehicle_model VARCHAR(100),
    vehicle_year INT,
    
    -- Session metadata
    duration_seconds INT,
    distance_km DECIMAL(10, 2),
    
    -- Indexes for faster queries
    CONSTRAINT sessions_ended_after_started CHECK (ended_at IS NULL OR ended_at > started_at)
);

CREATE INDEX idx_sessions_created_at ON sessions(created_at DESC);
CREATE INDEX idx_sessions_dataset_source ON sessions(dataset_source);

-- ============================================================================
-- Table: telemetry_points
-- Stores individual OBD-II parameter readings
-- ============================================================================
CREATE TABLE IF NOT EXISTS telemetry_points (
    id BIGSERIAL PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- OBD-II Parameters (PIDs)
    -- Core parameters
    engine_rpm DECIMAL(10, 2),
    vehicle_speed DECIMAL(10, 2),
    coolant_temp DECIMAL(10, 2),
    
    -- Extended parameters
    throttle_position DECIMAL(10, 2),
    engine_load DECIMAL(10, 2),
    fuel_level DECIMAL(10, 2),
    intake_air_temp DECIMAL(10, 2),
    timing_advance DECIMAL(10, 2),
    battery_voltage DECIMAL(10, 2),
    
    -- Additional sensors (future)
    maf_flow_rate DECIMAL(10, 2),
    o2_sensor_voltage DECIMAL(10, 2),
    fuel_pressure DECIMAL(10, 2),
    
    -- Indexes for performance
    CONSTRAINT telemetry_session_fk FOREIGN KEY (session_id) REFERENCES sessions(id)
);

CREATE INDEX idx_telemetry_session_id ON telemetry_points(session_id);
CREATE INDEX idx_telemetry_timestamp ON telemetry_points(timestamp DESC);
CREATE INDEX idx_telemetry_session_timestamp ON telemetry_points(session_id, timestamp);

-- ============================================================================
-- Table: session_stats
-- Aggregated statistics per session (for quick access)
-- ============================================================================
CREATE TABLE IF NOT EXISTS session_stats (
    session_id UUID PRIMARY KEY REFERENCES sessions(id) ON DELETE CASCADE,
    
    -- Aggregated statistics
    avg_speed DECIMAL(10, 2),
    max_speed DECIMAL(10, 2),
    avg_rpm DECIMAL(10, 2),
    max_rpm DECIMAL(10, 2),
    avg_coolant_temp DECIMAL(10, 2),
    max_coolant_temp DECIMAL(10, 2),
    
    -- Fuel efficiency
    avg_throttle_position DECIMAL(10, 2),
    avg_engine_load DECIMAL(10, 2),
    fuel_consumed_liters DECIMAL(10, 2),
    
    -- Data quality
    total_points INT NOT NULL DEFAULT 0,
    missing_values_count INT NOT NULL DEFAULT 0,
    
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- Row Level Security (RLS) Policies
-- Optional: Enable if you want user-based access control
-- ============================================================================

-- Enable RLS on tables (uncomment if needed)
-- ALTER TABLE sessions ENABLE ROW LEVEL SECURITY;
-- ALTER TABLE telemetry_points ENABLE ROW LEVEL SECURITY;
-- ALTER TABLE session_stats ENABLE ROW LEVEL SECURITY;

-- Example policy: Users can only see their own sessions
-- CREATE POLICY "Users can view own sessions" ON sessions
--     FOR SELECT USING (auth.uid() = user_id);

-- ============================================================================
-- Helper Functions
-- ============================================================================

-- Function to calculate session statistics
CREATE OR REPLACE FUNCTION calculate_session_stats(p_session_id UUID)
RETURNS VOID AS $$
BEGIN
    INSERT INTO session_stats (
        session_id,
        avg_speed, max_speed,
        avg_rpm, max_rpm,
        avg_coolant_temp, max_coolant_temp,
        avg_throttle_position, avg_engine_load,
        total_points
    )
    SELECT
        p_session_id,
        AVG(vehicle_speed), MAX(vehicle_speed),
        AVG(engine_rpm), MAX(engine_rpm),
        AVG(coolant_temp), MAX(coolant_temp),
        AVG(throttle_position), AVG(engine_load),
        COUNT(*)
    FROM telemetry_points
    WHERE session_id = p_session_id
    ON CONFLICT (session_id) DO UPDATE SET
        avg_speed = EXCLUDED.avg_speed,
        max_speed = EXCLUDED.max_speed,
        avg_rpm = EXCLUDED.avg_rpm,
        max_rpm = EXCLUDED.max_rpm,
        avg_coolant_temp = EXCLUDED.avg_coolant_temp,
        max_coolant_temp = EXCLUDED.max_coolant_temp,
        avg_throttle_position = EXCLUDED.avg_throttle_position,
        avg_engine_load = EXCLUDED.avg_engine_load,
        total_points = EXCLUDED.total_points,
        updated_at = NOW();
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Triggers
-- ============================================================================

-- Automatically update session end time when last telemetry point is added
CREATE OR REPLACE FUNCTION update_session_end_time()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE sessions
    SET ended_at = NEW.timestamp
    WHERE id = NEW.session_id
      AND (ended_at IS NULL OR ended_at < NEW.timestamp);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_session_end_time
    AFTER INSERT ON telemetry_points
    FOR EACH ROW
    EXECUTE FUNCTION update_session_end_time();

-- ============================================================================
-- Sample Queries (for reference)
-- ============================================================================

-- Get all sessions with their stats
-- SELECT s.*, st.*
-- FROM sessions s
-- LEFT JOIN session_stats st ON s.id = st.session_id
-- ORDER BY s.created_at DESC
-- LIMIT 10;

-- Get telemetry data for a specific session
-- SELECT *
-- FROM telemetry_points
-- WHERE session_id = 'your-session-id'
-- ORDER BY timestamp ASC;

-- Calculate stats for a session
-- SELECT calculate_session_stats('your-session-id');
