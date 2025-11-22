use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::sleep;
use crate::models::CanFrame;
use crate::simulation::state::SimulationState;

pub async fn run_mock_can_loop(app: AppHandle, state: Arc<SimulationState>) {
    loop {
        // Check if sniffing is active
        let is_sniffing = *state.is_sniffing.lock().unwrap();
        if !is_sniffing {
            sleep(Duration::from_millis(500)).await;
            continue;
        }

        // 1. Generate Random Noise Frame
        if rand::random::<f32>() > 0.5 {
            let frame = CanFrame {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_micros() as u64,
                id: 0x100 + (rand::random::<u32>() % 0x400), // Random ID between 0x100 and 0x500
                data: rand::random::<[u8; 8]>(),
                is_extended: false,
            };
            let _ = app.emit("can-frame", &frame);
        }

        // 2. Generate OBD Response Frame (0x7E8) based on current state
        // Only if engine is running
        let is_running = *state.is_running.lock().unwrap();
        if is_running {
            let rpm = *state.rpm.lock().unwrap();
            let speed = *state.speed.lock().unwrap();

            // Simulate RPM response (PID 0x0C)
            // Formula: (A * 256 + B) / 4
            // Reverse: val * 4 = A * 256 + B
            let rpm_val = (rpm * 4.0) as u32;
            let a = (rpm_val / 256) as u8;
            let b = (rpm_val % 256) as u8;

            let rpm_frame = CanFrame {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_micros() as u64,
                id: 0x7E8,
                data: [0x04, 0x41, 0x0C, a, b, 0x00, 0x00, 0x00], // 0x04 len, 0x41 mode, 0x0C PID
                is_extended: false,
            };
            let _ = app.emit("can-frame", &rpm_frame);

            // Simulate Speed response (PID 0x0D)
            // Formula: A
            let speed_val = speed as u8;
            let speed_frame = CanFrame {
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_micros() as u64,
                id: 0x7E8,
                data: [0x03, 0x41, 0x0D, speed_val, 0x00, 0x00, 0x00, 0x00],
                is_extended: false,
            };
            let _ = app.emit("can-frame", &speed_frame);
        }

        // Sleep for a bit (simulate bus traffic)
        // 50ms = 20Hz roughly
        sleep(Duration::from_millis(50)).await;
    }
}
