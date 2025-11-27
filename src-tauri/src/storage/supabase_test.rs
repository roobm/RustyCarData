use super::supabase::*;
use chrono::Utc;

#[tokio::test]
#[ignore] // Requires .env file with valid Supabase credentials
async fn test_supabase_connection() {
    // Load .env file
    dotenv::dotenv().ok();

    // Create client
    let client = SupabaseClient::new()
        .expect("Failed to create Supabase client. Make sure SUPABASE_URL and SUPABASE_ANON_KEY are set in .env");

    // Create a test session
    let session = SessionRow {
        id: None,
        created_at: None,
        started_at: Utc::now().to_rfc3339(),
        ended_at: None,
        session_name: Some("Test Session".to_string()),
        dataset_source: Some("playback".to_string()),
        vehicle_make: Some("Toyota".to_string()),
        vehicle_model: Some("Etios".to_string()),
        vehicle_year: Some(2014),
        duration_seconds: None,
        distance_km: None,
    };

    // Test creating a session
    let session_id = client
        .create_session(&session)
        .await
        .expect("Failed to create session in Supabase");

    println!("✅ Created session with ID: {}", session_id);

    // Test inserting telemetry
    let telemetry = vec![TelemetryRow {
        session_id,
        timestamp: None,
        engine_rpm: Some(2500.0),
        vehicle_speed: Some(60.0),
        coolant_temp: Some(85.0),
        throttle_position: Some(45.0),
        engine_load: Some(50.0),
        fuel_level: Some(75.0),
        intake_air_temp: Some(25.0),
        timing_advance: Some(15.0),
        battery_voltage: Some(14.2),
        maf_flow_rate: None,
        o2_sensor_voltage: None,
        fuel_pressure: None,
    }];

    client
        .insert_telemetry_batch(&telemetry)
        .await
        .expect("Failed to insert telemetry");

    println!("✅ Inserted {} telemetry points", telemetry.len());

    // Test retrieving the session
    let retrieved = client
        .get_session(session_id)
        .await
        .expect("Failed to get session")
        .expect("Session not found");

    println!("✅ Retrieved session: {:?}", retrieved.session_name);

    // Test listing sessions
    let sessions = client
        .list_sessions(10)
        .await
        .expect("Failed to list sessions");

    println!("✅ Listed {} sessions", sessions.len());

    println!("\n🎉 All Supabase tests passed!");
}
