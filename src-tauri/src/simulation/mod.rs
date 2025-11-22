use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::sleep;
use crate::models::{ObdPid, TelemetryPoint};
use self::state::SimulationState;

pub mod state;

pub async fn run_simulation_loop(app: AppHandle, state: Arc<SimulationState>) {
    loop {
        // Check if simulation should run
        let is_running = *state.is_running.lock().unwrap();
        if !is_running {
            sleep(Duration::from_millis(100)).await;
            continue;
        }

        // Simulate physics (simple random walk for now)
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

        // Create telemetry points
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        let points = {
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
        };

        // Emit event to frontend
        if let Err(e) = app.emit("telemetry-update", &points) {
            eprintln!("Failed to emit telemetry: {}", e);
        }

        // 100ms update rate (10Hz)
        sleep(Duration::from_millis(100)).await;
    }
}
