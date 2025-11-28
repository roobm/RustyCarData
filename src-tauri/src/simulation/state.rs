use std::sync::Mutex;

pub struct SessionStats {
    pub max_rpm: f64,
    pub max_speed: f64,
    pub total_rpm: f64,
    pub sample_count: u64,
    pub high_rpm_events: u64, // Times RPM > 5000
}

impl SessionStats {
    pub fn new() -> Self {
        Self {
            max_rpm: 0.0,
            max_speed: 0.0,
            total_rpm: 0.0,
            sample_count: 0,
            high_rpm_events: 0,
        }
    }
}

/// Simulation mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimulationMode {
    /// Random walk simulation (old behavior)
    RandomWalk,
    /// Dataset playback mode
    Playback,
}

pub struct SimulationState {
    // Current values
    pub rpm: Mutex<f64>,
    pub speed: Mutex<f64>,
    pub coolant_temp: Mutex<f64>,

    // New PIDs from real data
    pub throttle_position: Mutex<f64>,
    pub engine_load: Mutex<f64>,
    pub fuel_level: Mutex<f64>,
    pub intake_air_temp: Mutex<f64>,
    pub timing_advance: Mutex<f64>,
    pub battery_voltage: Mutex<f64>,

    // Control flags
    pub is_running: Mutex<bool>,
    pub is_sniffing: Mutex<bool>,

    // Playback control
    pub mode: Mutex<SimulationMode>,
    pub current_dataset: Mutex<Option<String>>,
    pub playback_speed: Mutex<f64>,

    // Session metadata
    pub session_name: Mutex<Option<String>>,

    // Statistics
    pub stats: Mutex<SessionStats>,
}

impl SimulationState {
    pub fn new() -> Self {
        Self {
            rpm: Mutex::new(0.0),
            speed: Mutex::new(0.0),
            coolant_temp: Mutex::new(20.0),

            throttle_position: Mutex::new(0.0),
            engine_load: Mutex::new(0.0),
            fuel_level: Mutex::new(100.0),
            intake_air_temp: Mutex::new(20.0),
            timing_advance: Mutex::new(0.0),
            battery_voltage: Mutex::new(12.6),

            is_running: Mutex::new(false),
            is_sniffing: Mutex::new(false),

            mode: Mutex::new(SimulationMode::Playback),
            current_dataset: Mutex::new(Some("city_commute_1".to_string())),
            playback_speed: Mutex::new(1.0),

            session_name: Mutex::new(None),

            stats: Mutex::new(SessionStats::new()),
        }
    }

    /// Update state from telemetry point
    pub fn update_from_pid(&self, pid: crate::models::ObdPid, value: f64) {
        use crate::models::ObdPid;

        match pid {
            ObdPid::EngineRPM => *self.rpm.lock().unwrap() = value,
            ObdPid::VehicleSpeed => *self.speed.lock().unwrap() = value,
            ObdPid::CoolantTemp => *self.coolant_temp.lock().unwrap() = value,
            ObdPid::ThrottlePosition => *self.throttle_position.lock().unwrap() = value,
            ObdPid::EngineLoad => *self.engine_load.lock().unwrap() = value,
            ObdPid::FuelLevel => *self.fuel_level.lock().unwrap() = value,
            ObdPid::IntakeAirTemp => *self.intake_air_temp.lock().unwrap() = value,
            ObdPid::TimingAdvance => *self.timing_advance.lock().unwrap() = value,
            ObdPid::BatteryVoltage => *self.battery_voltage.lock().unwrap() = value,
            _ => {} // Ignore unmapped PIDs
        }
    }
}
