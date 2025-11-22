use std::path::PathBuf;
use chrono::{DateTime, Local};
use std::fs;
use crate::models::{TelemetryPoint, ObdPid};
use crate::storage::writer::ParquetWriter;

pub struct SessionManager {
    pub data_dir: PathBuf,
}

impl SessionManager {
    pub fn new(_app_handle: &tauri::AppHandle) -> Self {
        // Use current working directory / project folder instead of app_data_dir
        let data_dir = std::env::current_dir()
            .expect("failed to get current directory")
            .join("data")
            .join("sessions");

        if !data_dir.exists() {
            fs::create_dir_all(&data_dir).expect("failed to create session dir");
        }

        Self { data_dir }
    }

    pub fn create_new_session_path(&self) -> PathBuf {
        let now: DateTime<Local> = Local::now();
        let filename = format!("session_{}.csv", now.format("%Y%m%d_%H%M%S"));
        self.data_dir.join(filename)
    }

    pub fn list_sessions(&self) -> Vec<PathBuf> {
        let mut sessions = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.data_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                    sessions.push(path);
                }
            }
        }
        // Sort by name (timestamp) descending
        sessions.sort_by(|a, b| b.cmp(a));
        sessions
    }

    pub fn import_csv(&self, csv_path: PathBuf) -> Result<PathBuf, String> {
        let mut rdr = csv::Reader::from_path(&csv_path).map_err(|e| e.to_string())?;
        
        // Create a new session file for the import
        let now: DateTime<Local> = Local::now();
        let filename = format!("imported_{}.csv", now.format("%Y%m%d_%H%M%S"));
        let csv_dest_path = self.data_dir.join(filename);

        let mut writer = ParquetWriter::new(csv_dest_path.clone());
        writer.init()?;

        // Assume CSV format: timestamp,pid,value,unit
        // We need to be flexible or strict. Let's be strict for MVP.
        // Headers: timestamp, pid, value, unit
        
        for result in rdr.records() {
            let record = result.map_err(|e| e.to_string())?;
            if record.len() < 4 { continue; }

            let timestamp: u64 = record[0].parse().unwrap_or(0);
            let pid_str = &record[1];
            let value: f64 = record[2].parse().unwrap_or(0.0);
            let unit = record[3].to_string();

            // Map string PID to Enum if possible, or just use generic
            let pid = match pid_str {
                "EngineRPM" => ObdPid::EngineRPM,
                "VehicleSpeed" => ObdPid::VehicleSpeed,
                "CoolantTemp" => ObdPid::CoolantTemp,
                _ => ObdPid::EngineRPM, // Fallback or handle unknown
            };

            let point = TelemetryPoint {
                timestamp,
                pid,
                value,
                unit,
            };

            writer.write_point(&point)?;
        }

        writer.close()?;
        Ok(csv_dest_path)
    }
}
