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
pub fn analyze_current_session(state: State<'_, Arc<SimulationState>>) -> String {
    let stats = state.stats.lock().unwrap();
    
    if stats.sample_count == 0 {
        return "No driving data available yet. Start the engine!".to_string();
    }

    let avg_rpm = stats.total_rpm / stats.sample_count as f64;
    let duration_sec = stats.sample_count as f64 * 0.1; // 10Hz

    let mut report = String::new();
    report.push_str(&format!("## Driving Session Analysis\n\n"));
    report.push_str(&format!("- **Duration**: {:.1} seconds\n", duration_sec));
    report.push_str(&format!("- **Max Speed**: {:.1} km/h\n", stats.max_speed));
    report.push_str(&format!("- **Average RPM**: {:.0} rpm\n", avg_rpm));
    report.push_str(&format!("- **High RPM Events**: {}\n\n", stats.high_rpm_events));

    report.push_str("### AI Insights:\n");
    
    if stats.max_speed > 120.0 {
        report.push_str("⚠️ **Speed Warning**: You exceeded 120 km/h. Please drive safely and adhere to speed limits.\n");
    } else {
        report.push_str("✅ **Speed**: Your speed was within reasonable limits.\n");
    }

    if avg_rpm > 4000.0 || stats.high_rpm_events > 10 {
        report.push_str("⚠️ **Efficiency**: High RPM detected frequently. Consider shifting gears earlier to save fuel.\n");
    } else {
        report.push_str("✅ **Efficiency**: Good gear management. You are driving efficiently.\n");
    }

    report
}
