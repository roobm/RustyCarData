use std::collections::HashMap;
use std::path::Path;
use chrono::Utc;
use uuid::Uuid;

// Import from the main library
use tauri_app_lib::storage::supabase::{SupabaseClient, SessionRow, TelemetryRow};

#[derive(Debug)]
struct CsvRow {
    timestamp: u64,
    pid: String,
    value: f64,
    _unit: String, // Keep for CSV parsing, not used in processing
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("🚀 Dataset to Supabase Import Tool");
    println!("===================================\n");

    // Initialize Supabase client
    let client = SupabaseClient::new()
        .expect("Failed to connect to Supabase. Check your .env file!");

    println!("✅ Connected to Supabase\n");

    // Define dataset files to import
    let dataset_files = vec![
        ("data/datasets/city_commute/live1.csv", "City Commute - Live Recording 1"),
        ("data/datasets/city_commute/obd_data.csv", "City Commute - OBD Data"),
        ("data/datasets/highway/drive1.csv", "Highway - Drive 1"),
    ];

    println!("📁 Found {} dataset files to import\n", dataset_files.len());

    for (index, (path, name)) in dataset_files.iter().enumerate() {
        println!("📄 [{}/{}] Processing: {}", index + 1, dataset_files.len(), name);

        match import_dataset(&client, Path::new(path), name).await {
            Ok((session_id, point_count)) => {
                println!("   ✅ Imported as session: {} ({} points)\n", session_id, point_count);
            }
            Err(e) => {
                eprintln!("   ⚠️  Failed: {}\n", e);
            }
        }
    }

    println!("🎉 Import complete!");
    println!("\n📊 View your data at:");
    println!("   https://supabase.com/dashboard/project/wbzwkpzimlidyvghivbo/editor");

    Ok(())
}

async fn import_dataset(
    client: &SupabaseClient,
    csv_path: &Path,
    dataset_name: &str,
) -> Result<(Uuid, usize), Box<dyn std::error::Error>> {
    // Read CSV file
    let mut rdr = csv::Reader::from_path(csv_path)?;
    let mut rows: Vec<CsvRow> = Vec::new();

    println!("   📖 Reading CSV file...");

    for result in rdr.records() {
        let record = result?;
        if record.len() < 4 {
            continue;
        }

        let timestamp: u64 = record[0].parse().unwrap_or(0);
        let pid = record[1].to_string();
        let value: f64 = record[2].parse().unwrap_or(0.0);
        let unit = record[3].to_string();

        rows.push(CsvRow {
            timestamp,
            pid,
            value,
            _unit: unit,
        });
    }

    if rows.is_empty() {
        return Err("No data in CSV file".into());
    }

    println!("   📊 Read {} raw data points", rows.len());

    // Group by timestamp to create telemetry snapshots
    let mut timestamps: HashMap<u64, HashMap<String, f64>> = HashMap::new();
    for row in &rows {
        timestamps.entry(row.timestamp)
            .or_insert_with(HashMap::new)
            .insert(row.pid.clone(), row.value);
    }

    let mut sorted_timestamps: Vec<_> = timestamps.keys().collect();
    sorted_timestamps.sort();

    let snapshot_count = sorted_timestamps.len();
    println!("   📸 Grouped into {} telemetry snapshots", snapshot_count);

    // Calculate duration (last timestamp in microseconds)
    let first_timestamp = **sorted_timestamps.first().unwrap();
    let last_timestamp = **sorted_timestamps.last().unwrap();
    let duration_seconds = ((last_timestamp - first_timestamp) / 1_000_000) as i32;

    // Use a fixed reference date for dataset imports (Jan 1, 2024)
    let start_time = Utc::now(); // Use current time for simplicity

    // Create session
    let session = SessionRow {
        id: None,
        created_at: None,
        started_at: start_time.to_rfc3339(),
        ended_at: Some((start_time + chrono::Duration::seconds(duration_seconds as i64)).to_rfc3339()),
        session_name: Some(dataset_name.to_string()),
        dataset_source: Some("reference_dataset".to_string()),
        vehicle_make: Some("Toyota".to_string()),
        vehicle_model: Some("Etios".to_string()),
        vehicle_year: Some(2014),
        duration_seconds: Some(duration_seconds),
        distance_km: None,
    };

    let session_id = client.create_session(&session).await?;
    println!("   📝 Created session: {}", session_id);
    println!("   ⏱️  Duration: {} seconds ({} minutes)", duration_seconds, duration_seconds / 60);

    // Convert telemetry data
    let mut telemetry_points: Vec<TelemetryRow> = Vec::new();
    let mut total_uploaded = 0;

    for ts in sorted_timestamps {
        let snapshot = &timestamps[ts];

        let telemetry = TelemetryRow {
            session_id,
            timestamp: None,  // Use server timestamp
            engine_rpm: snapshot.get("EngineRPM").copied(),
            vehicle_speed: snapshot.get("VehicleSpeed").copied(),
            coolant_temp: snapshot.get("CoolantTemp").copied(),
            throttle_position: snapshot.get("ThrottlePosition").copied(),
            engine_load: snapshot.get("EngineLoad").copied(),
            fuel_level: snapshot.get("FuelLevel").copied(),
            intake_air_temp: snapshot.get("IntakeAirTemp").copied(),
            timing_advance: snapshot.get("TimingAdvance").copied(),
            battery_voltage: snapshot.get("BatteryVoltage").copied(),
            maf_flow_rate: snapshot.get("MAFFlowRate").copied(),
            o2_sensor_voltage: snapshot.get("O2SensorVoltage").copied(),
            fuel_pressure: snapshot.get("FuelPressure").copied(),
        };

        telemetry_points.push(telemetry);

        // Batch insert every 500 points (larger batches for efficiency)
        if telemetry_points.len() >= 500 {
            client.insert_telemetry_batch(&telemetry_points).await?;
            total_uploaded += telemetry_points.len();
            print!("\r   📊 Uploaded {}/{} points ({:.1}%)...",
                total_uploaded,
                snapshot_count,
                (total_uploaded as f64 / snapshot_count as f64) * 100.0
            );
            std::io::Write::flush(&mut std::io::stdout())?;
            telemetry_points.clear();
        }
    }

    // Insert remaining points
    if !telemetry_points.is_empty() {
        client.insert_telemetry_batch(&telemetry_points).await?;
        total_uploaded += telemetry_points.len();
    }

    println!("\r   📊 Uploaded {}/{} points (100.0%) ✓              ", total_uploaded, snapshot_count);

    Ok((session_id, total_uploaded))
}
