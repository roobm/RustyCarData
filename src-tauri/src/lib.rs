mod models;
mod simulation;
mod commands;
mod can;
mod storage;

use std::sync::Arc;
use simulation::state::SimulationState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let simulation_state = Arc::new(SimulationState::new());
    let state_clone = simulation_state.clone();
    let can_state_clone = simulation_state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(simulation_state)
        .invoke_handler(tauri::generate_handler![
            commands::start_simulation,
            commands::stop_simulation,
            commands::start_sniffing,
            commands::stop_sniffing,
            commands::get_simulation_status,
            commands::analyze_current_session,
            commands::get_session_history,
            commands::import_file,
            commands::save_export_csv,
            commands::list_sessions,
            commands::read_session
        ])
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let can_app_handle = app.handle().clone();
            
            // Spawn simulation loop in background
            tauri::async_runtime::spawn(async move {
                simulation::run_simulation_loop(app_handle, state_clone).await;
            });

            // Spawn CAN mock loop in background
            tauri::async_runtime::spawn(async move {
                can::run_mock_can_loop(can_app_handle, can_state_clone).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


