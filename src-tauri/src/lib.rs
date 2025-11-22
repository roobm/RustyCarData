mod models;
mod simulation;
mod commands;

use std::sync::Arc;
use simulation::state::SimulationState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let simulation_state = Arc::new(SimulationState::new());
    let state_clone = simulation_state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(simulation_state)
        .invoke_handler(tauri::generate_handler![
            commands::start_simulation,
            commands::stop_simulation,
            commands::get_simulation_status,
            commands::analyze_current_session
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            
            // Spawn simulation loop in background
            tauri::async_runtime::spawn(async move {
                simulation::run_simulation_loop(app_handle, state_clone).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


