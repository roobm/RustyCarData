use tauri::{State, AppHandle};
use std::sync::Arc;
use crate::simulation::state::SimulationState;
use crate::storage::manager::SessionManager;
use serde_json::json;
use std::path::PathBuf;

#[tauri::command]
pub fn start_simulation(state: State<'_, Arc<SimulationState>>, session_name: Option<String>) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = true;

    // Store session name for later use when saving to Supabase
    if let Some(name) = session_name {
        let mut session_name_guard = state.session_name.lock().unwrap();
        *session_name_guard = Some(name.clone());
        println!("Simulation started with session name: {}", name);
    } else {
        println!("Simulation started");
    }
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

#[tauri::command]
pub fn get_session_history(app: AppHandle) -> Vec<String> {
    let manager = SessionManager::new(&app);
    manager.list_sessions()
        .into_iter()
        .filter_map(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
        .collect()
}

#[tauri::command]
pub fn import_file(app: AppHandle, file_path: String) -> Result<String, String> {
    let manager = SessionManager::new(&app);
    let path = PathBuf::from(file_path);
    
    if !path.exists() {
        return Err("File does not exist".to_string());
    }

    match manager.import_csv(path) {
        Ok(new_path) => Ok(new_path.file_name().unwrap().to_string_lossy().to_string()),
        Err(e) => Err(format!("Import failed: {}", e)),
    }
}

#[tauri::command]
pub fn save_export_csv(_app: AppHandle, csv_content: String, filename: String) -> Result<String, String> {
    use std::fs;

    // Use project folder instead of app_data_dir
    let exports_dir = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .join("data")
        .join("exports");

    if !exports_dir.exists() {
        fs::create_dir_all(&exports_dir).map_err(|e| e.to_string())?;
    }

    let file_path = exports_dir.join(&filename);
    fs::write(&file_path, csv_content).map_err(|e| e.to_string())?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn analyze_current_session(state: State<'_, Arc<SimulationState>>, api_key: Option<String>) -> Result<String, String> {
    let stats = {
        let stats_guard = state.stats.lock().unwrap();
        // Clone the data we need so we don't hold the lock during the async request
        (
            stats_guard.sample_count,
            stats_guard.total_rpm,
            stats_guard.max_speed,
            stats_guard.high_rpm_events
        )
    };

    let (sample_count, total_rpm, max_speed, high_rpm_events) = stats;

    if sample_count == 0 {
        return Ok("No driving data available yet. Start the engine!".to_string());
    }

    let avg_rpm = total_rpm / sample_count as f64;
    let duration_sec = sample_count as f64 * 0.1; // 10Hz

    // Basic Report (Always generated)
    let mut report = String::new();
    report.push_str(&format!("## Driving Session Analysis\n\n"));
    report.push_str(&format!("- **Duration**: {:.1} seconds\n", duration_sec));
    report.push_str(&format!("- **Max Speed**: {:.1} km/h\n", max_speed));
    report.push_str(&format!("- **Average RPM**: {:.0} rpm\n", avg_rpm));
    report.push_str(&format!("- **High RPM Events**: {}\n\n", high_rpm_events));

    // If API Key is provided, call OpenAI
    if let Some(key) = api_key {
        if !key.trim().is_empty() {
            report.push_str("### 🤖 AI Analysis (OpenAI)\n\n");
            
            let prompt = format!(
                "Analyze this driving session data and give specific advice on fuel efficiency and safety. 
                Data: Duration: {:.1}s, Max Speed: {:.1} km/h, Avg RPM: {:.0}, High RPM Events (>5000): {}.
                Keep it concise and use bullet points.",
                duration_sec, max_speed, avg_rpm, high_rpm_events
            );

            let client = reqwest::Client::new();
            let res = client.post("https://api.openai.com/v1/chat/completions")
                .header("Authorization", format!("Bearer {}", key))
                .header("Content-Type", "application/json")
                .json(&json!({
                    "model": "gpt-3.5-turbo",
                    "messages": [
                        {"role": "system", "content": "You are an expert automotive driving instructor."},
                        {"role": "user", "content": prompt}
                    ],
                    "max_tokens": 150
                }))
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;

            if res.status().is_success() {
                let body: serde_json::Value = res.json().await.map_err(|e| format!("Parse failed: {}", e))?;
                if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
                    report.push_str(content);
                    return Ok(report);
                }
            } else {
                report.push_str(&format!("⚠️ **AI Error**: Failed to call OpenAI API. Status: {}", res.status()));
            }
        }
    }

    // Fallback to local logic if no key or API failed
    report.push_str("### 💡 Local Insights (Mock)\n");
    
    if max_speed > 120.0 {
        report.push_str("⚠️ **Speed Warning**: You exceeded 120 km/h. Please drive safely and adhere to speed limits.\n");
    } else {
        report.push_str("✅ **Speed**: Your speed was within reasonable limits.\n");
    }

    if avg_rpm > 4000.0 || high_rpm_events > 10 {
        report.push_str("⚠️ **Efficiency**: High RPM detected frequently. Consider shifting gears earlier to save fuel.\n");
    } else {
        report.push_str("✅ **Efficiency**: Good gear management. You are driving efficiently.\n");
    }

    Ok(report)
}

#[tauri::command]
pub fn list_sessions(app: AppHandle) -> Result<Vec<String>, String> {
    let manager = SessionManager::new(&app);
    let sessions = manager.list_sessions();
    
    Ok(sessions.iter()
        .filter_map(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .collect())
}

#[tauri::command]
pub fn read_session(app: AppHandle, filename: String) -> Result<String, String> {
    use std::fs;
    
    let manager = SessionManager::new(&app);
    let session_path = manager.data_dir.join(&filename);
    
    if !session_path.exists() {
        return Err("Session file not found".to_string());
    }

    fs::read_to_string(session_path).map_err(|e| e.to_string())
}

// ============================================================================
// Agent Commands
// ============================================================================

use crate::agents::{AgentOrchestrator, AnalysisResult, AgentMetadata};

/// Get list of all registered agents
#[tauri::command]
pub fn list_agents(orchestrator: State<'_, Arc<AgentOrchestrator>>) -> Vec<AgentMetadata> {
    orchestrator.registry().list()
}

/// Analyze a session with specific agents
#[tauri::command]
pub async fn analyze_session_with_agents(
    session_id: String,
    agent_ids: Vec<String>,
    orchestrator: State<'_, Arc<AgentOrchestrator>>,
) -> Result<Vec<AnalysisResult>, String> {
    use crate::agents::types::{AnalysisRequest, VehicleInfo};
    use chrono::Utc;

    // Load session data from Supabase
    let session_data = get_supabase_session_data(session_id.clone()).await?;

    // Convert Supabase data to SessionSummary
    let session_summary = convert_to_session_summary(&session_data)?;

    // Extract vehicle info
    let vehicle_info = VehicleInfo {
        make: session_data.session.vehicle_make.clone().unwrap_or_else(|| "Unknown".to_string()),
        model: session_data.session.vehicle_model.clone().unwrap_or_else(|| "Unknown".to_string()),
        year: session_data.session.vehicle_year.unwrap_or(2020) as u16,
        vin: None,
        engine_type: crate::agents::types::EngineType::GasolineNaturallyAspirated,
        transmission: crate::agents::types::TransmissionType::Automatic,
        fuel_type: crate::agents::types::FuelType::Regular87,
    };

    // Create analysis request
    let request = AnalysisRequest {
        request_id: format!("req_{}", Utc::now().timestamp_millis()),
        session_data: session_summary,
        vehicle_info,
        user_preferences: None,
        historical_context: None,
        force_local: true, // Force local Ollama processing
        timeout_seconds: 120, // Give Ollama enough time to respond
    };

    // Run analysis with orchestrator
    let results = orchestrator.analyze_session(&request, agent_ids).await;

    // Convert results
    results
        .into_iter()
        .map(|r| r.map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()
}

/// Convert Supabase session data to SessionSummary
fn convert_to_session_summary(data: &SessionData) -> Result<crate::agents::types::SessionSummary, String> {
    use crate::agents::types::{Event, EventType};
    use chrono::{DateTime, Utc};

    // Parse timestamps
    let start_time = DateTime::parse_from_rfc3339(&data.session.started_at)
        .map_err(|e| format!("Invalid start time: {}", e))?
        .with_timezone(&Utc);

    let end_time = if let Some(ended_at) = &data.session.ended_at {
        DateTime::parse_from_rfc3339(ended_at)
            .map_err(|e| format!("Invalid end time: {}", e))?
            .with_timezone(&Utc)
    } else {
        Utc::now()
    };

    // Calculate statistics from telemetry
    let telemetry = &data.telemetry;
    let stats = &data.stats;

    // Calculate derived metrics from telemetry
    let mut max_speed_mph = 0.0_f32;
    let mut max_rpm = 0.0_f32;
    let mut max_coolant_temp_f = 0.0_f32;
    let mut avg_coolant_temp_sum = 0.0_f64;
    let mut avg_oil_temp_sum = 0.0_f64;
    let mut coolant_temp_count = 0;
    let mut oil_temp_count = 0;

    let mut hard_braking_events = Vec::new();
    let mut rapid_acceleration_events = Vec::new();

    // Analyze telemetry points
    let mut prev_speed: Option<f64> = None;
    for point in telemetry {
        // Speed
        if let Some(speed_kmh) = point.vehicle_speed {
            let speed_mph = (speed_kmh * 0.621371) as f32;
            max_speed_mph = max_speed_mph.max(speed_mph);

            // Detect hard braking (speed drop > 20 km/h in one sample)
            if let Some(prev) = prev_speed {
                let delta = prev - speed_kmh;
                if delta > 20.0 {
                    if let Some(ts_str) = &point.timestamp {
                        if let Ok(timestamp) = DateTime::parse_from_rfc3339(ts_str) {
                            hard_braking_events.push(Event {
                                timestamp: timestamp.with_timezone(&Utc),
                                event_type: EventType::HardBraking,
                                severity: (delta / 30.0) as f32,
                                value: delta as f32,
                                location: None,
                            });
                        }
                    }
                }
                // Detect rapid acceleration (speed gain > 15 km/h)
                if delta < -15.0 {
                    if let Some(ts_str) = &point.timestamp {
                        if let Ok(timestamp) = DateTime::parse_from_rfc3339(ts_str) {
                            rapid_acceleration_events.push(Event {
                                timestamp: timestamp.with_timezone(&Utc),
                                event_type: EventType::RapidAcceleration,
                                severity: (delta.abs() / 20.0) as f32,
                                value: delta.abs() as f32,
                                location: None,
                            });
                        }
                    }
                }
            }
            prev_speed = Some(speed_kmh);
        }

        // RPM
        if let Some(rpm) = point.engine_rpm {
            max_rpm = max_rpm.max(rpm as f32);
        }

        // Coolant temperature (convert from °C to °F)
        if let Some(temp_c) = point.coolant_temp {
            let temp_f = temp_c * 1.8 + 32.0;
            max_coolant_temp_f = max_coolant_temp_f.max(temp_f as f32);
            avg_coolant_temp_sum += temp_f;
            coolant_temp_count += 1;
        }

        // Oil temperature (using intake air temp as proxy if available)
        if let Some(temp_c) = point.intake_air_temp {
            let temp_f = temp_c * 1.8 + 32.0;
            avg_oil_temp_sum += temp_f;
            oil_temp_count += 1;
        }
    }

    let avg_coolant_temp_f = if coolant_temp_count > 0 {
        (avg_coolant_temp_sum / coolant_temp_count as f64) as f32
    } else {
        195.0 // Default normal temp
    };

    let avg_oil_temp_f = if oil_temp_count > 0 {
        (avg_oil_temp_sum / oil_temp_count as f64) as f32
    } else {
        220.0 // Default normal temp
    };

    // Use stats if available, otherwise calculate from telemetry
    let avg_speed_mph = stats.as_ref()
        .and_then(|s| s.avg_speed)
        .map(|kmh| (kmh * 0.621371) as f32)
        .unwrap_or(0.0);

    let avg_rpm = stats.as_ref()
        .and_then(|s| s.avg_rpm)
        .map(|r| r as f32)
        .unwrap_or(2000.0);

    // Calculate distance (convert km to miles)
    let distance_miles = data.session.distance_km
        .map(|km| (km * 0.621371) as f32)
        .unwrap_or(0.0);

    // Calculate fuel consumption (rough estimate: 1L per 10km for city, convert to gallons)
    let distance_km = data.session.distance_km.unwrap_or(0.0);
    let fuel_consumed_gallons = stats.as_ref()
        .and_then(|s| s.fuel_consumed_liters)
        .map(|liters| (liters * 0.264172) as f32)
        .unwrap_or_else(|| ((distance_km / 10.0) * 0.264172) as f32);

    let avg_fuel_efficiency_mpg = if fuel_consumed_gallons > 0.0 {
        distance_miles / fuel_consumed_gallons
    } else {
        25.0 // Default MPG
    };

    // Count high RPM duration (estimate: count points above 5000 RPM, multiply by 0.1s per point)
    let high_rpm_count = telemetry.iter()
        .filter(|p| p.engine_rpm.unwrap_or(0.0) > 5000.0)
        .count();
    let high_rpm_duration_seconds = (high_rpm_count as f64 * 0.1) as u64;

    Ok(crate::agents::types::SessionSummary {
        session_id: data.session.id.unwrap().to_string(),
        start_time,
        end_time,
        duration_seconds: data.session.duration_seconds.unwrap_or(0) as u64,
        distance_miles,
        fuel_consumed_gallons,
        avg_fuel_efficiency_mpg,
        avg_speed_mph,
        max_speed_mph,
        speed_violations_count: 0, // TODO: Calculate based on speed limit
        avg_rpm,
        max_rpm,
        high_rpm_duration_seconds,
        avg_coolant_temp_f,
        max_coolant_temp_f,
        avg_oil_temp_f,
        hard_braking_events,
        rapid_acceleration_events,
        error_codes: Vec::new(), // No DTCs in dataset
        sensor_readings: std::collections::HashMap::new(),
    })
}

/// Get agent metadata by ID
#[tauri::command]
pub fn get_agent_info(
    agent_id: String,
    orchestrator: State<'_, Arc<AgentOrchestrator>>,
) -> Result<AgentMetadata, String> {
    orchestrator
        .registry()
        .get(&agent_id)
        .map(|agent| agent.metadata())
        .map_err(|e| e.to_string())
}

/// Clear agent analysis cache
#[tauri::command]
pub async fn clear_agent_cache(orchestrator: State<'_, Arc<AgentOrchestrator>>) -> Result<(), String> {
    orchestrator.clear_cache().await;
    Ok(())
}

/// Invalidate cache for a specific session
#[tauri::command]
pub async fn invalidate_session_cache(
    session_id: String,
    orchestrator: State<'_, Arc<AgentOrchestrator>>,
) -> Result<(), String> {
    orchestrator.invalidate_cache(&session_id).await;
    Ok(())
}

// ============================================================================
// Playback Commands
// ============================================================================

/// Get current simulation mode (Playback or RandomWalk)
#[tauri::command]
pub fn get_simulation_mode(state: State<'_, Arc<SimulationState>>) -> String {
    use crate::simulation::state::SimulationMode;
    let mode = *state.mode.lock().unwrap();
    match mode {
        SimulationMode::Playback => "Playback".to_string(),
        SimulationMode::RandomWalk => "RandomWalk".to_string(),
    }
}

/// Set simulation mode
#[tauri::command]
pub fn set_simulation_mode(state: State<'_, Arc<SimulationState>>, mode: String) -> Result<(), String> {
    use crate::simulation::state::SimulationMode;
    let mut current_mode = state.mode.lock().unwrap();
    *current_mode = match mode.as_str() {
        "Playback" => SimulationMode::Playback,
        "RandomWalk" => SimulationMode::RandomWalk,
        _ => return Err("Invalid mode. Use 'Playback' or 'RandomWalk'".to_string()),
    };
    Ok(())
}

/// Get current dataset ID
#[tauri::command]
pub fn get_current_dataset(state: State<'_, Arc<SimulationState>>) -> Option<String> {
    state.current_dataset.lock().unwrap().clone()
}

/// Set current dataset for playback
#[tauri::command]
pub fn set_current_dataset(state: State<'_, Arc<SimulationState>>, dataset_id: String) -> Result<(), String> {
    let mut current = state.current_dataset.lock().unwrap();
    *current = Some(dataset_id);
    Ok(())
}

/// Get playback speed multiplier
#[tauri::command]
pub fn get_playback_speed(state: State<'_, Arc<SimulationState>>) -> f64 {
    *state.playback_speed.lock().unwrap()
}

/// Set playback speed (0.1x to 10x)
#[tauri::command]
pub fn set_playback_speed(state: State<'_, Arc<SimulationState>>, speed: f64) -> Result<(), String> {
    if speed < 0.1 || speed > 10.0 {
        return Err("Speed must be between 0.1 and 10.0".to_string());
    }
    let mut current_speed = state.playback_speed.lock().unwrap();
    *current_speed = speed;
    Ok(())
}

/// Get list of available datasets
#[tauri::command]
pub fn list_datasets() -> Vec<serde_json::Value> {
    vec![
        json!({
            "id": "city_commute_1",
            "name": "City Commute - Work to Home",
            "vehicle": "Toyota Etios 2014",
            "style": "Stop-and-go traffic",
            "duration": "64 min",
            "samples": 3853
        }),
        json!({
            "id": "highway_drive_1",
            "name": "Highway Driving - High Speed",
            "vehicle": "Toyota Etios 2014",
            "style": "Highway cruise",
            "duration": "45 min",
            "samples": 2709
        }),
        json!({
            "id": "mixed_long_1",
            "name": "Extended Mixed Driving",
            "vehicle": "Toyota Etios 2014",
            "style": "City and highway mix",
            "duration": "59 min",
            "samples": 3528
        }),
    ]
}

// ============================================================================
// Supabase Session History Commands
// ============================================================================

use crate::storage::supabase::{SupabaseClient, SessionRow, TelemetryRow, SessionStatsRow};
use uuid::Uuid;

/// Combined session data for frontend
#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionData {
    pub session: SessionRow,
    pub telemetry: Vec<TelemetryRow>,
    pub stats: Option<SessionStatsRow>,
}

/// List all sessions from Supabase
#[tauri::command]
pub async fn list_supabase_sessions(limit: Option<usize>) -> Result<Vec<SessionRow>, String> {
    let client = SupabaseClient::new()
        .map_err(|e| format!("Failed to create Supabase client: {}", e))?;

    let limit = limit.unwrap_or(50);
    client.list_sessions(limit)
        .await
        .map_err(|e| format!("Failed to list sessions: {}", e))
}

/// Get full session data including telemetry and stats
#[tauri::command]
pub async fn get_supabase_session_data(session_id: String) -> Result<SessionData, String> {
    let client = SupabaseClient::new()
        .map_err(|e| format!("Failed to create Supabase client: {}", e))?;

    let id = Uuid::parse_str(&session_id)
        .map_err(|e| format!("Invalid session ID: {}", e))?;

    // Fetch session metadata
    let session = client.get_session(id)
        .await
        .map_err(|e| format!("Failed to get session: {}", e))?
        .ok_or_else(|| "Session not found".to_string())?;

    // Fetch telemetry data (limit to 10,000 points for UI performance)
    let telemetry = client.get_session_telemetry(id, Some(10000))
        .await
        .map_err(|e| format!("Failed to get telemetry: {}", e))?;

    // Fetch stats (optional, may not exist)
    let stats = client.get_session_stats(id)
        .await
        .map_err(|e| format!("Failed to get stats: {}", e))?;

    Ok(SessionData {
        session,
        telemetry,
        stats,
    })
}

/// Update session name
#[tauri::command]
pub async fn update_supabase_session_name(
    session_id: String,
    name: String,
) -> Result<(), String> {
    let client = SupabaseClient::new()
        .map_err(|e| format!("Failed to create Supabase client: {}", e))?;

    let id = Uuid::parse_str(&session_id)
        .map_err(|e| format!("Invalid session ID: {}", e))?;

    client.update_session_name(id, name)
        .await
        .map_err(|e| format!("Failed to update session name: {}", e))
}

/// Delete session and all related data
#[tauri::command]
pub async fn delete_supabase_session(session_id: String) -> Result<(), String> {
    let client = SupabaseClient::new()
        .map_err(|e| format!("Failed to create Supabase client: {}", e))?;

    let id = Uuid::parse_str(&session_id)
        .map_err(|e| format!("Invalid session ID: {}", e))?;

    client.delete_session(id)
        .await
        .map_err(|e| format!("Failed to delete session: {}", e))
}
