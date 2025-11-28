mod agents;
mod models;
mod simulation;
mod commands;
mod can;
pub mod storage;  // Public for binary utilities

use std::sync::Arc;
use simulation::state::SimulationState;
use agents::{AgentRegistry, AgentOrchestrator};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let simulation_state = Arc::new(SimulationState::new());
    let state_clone = simulation_state.clone();
    let can_state_clone = simulation_state.clone();

    // Initialize agent orchestrator with default agents
    let registry = AgentRegistry::with_default_agents();
    let orchestrator = Arc::new(AgentOrchestrator::new(registry));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(simulation_state)
        .manage(orchestrator)
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
            commands::read_session,
            commands::list_agents,
            commands::analyze_session_with_agents,
            commands::get_agent_info,
            commands::clear_agent_cache,
            commands::invalidate_session_cache,
            commands::get_simulation_mode,
            commands::set_simulation_mode,
            commands::get_current_dataset,
            commands::set_current_dataset,
            commands::get_playback_speed,
            commands::set_playback_speed,
            commands::list_datasets,
            commands::list_supabase_sessions,
            commands::get_supabase_session_data,
            commands::update_supabase_session_name,
            commands::delete_supabase_session
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


