-- Supabase Database Schema for OBD-II Telemetry System
-- Feature: 010-simulation-improvements
-- Run this in your Supabase SQL Editor

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Sessions table: Stores driving session metadata
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    session_name TEXT NOT NULL,
    vehicle_info JSONB DEFAULT '{}'::jsonb,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    duration_seconds INTEGER,
    distance_miles FLOAT,
    dataset_source TEXT NOT NULL DEFAULT 'live',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add indexes for common queries
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_created_at ON sessions(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_sessions_dataset_source ON sessions(dataset_source);

-- Telemetry points table: Stores individual sensor readings
CREATE TABLE IF NOT EXISTS telemetry_points (
    id BIGSERIAL PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    timestamp BIGINT NOT NULL,
    pid TEXT NOT NULL,
    value FLOAT NOT NULL,
    unit TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_telemetry_session_id ON telemetry_points(session_id);
CREATE INDEX IF NOT EXISTS idx_telemetry_session_time ON telemetry_points(session_id, timestamp);
CREATE INDEX IF NOT EXISTS idx_telemetry_pid ON telemetry_points(pid);

-- Session statistics table: Stores aggregated session metrics
CREATE TABLE IF NOT EXISTS session_stats (
    session_id UUID PRIMARY KEY REFERENCES sessions(id) ON DELETE CASCADE,
    max_rpm FLOAT DEFAULT 0,
    avg_rpm FLOAT DEFAULT 0,
    max_speed FLOAT DEFAULT 0,
    avg_speed FLOAT DEFAULT 0,
    fuel_consumed FLOAT DEFAULT 0,
    avg_fuel_efficiency FLOAT DEFAULT 0,
    hard_braking_events INTEGER DEFAULT 0,
    rapid_accel_events INTEGER DEFAULT 0,
    error_codes JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Add index for session lookup
CREATE INDEX IF NOT EXISTS idx_session_stats_session_id ON session_stats(session_id);

-- Row Level Security (RLS) Policies
-- Enable RLS on all tables
ALTER TABLE sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE telemetry_points ENABLE ROW LEVEL SECURITY;
ALTER TABLE session_stats ENABLE ROW LEVEL SECURITY;

-- Policy: Users can view their own sessions
CREATE POLICY "Users can view own sessions"
    ON sessions FOR SELECT
    USING (auth.uid() = user_id OR user_id IS NULL);

-- Policy: Users can insert their own sessions
CREATE POLICY "Users can insert own sessions"
    ON sessions FOR INSERT
    WITH CHECK (auth.uid() = user_id OR user_id IS NULL);

-- Policy: Users can update their own sessions
CREATE POLICY "Users can update own sessions"
    ON sessions FOR UPDATE
    USING (auth.uid() = user_id OR user_id IS NULL);

-- Policy: Users can delete their own sessions
CREATE POLICY "Users can delete own sessions"
    ON sessions FOR DELETE
    USING (auth.uid() = user_id OR user_id IS NULL);

-- Policy: Users can view telemetry for their sessions
CREATE POLICY "Users can view own telemetry"
    ON telemetry_points FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM sessions
            WHERE sessions.id = telemetry_points.session_id
            AND (sessions.user_id = auth.uid() OR sessions.user_id IS NULL)
        )
    );

-- Policy: Users can insert telemetry for their sessions
CREATE POLICY "Users can insert own telemetry"
    ON telemetry_points FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM sessions
            WHERE sessions.id = telemetry_points.session_id
            AND (sessions.user_id = auth.uid() OR sessions.user_id IS NULL)
        )
    );

-- Policy: Users can view stats for their sessions
CREATE POLICY "Users can view own stats"
    ON session_stats FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM sessions
            WHERE sessions.id = session_stats.session_id
            AND (sessions.user_id = auth.uid() OR sessions.user_id IS NULL)
        )
    );

-- Policy: Users can insert/update stats for their sessions
CREATE POLICY "Users can insert own stats"
    ON session_stats FOR INSERT
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM sessions
            WHERE sessions.id = session_stats.session_id
            AND (sessions.user_id = auth.uid() OR sessions.user_id IS NULL)
        )
    );

CREATE POLICY "Users can update own stats"
    ON session_stats FOR UPDATE
    USING (
        EXISTS (
            SELECT 1 FROM sessions
            WHERE sessions.id = session_stats.session_id
            AND (sessions.user_id = auth.uid() OR sessions.user_id IS NULL)
        )
    );

-- Function: Update updated_at timestamp automatically
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger: Auto-update sessions.updated_at
CREATE TRIGGER update_sessions_updated_at
    BEFORE UPDATE ON sessions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Trigger: Auto-update session_stats.updated_at
CREATE TRIGGER update_session_stats_updated_at
    BEFORE UPDATE ON session_stats
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Sample data for testing (optional - uncomment to use)
-- INSERT INTO sessions (session_name, vehicle_info, start_time, dataset_source)
-- VALUES (
--     'Test City Commute',
--     '{"make": "Toyota", "model": "Etios", "year": 2014}'::jsonb,
--     NOW() - INTERVAL '1 hour',
--     'bundled'
-- );

COMMENT ON TABLE sessions IS 'Stores driving session metadata including vehicle info and timing';
COMMENT ON TABLE telemetry_points IS 'Individual sensor readings (PIDs) with timestamps';
COMMENT ON TABLE session_stats IS 'Aggregated statistics for each session';

COMMENT ON COLUMN sessions.dataset_source IS 'Source of data: bundled, imported, or live';
COMMENT ON COLUMN telemetry_points.timestamp IS 'Microseconds since UNIX epoch';
COMMENT ON COLUMN telemetry_points.pid IS 'Parameter ID: EngineRPM, VehicleSpeed, CoolantTemp, etc.';
