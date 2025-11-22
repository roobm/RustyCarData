use tauri::State;
use std::sync::Arc;
use crate::simulation::state::SimulationState;

#[tauri::command]
pub fn start_simulation(state: State<'_, Arc<SimulationState>>) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = true;
    println!("Simulation started");
}

#[tauri::command]
pub fn stop_simulation(state: State<'_, Arc<SimulationState>>) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = false;
    println!("Simulation stopped");
}

#[tauri::command]
pub fn start_sniffing(state: State<'_, Arc<SimulationState>>) {
    let mut is_sniffing = state.is_sniffing.lock().unwrap();
    *is_sniffing = true;
    println!("CAN Sniffing started");
}

#[tauri::command]
pub fn stop_sniffing(state: State<'_, Arc<SimulationState>>) {
    let mut is_sniffing = state.is_sniffing.lock().unwrap();
    *is_sniffing = false;
    println!("CAN Sniffing stopped");
}

#[tauri::command]
pub fn get_simulation_status(state: State<'_, Arc<SimulationState>>) -> bool {
    *state.is_running.lock().unwrap()
}

// Placeholder for future AI analysis
#[tauri::command]
pub fn analyze_current_session() -> String {
    "AI Analysis: Driving style is aggressive. Average RPM: 3500. Recommendation: Shift earlier.".to_string()
}
