use std::sync::Mutex;

pub struct SimulationState {
    pub rpm: Mutex<f64>,
    pub speed: Mutex<f64>,
    pub coolant_temp: Mutex<f64>,
    pub is_running: Mutex<bool>,
    pub is_sniffing: Mutex<bool>,
}

impl SimulationState {
    pub fn new() -> Self {
        Self {
            rpm: Mutex::new(0.0),
            speed: Mutex::new(0.0),
            coolant_temp: Mutex::new(20.0), // Start at ambient temp
            is_running: Mutex::new(false),
            is_sniffing: Mutex::new(false),
        }
    }
}
