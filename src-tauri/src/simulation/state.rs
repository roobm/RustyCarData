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

pub struct SimulationState {
    pub rpm: Mutex<f64>,
    pub speed: Mutex<f64>,
    pub coolant_temp: Mutex<f64>,
    pub is_running: Mutex<bool>,
    pub is_sniffing: Mutex<bool>,
    pub stats: Mutex<SessionStats>,
}

impl SimulationState {
    pub fn new() -> Self {
        Self {
            rpm: Mutex::new(0.0),
            speed: Mutex::new(0.0),
            coolant_temp: Mutex::new(20.0), // Start at ambient temp
            is_running: Mutex::new(false),
            is_sniffing: Mutex::new(false),
            stats: Mutex::new(SessionStats::new()),
        }
    }
}
