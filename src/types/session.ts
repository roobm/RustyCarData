// TypeScript interfaces matching Rust backend structures

export interface Session {
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

export interface TelemetryPoint {
  session_id: string;
  timestamp: string;
  // Core parameters
  engine_rpm?: number;
  vehicle_speed?: number;
  coolant_temp?: number;
  // Extended parameters
  throttle_position?: number;
  engine_load?: number;
  fuel_level?: number;
  intake_air_temp?: number;
  timing_advance?: number;
  battery_voltage?: number;
  // Additional sensors
  maf_flow_rate?: number;
  o2_sensor_voltage?: number;
  fuel_pressure?: number;
}

export interface SessionStats {
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
  updated_at?: string;
}

export interface SessionData {
  session: Session;
  telemetry: TelemetryPoint[];
  stats?: SessionStats;
}

// Chart data point format for Recharts
export interface ChartDataPoint {
  timestamp: number; // Unix timestamp for X-axis
  value: number; // Parameter value for Y-axis
  formattedTime?: string; // Human-readable time for tooltip
}
