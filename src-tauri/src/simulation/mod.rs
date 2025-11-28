use std::sync::Arc;
use std::time::Duration;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use tokio::time::sleep;
use chrono::Utc;
use uuid::Uuid;
use crate::models::{ObdPid, TelemetryPoint};
use crate::storage::supabase::{SupabaseClient, SessionRow, TelemetryRow};
use self::state::{SimulationState, SimulationMode};
use self::dataset::DatasetLoader;
use self::playback::DatasetPlayback;

pub mod state;
pub mod dataset;
pub mod playback;

pub async fn run_simulation_loop(app: AppHandle, state: Arc<SimulationState>) {
    let mut was_running = false;

    // Initialize Supabase client
    let supabase_client = match SupabaseClient::new() {
        Ok(client) => {
            println!("✅ Supabase client connected successfully");
            Some(client)
        },
        Err(e) => {
            eprintln!("⚠️  Supabase not configured: {}. Data will only be stored locally.", e);
            None
        }
    };

    // Track current cloud session
    let mut current_session_id: Option<Uuid> = None;
    let mut session_start_time: Option<String> = None;
    let mut telemetry_buffer: Vec<TelemetryRow> = Vec::new();

    // Initialize dataset loader
    // Use hard-coded path relative to executable for now
    let data_dir = PathBuf::from("data/datasets");
    let catalog_path = data_dir.join("catalog.json");

    let loader = match DatasetLoader::new(&catalog_path, "data/datasets") {
        Ok(l) => {
            println!("Dataset loader initialized successfully");
            Some(l)
        },
        Err(e) => {
            eprintln!("Failed to init dataset loader: {}. Falling back to random walk.", e);
            None
        }
    };

    // Load initial dataset if in playback mode
    let mut playback: Option<DatasetPlayback> = None;

    loop {
        // Check if simulation should run
        let is_running = *state.is_running.lock().unwrap();

        // Handle Start/Stop transitions
        if is_running && !was_running {
            // Just started - Create Supabase session
            if let Some(ref client) = supabase_client {
                let mode = *state.mode.lock().unwrap();
                let dataset_id = state.current_dataset.lock().unwrap().clone();
                let start_time = Utc::now().to_rfc3339();
                session_start_time = Some(start_time.clone());

                // Get session name from state, or use default
                let session_name = state.session_name.lock().unwrap().clone()
                    .unwrap_or_else(|| format!("Session {}", Utc::now().format("%Y-%m-%d %H:%M")));

                let session = SessionRow {
                    id: None,
                    created_at: None,
                    started_at: start_time,
                    ended_at: None,
                    session_name: Some(session_name),
                    dataset_source: Some(if mode == SimulationMode::Playback { "playback" } else { "random_walk" }.to_string()),
                    vehicle_make: Some("Toyota".to_string()),
                    vehicle_model: Some("Etios".to_string()),
                    vehicle_year: Some(2014),
                    duration_seconds: None,
                    distance_km: None,
                };

                match client.create_session(&session).await {
                    Ok(session_id) => {
                        current_session_id = Some(session_id);
                        println!("✅ Created Supabase session: {}", session_id);
                    },
                    Err(e) => {
                        eprintln!("⚠️  Failed to create Supabase session: {}", e);
                    }
                }
            }

            // Initialize playback if in playback mode
            let mode = *state.mode.lock().unwrap();
            if mode == SimulationMode::Playback {
                if let Some(ref loader) = loader {
                    let dataset_id = state.current_dataset.lock().unwrap().clone()
                        .unwrap_or_else(|| "city_commute_1".to_string());

                    match loader.load_dataset(&dataset_id) {
                        Ok(data) => {
                            println!("Loaded dataset '{}' with {} points", dataset_id, data.len());
                            let mut pb = DatasetPlayback::new(data);
                            pb.play();
                            playback = Some(pb);
                        },
                        Err(e) => {
                            eprintln!("Failed to load dataset: {}", e);
                        }
                    }
                }
            }
        } else if !is_running && was_running {
            // Just stopped - Complete Supabase session
            if let (Some(ref client), Some(session_id), Some(start_time)) =
                (&supabase_client, current_session_id, &session_start_time) {
                // Flush any remaining telemetry
                if !telemetry_buffer.is_empty() {
                    if let Err(e) = client.insert_telemetry_batch(&telemetry_buffer).await {
                        eprintln!("⚠️  Failed to flush telemetry: {}", e);
                    }
                    telemetry_buffer.clear();
                }

                // Calculate duration
                let end_time = Utc::now();
                let start = chrono::DateTime::parse_from_rfc3339(&start_time)
                    .unwrap()
                    .with_timezone(&Utc);
                let duration_seconds = (end_time - start).num_seconds() as i32;

                // Update session
                match client.complete_session(session_id, end_time.to_rfc3339(), duration_seconds).await {
                    Ok(_) => println!("✅ Completed Supabase session: {}", session_id),
                    Err(e) => eprintln!("⚠️  Failed to complete session: {}", e),
                }

                current_session_id = None;
                session_start_time = None;
            }

            playback = None;
        }
        was_running = is_running;

        if !is_running {
            sleep(Duration::from_millis(100)).await;
            continue;
        }

        // Determine mode
        let mode = *state.mode.lock().unwrap();

        let points = if mode == SimulationMode::Playback && playback.is_some() {
            // PLAYBACK MODE: Get next frame from dataset
            if let Some(ref mut pb) = playback {
                if let Some(frame) = pb.next_frame() {
                    // Update state from playback data
                    for point in &frame {
                        state.update_from_pid(point.pid, point.value);
                    }
                    frame
                } else {
                    // Playback ended, create empty frame
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            // RANDOM WALK MODE (fallback)
            simulate_random_walk(&state)
        };

        // Skip if no points generated
        if points.is_empty() {
            sleep(Duration::from_millis(100)).await;
            continue;
        }

        // Update Stats
        {
            let rpm = *state.rpm.lock().unwrap();
            let speed = *state.speed.lock().unwrap();
            let mut stats = state.stats.lock().unwrap();

            if rpm > stats.max_rpm { stats.max_rpm = rpm; }
            if speed > stats.max_speed { stats.max_speed = speed; }
            if rpm > 5000.0 { stats.high_rpm_events += 1; }

            stats.total_rpm += rpm;
            stats.sample_count += 1;
        }

        // Buffer telemetry for Supabase
        if let (Some(session_id), Some(ref client)) = (current_session_id, &supabase_client) {
            // Convert telemetry points to Supabase row format
            // Collect current state snapshot
            let telemetry_row = TelemetryRow {
                session_id,
                timestamp: None,  // Use database default (NOW())
                engine_rpm: Some(*state.rpm.lock().unwrap()),
                vehicle_speed: Some(*state.speed.lock().unwrap()),
                coolant_temp: Some(*state.coolant_temp.lock().unwrap()),
                throttle_position: Some(*state.throttle_position.lock().unwrap()),
                engine_load: Some(*state.engine_load.lock().unwrap()),
                fuel_level: Some(*state.fuel_level.lock().unwrap()),
                intake_air_temp: Some(*state.intake_air_temp.lock().unwrap()),
                timing_advance: Some(*state.timing_advance.lock().unwrap()),
                battery_voltage: Some(*state.battery_voltage.lock().unwrap()),
                maf_flow_rate: None,
                o2_sensor_voltage: None,
                fuel_pressure: None,
            };

            telemetry_buffer.push(telemetry_row);

            // Batch insert every 10 points (1 second at 10Hz)
            if telemetry_buffer.len() >= 10 {
                let batch = telemetry_buffer.clone();
                telemetry_buffer.clear();

                // Spawn async task to avoid blocking
                let client_clone = client.clone();
                tokio::spawn(async move {
                    if let Err(e) = client_clone.insert_telemetry_batch(&batch).await {
                        eprintln!("⚠️  Failed to insert telemetry batch: {}", e);
                    }
                });
            }
        }

        // Emit event to frontend
        if let Err(e) = app.emit("telemetry-update", &points) {
            eprintln!("Failed to emit telemetry: {}", e);
        }

        // 100ms update rate (10Hz) - adjust for playback speed
        let speed_multiplier = if mode == SimulationMode::Playback {
            *state.playback_speed.lock().unwrap()
        } else {
            1.0
        };

        let delay_ms = (100.0 / speed_multiplier).max(10.0) as u64;
        sleep(Duration::from_millis(delay_ms)).await;
    }
}

/// Random walk simulation (legacy mode)
fn simulate_random_walk(state: &SimulationState) -> Vec<TelemetryPoint> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64;

    {
        let mut rpm = state.rpm.lock().unwrap();
        let mut speed = state.speed.lock().unwrap();
        let mut temp = state.coolant_temp.lock().unwrap();

        // RPM logic: Idle at 800, rev up randomly, drop if too high
        let change = (rand::random::<f64>() - 0.4) * 100.0;
        *rpm = (*rpm + change).clamp(800.0, 7000.0);

        // Speed logic: Follows RPM roughly (gear ratio simulation simplified)
        *speed = (*rpm / 60.0).clamp(0.0, 240.0);

        // Temp logic: Slowly warms up to 90C
        if *temp < 90.0 {
            *temp += 0.05;
        }
    }

    let rpm = *state.rpm.lock().unwrap();
    let speed = *state.speed.lock().unwrap();
    let temp = *state.coolant_temp.lock().unwrap();

    vec![
        TelemetryPoint {
            timestamp,
            pid: ObdPid::EngineRPM,
            value: rpm,
            unit: "rpm".to_string(),
        },
        TelemetryPoint {
            timestamp,
            pid: ObdPid::VehicleSpeed,
            value: speed,
            unit: "km/h".to_string(),
        },
        TelemetryPoint {
            timestamp,
            pid: ObdPid::CoolantTemp,
            value: temp,
            unit: "°C".to_string(),
        },
    ]
}
