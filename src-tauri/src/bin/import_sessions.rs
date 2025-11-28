use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc, NaiveDateTime};
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

    println!("🚀 CSV to Supabase Import Tool");
    println!("================================\n");

    // Initialize Supabase client
    let client = SupabaseClient::new()
        .expect("Failed to connect to Supabase. Check your .env file!");

    println!("✅ Connected to Supabase\n");

    // Find all session CSV files
    let sessions_dir = Path::new("data/sessions");
    if !sessions_dir.exists() {
        eprintln!("❌ Sessions directory not found: {:?}", sessions_dir);
        return Ok(());
    }

    let mut csv_files: Vec<_> = fs::read_dir(sessions_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "csv")
                .unwrap_or(false)
        })
        .collect();

    csv_files.sort_by_key(|entry| entry.path());

    println!("📁 Found {} CSV session files\n", csv_files.len());

    for (index, entry) in csv_files.iter().enumerate() {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();

        println!("📄 [{}/{}] Processing: {}", index + 1, csv_files.len(), filename);

        match import_session(&client, &path).await {
            Ok(session_id) => {
                println!("   ✅ Imported as session: {}\n", session_id);
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

async fn import_session(
    client: &SupabaseClient,
    csv_path: &Path,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    // Parse filename to get timestamp
    let filename = csv_path.file_stem().unwrap().to_str().unwrap();
    let timestamp_str = filename.replace("session_", "");

    // Parse timestamp from filename (format: YYYYMMDD_HHMMSS)
    let start_time = parse_timestamp_from_filename(&timestamp_str)?;

    // Read CSV file
    let mut rdr = csv::Reader::from_path(csv_path)?;
    let mut rows: Vec<CsvRow> = Vec::new();

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

    // Group by timestamp to create telemetry snapshots
    let mut timestamps: HashMap<u64, HashMap<String, f64>> = HashMap::new();
    for row in &rows {
        timestamps.entry(row.timestamp)
            .or_insert_with(HashMap::new)
            .insert(row.pid.clone(), row.value);
    }

    let mut sorted_timestamps: Vec<_> = timestamps.keys().collect();
    sorted_timestamps.sort();

    // Calculate duration (last timestamp in microseconds)
    let last_timestamp = **sorted_timestamps.last().unwrap();
    let duration_seconds = (last_timestamp / 1_000_000) as i32;

    // Create session
    let session = SessionRow {
        id: None,
        created_at: None,
        started_at: start_time.to_rfc3339(),
        ended_at: Some((start_time + chrono::Duration::seconds(duration_seconds as i64)).to_rfc3339()),
        session_name: Some(format!("Imported: {}", filename)),
        dataset_source: Some("imported".to_string()),
        vehicle_make: Some("Toyota".to_string()),
        vehicle_model: Some("Etios".to_string()),
        vehicle_year: Some(2014),
        duration_seconds: Some(duration_seconds),
        distance_km: None,
    };

    let session_id = client.create_session(&session).await?;
    println!("   📝 Created session: {}", session_id);

    // Convert telemetry data
    let mut telemetry_points: Vec<TelemetryRow> = Vec::new();

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

        // Batch insert every 100 points
        if telemetry_points.len() >= 100 {
            client.insert_telemetry_batch(&telemetry_points).await?;
            print!("   📊 Uploaded {} points... ", telemetry_points.len());
            std::io::Write::flush(&mut std::io::stdout())?;
            telemetry_points.clear();
        }
    }

    // Insert remaining points
    if !telemetry_points.is_empty() {
        client.insert_telemetry_batch(&telemetry_points).await?;
        println!("   📊 Uploaded {} points (final batch)", telemetry_points.len());
    }

    Ok(session_id)
}

fn parse_timestamp_from_filename(timestamp_str: &str) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
    // Format: YYYYMMDD_HHMMSS
    if timestamp_str.len() != 15 {
        return Err(format!("Invalid timestamp format: {}", timestamp_str).into());
    }

    let year: i32 = timestamp_str[0..4].parse()?;
    let month: u32 = timestamp_str[4..6].parse()?;
    let day: u32 = timestamp_str[6..8].parse()?;
    let hour: u32 = timestamp_str[9..11].parse()?;
    let minute: u32 = timestamp_str[11..13].parse()?;
    let second: u32 = timestamp_str[13..15].parse()?;

    let naive = NaiveDateTime::parse_from_str(
        &format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hour, minute, second),
        "%Y-%m-%d %H:%M:%S"
    )?;

    Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
}
