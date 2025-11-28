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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ObdPid;

    #[test]
    fn test_simulation_state_new() {
        let state = SimulationState::new();

        assert_eq!(*state.rpm.lock().unwrap(), 0.0);
        assert_eq!(*state.speed.lock().unwrap(), 0.0);
        assert_eq!(*state.coolant_temp.lock().unwrap(), 20.0);
        assert_eq!(*state.throttle_position.lock().unwrap(), 0.0);
        assert_eq!(*state.engine_load.lock().unwrap(), 0.0);
        assert_eq!(*state.fuel_level.lock().unwrap(), 100.0);
        assert_eq!(*state.intake_air_temp.lock().unwrap(), 20.0);
        assert_eq!(*state.timing_advance.lock().unwrap(), 0.0);
        assert_eq!(*state.battery_voltage.lock().unwrap(), 12.6);
        assert_eq!(*state.is_running.lock().unwrap(), false);
        assert_eq!(*state.is_sniffing.lock().unwrap(), false);
        assert_eq!(*state.mode.lock().unwrap(), SimulationMode::Playback);
        assert_eq!(*state.playback_speed.lock().unwrap(), 1.0);
        assert_eq!(*state.session_name.lock().unwrap(), None);
    }

    #[test]
    fn test_session_stats_new() {
        let stats = SessionStats::new();

        assert_eq!(stats.max_rpm, 0.0);
        assert_eq!(stats.max_speed, 0.0);
        assert_eq!(stats.total_rpm, 0.0);
        assert_eq!(stats.sample_count, 0);
        assert_eq!(stats.high_rpm_events, 0);
    }

    #[test]
    fn test_update_from_pid_rpm() {
        let state = SimulationState::new();

        state.update_from_pid(ObdPid::EngineRPM, 3000.0);

        assert_eq!(*state.rpm.lock().unwrap(), 3000.0);
    }

    #[test]
    fn test_update_from_pid_speed() {
        let state = SimulationState::new();

        state.update_from_pid(ObdPid::VehicleSpeed, 120.5);

        assert_eq!(*state.speed.lock().unwrap(), 120.5);
    }

    #[test]
    fn test_update_from_pid_coolant_temp() {
        let state = SimulationState::new();

        state.update_from_pid(ObdPid::CoolantTemp, 90.0);

        assert_eq!(*state.coolant_temp.lock().unwrap(), 90.0);
    }

    #[test]
    fn test_update_from_pid_throttle() {
        let state = SimulationState::new();

        state.update_from_pid(ObdPid::ThrottlePosition, 75.5);

        assert_eq!(*state.throttle_position.lock().unwrap(), 75.5);
    }

    #[test]
    fn test_update_from_pid_engine_load() {
        let state = SimulationState::new();

        state.update_from_pid(ObdPid::EngineLoad, 45.2);

        assert_eq!(*state.engine_load.lock().unwrap(), 45.2);
    }

    #[test]
    fn test_update_from_pid_fuel_level() {
        let state = SimulationState::new();

        state.update_from_pid(ObdPid::FuelLevel, 65.0);

        assert_eq!(*state.fuel_level.lock().unwrap(), 65.0);
    }

    #[test]
    fn test_update_from_pid_battery_voltage() {
        let state = SimulationState::new();

        state.update_from_pid(ObdPid::BatteryVoltage, 14.2);

        assert_eq!(*state.battery_voltage.lock().unwrap(), 14.2);
    }

    #[test]
    fn test_simulation_mode_equality() {
        assert_eq!(SimulationMode::Playback, SimulationMode::Playback);
        assert_eq!(SimulationMode::RandomWalk, SimulationMode::RandomWalk);
        assert_ne!(SimulationMode::Playback, SimulationMode::RandomWalk);
    }

    #[test]
    fn test_multiple_updates() {
        let state = SimulationState::new();

        state.update_from_pid(ObdPid::EngineRPM, 1000.0);
        state.update_from_pid(ObdPid::VehicleSpeed, 50.0);
        state.update_from_pid(ObdPid::EngineRPM, 2000.0);

        assert_eq!(*state.rpm.lock().unwrap(), 2000.0);
        assert_eq!(*state.speed.lock().unwrap(), 50.0);
    }
}
