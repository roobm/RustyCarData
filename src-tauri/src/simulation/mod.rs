use std::sync::Arc;
use std::time::Duration;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use tokio::time::sleep;
use crate::models::{ObdPid, TelemetryPoint};
use crate::storage::manager::SessionManager;
use crate::storage::writer::ParquetWriter;
use self::state::{SimulationState, SimulationMode};
use self::dataset::DatasetLoader;
use self::playback::DatasetPlayback;

pub mod state;
pub mod dataset;
pub mod playback;

pub async fn run_simulation_loop(app: AppHandle, state: Arc<SimulationState>) {
    let session_manager = SessionManager::new(&app);
    let mut current_writer: Option<ParquetWriter> = None;
    let mut was_running = false;

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
            // Just started
            let path = session_manager.create_new_session_path();
            println!("Starting new session recording: {:?}", path);
            let mut writer = ParquetWriter::new(path);
            if let Err(e) = writer.init() {
                eprintln!("Failed to init parquet writer: {}", e);
            } else {
                current_writer = Some(writer);
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
            // Just stopped
            if let Some(mut writer) = current_writer.take() {
                println!("Stopping session recording");
                if let Err(e) = writer.close() {
                    eprintln!("Failed to close parquet writer: {}", e);
                }
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

        // Write to disk
        if let Some(writer) = &mut current_writer {
            for point in &points {
                if let Err(e) = writer.write_point(point) {
                    eprintln!("Failed to write point: {}", e);
                }
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
