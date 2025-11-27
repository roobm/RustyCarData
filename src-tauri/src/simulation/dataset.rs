use crate::models::{ObdPid, TelemetryPoint};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Dataset metadata from catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub id: String,
    pub name: String,
    pub file_path: String,
    pub vehicle: VehicleInfo,
    pub session: SessionInfo,
    pub quality: QualityInfo,
    pub description: String,
    pub source: String,
    pub license: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleInfo {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub engine: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub date: String,
    pub location: String,
    pub driving_style: String,
    pub conditions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityInfo {
    pub pids_available: u16,
    pub sample_count: u32,
    pub sample_rate_hz: u8,
    pub duration_minutes: u16,
}

/// Dataset catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetCatalog {
    pub datasets: Vec<DatasetInfo>,
    pub column_mappings: HashMap<String, String>,
    pub notes: Vec<String>,
}

impl DatasetCatalog {
    /// Load catalog from JSON file
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let catalog: DatasetCatalog = serde_json::from_reader(file)?;
        Ok(catalog)
    }

    /// Get dataset by ID
    pub fn get_dataset(&self, id: &str) -> Option<&DatasetInfo> {
        self.datasets.iter().find(|d| d.id == id)
    }

    /// List all dataset IDs and names
    pub fn list_datasets(&self) -> Vec<(String, String)> {
        self.datasets
            .iter()
            .map(|d| (d.id.clone(), d.name.clone()))
            .collect()
    }
}

/// Dataset loader
pub struct DatasetLoader {
    catalog: DatasetCatalog,
    data_dir: String,
}

impl DatasetLoader {
    /// Create new dataset loader
    pub fn new(catalog_path: &Path, data_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let catalog = DatasetCatalog::load(catalog_path)?;
        Ok(Self {
            catalog,
            data_dir: data_dir.to_string(),
        })
    }

    /// Load dataset by ID
    pub fn load_dataset(&self, id: &str) -> Result<Vec<TelemetryPoint>, Box<dyn std::error::Error>> {
        let dataset_info = self
            .catalog
            .get_dataset(id)
            .ok_or_else(|| format!("Dataset not found: {}", id))?;

        let full_path = format!("{}/{}", self.data_dir, dataset_info.file_path);
        self.load_csv_file(&full_path)
    }

    /// Load CSV file and convert to telemetry points
    fn load_csv_file(&self, path: &str) -> Result<Vec<TelemetryPoint>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut reader = ReaderBuilder::new()
            .flexible(true) // Allow variable number of fields per record
            .from_reader(file);

        let headers = reader.headers()?.clone();
        let column_names: Vec<String> = headers
            .iter()
            .map(|h| h.trim().to_string())
            .collect();

        let mut telemetry_points: Vec<TelemetryPoint> = Vec::new();
        let mut timestamp_microseconds: u64 = 0;

        for result in reader.records() {
            let record = result?;

            // Process each column in the row
            for (idx, value_str) in record.iter().enumerate() {
                if let Some(column_name) = column_names.get(idx) {
                    // Map column name to PID
                    if let Some(pid) = self.map_column_to_pid(column_name) {
                        // Parse value
                        if let Ok(value) = value_str.trim().parse::<f64>() {
                            // Determine unit from column name
                            let unit = self.extract_unit(column_name);

                            telemetry_points.push(TelemetryPoint {
                                timestamp: timestamp_microseconds,
                                pid,
                                value,
                                unit,
                            });
                        }
                    }
                }
            }

            // Increment timestamp by 1 second (assuming 1Hz sample rate)
            timestamp_microseconds += 1_000_000;
        }

        Ok(telemetry_points)
    }

    /// Map CSV column name to ObdPid
    fn map_column_to_pid(&self, column_name: &str) -> Option<ObdPid> {
        // Remove unit suffix if present (e.g., "ENGINE_RPM ()" -> "ENGINE_RPM")
        let clean_name = column_name.trim().replace(" ()", "");

        match clean_name.as_str() {
            "ENGINE_RPM" => Some(ObdPid::EngineRPM),
            "VEHICLE_SPEED" => Some(ObdPid::VehicleSpeed),
            "COOLANT_TEMPERATURE" => Some(ObdPid::CoolantTemp),
            "THROTTLE" => Some(ObdPid::ThrottlePosition),
            "ENGINE_LOAD" => Some(ObdPid::EngineLoad),
            "FUEL_TANK" => Some(ObdPid::FuelLevel),
            "INTAKE_AIR_TEMP" => Some(ObdPid::IntakeAirTemp),
            "TIMING_ADVANCE" => Some(ObdPid::TimingAdvance),
            "CONTROL_MODULE_VOLTAGE" => Some(ObdPid::BatteryVoltage),
            _ => None, // Ignore unmapped columns
        }
    }

    /// Extract unit from column name
    fn extract_unit(&self, column_name: &str) -> String {
        // For now, return generic units based on PID type
        let clean_name = column_name.trim().replace(" ()", "");

        match clean_name.as_str() {
            "ENGINE_RPM" => "rpm",
            "VEHICLE_SPEED" => "km/h",
            "COOLANT_TEMPERATURE" => "°C",
            "THROTTLE" => "%",
            "ENGINE_LOAD" => "%",
            "FUEL_TANK" => "%",
            "INTAKE_AIR_TEMP" => "°C",
            "TIMING_ADVANCE" => "°",
            "CONTROL_MODULE_VOLTAGE" => "V",
            _ => "",
        }
        .to_string()
    }

    /// Get dataset info
    pub fn get_dataset_info(&self, id: &str) -> Option<&DatasetInfo> {
        self.catalog.get_dataset(id)
    }

    /// List all datasets
    pub fn list_datasets(&self) -> Vec<(String, String)> {
        self.catalog.list_datasets()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_mapping() {
        let catalog_json = r#"{
            "datasets": [],
            "column_mappings": {},
            "notes": []
        }"#;

        let catalog: DatasetCatalog = serde_json::from_str(catalog_json).unwrap();
        assert!(catalog.datasets.is_empty());
    }

    #[test]
    fn test_pid_mapping() {
        let catalog = DatasetCatalog {
            datasets: vec![],
            column_mappings: HashMap::new(),
            notes: vec![],
        };

        let loader = DatasetLoader {
            catalog,
            data_dir: "test".to_string(),
        };

        assert_eq!(
            loader.map_column_to_pid("ENGINE_RPM ()"),
            Some(ObdPid::EngineRPM)
        );
        assert_eq!(
            loader.map_column_to_pid("VEHICLE_SPEED ()"),
            Some(ObdPid::VehicleSpeed)
        );
    }

    #[test]
    fn test_all_datasets_load() {
        // Test that all 3 bundled datasets can be loaded successfully
        let base_dir = "data/datasets";

        // Check if running from src-tauri or project root
        let (catalog_path, data_dir) = if Path::new(base_dir).exists() {
            (Path::new("data/datasets/catalog.json"), base_dir)
        } else {
            (Path::new("src-tauri/data/datasets/catalog.json"), "src-tauri/data/datasets")
        };

        let loader = match DatasetLoader::new(catalog_path, data_dir) {
            Ok(loader) => loader,
            Err(e) => {
                eprintln!("Failed to create dataset loader: {}", e);
                eprintln!("Tried catalog: {:?}", catalog_path);
                eprintln!("Tried data dir: {}", data_dir);
                eprintln!("Current dir: {:?}", std::env::current_dir());
                panic!("Dataset loader creation failed");
            }
        };

        // Expected telemetry points (not CSV rows)
        // Each row has 27 columns but we only map ~9 PIDs
        // So: 3853 rows × 9 PIDs = ~34,677 points
        let datasets = vec![
            ("city_commute_1", 30000, 40000, "City Commute"),
            ("highway_drive_1", 20000, 30000, "Highway Driving"),
            ("mixed_long_1", 25000, 35000, "Extended Mixed"),
        ];

        for (id, min_points, max_points, name) in datasets {
            println!("\n=== Testing dataset: {} ===", name);

            // Load dataset
            let result = loader.load_dataset(id);
            assert!(
                result.is_ok(),
                "Failed to load dataset {}: {:?}",
                id,
                result.err()
            );

            let data = result.unwrap();

            // Verify telemetry point count (individual PID readings, not CSV rows)
            println!("  Loaded {} telemetry points", data.len());
            assert!(
                data.len() >= min_points && data.len() <= max_points,
                "Telemetry point count out of range for {}: got {}, expected {}-{}",
                id,
                data.len(),
                min_points,
                max_points
            );

            // Verify we have telemetry points with valid PIDs
            assert!(!data.is_empty(), "Dataset {} is empty", id);

            // Check first point has expected PIDs
            let first_point = &data[0];
            println!("  First point: {:?} = {}", first_point.pid, first_point.value);

            // Verify we have some common PIDs in the dataset
            let has_rpm = data.iter().any(|p| p.pid == ObdPid::EngineRPM);
            let has_speed = data.iter().any(|p| p.pid == ObdPid::VehicleSpeed);

            assert!(has_rpm, "Dataset {} missing EngineRPM data", id);
            assert!(has_speed, "Dataset {} missing VehicleSpeed data", id);

            println!("  ✅ Dataset {} passed validation", name);
        }

        println!("\n✅ All 3 datasets loaded successfully!");
    }
}
