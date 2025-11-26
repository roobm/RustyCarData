use tauri_app_lib::agents::*;
use tauri_app_lib::agents::implementations::FuelCoachAgent;
use chrono::Utc;

fn create_test_request(
    avg_mpg: f32,
    avg_rpm: f32,
    rapid_accel_count: usize,
    avg_speed: f32,
) -> AnalysisRequest {
    AnalysisRequest {
        request_id: "test-001".to_string(),
        session_data: SessionSummary {
            session_id: "session-test-001".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now(),
            duration_seconds: 1800,
            distance_miles: 25.0,
            fuel_consumed_gallons: 1.0,
            avg_fuel_efficiency_mpg: avg_mpg,
            avg_speed_mph: avg_speed,
            max_speed_mph: 75.0,
            speed_violations_count: 0,
            avg_rpm: avg_rpm,
            max_rpm: 5000.0,
            high_rpm_duration_seconds: 0,
            avg_coolant_temp_f: 195.0,
            max_coolant_temp_f: 210.0,
            avg_oil_temp_f: 220.0,
            hard_braking_events: vec![],
            rapid_acceleration_events: (0..rapid_accel_count)
                .map(|i| Event {
                    timestamp: Utc::now(),
                    event_type: EventType::RapidAcceleration,
                    severity: 0.8,
                    location: None,
                    metadata: Default::default(),
                })
                .collect(),
            error_codes: vec![],
            sensor_readings: Default::default(),
        },
        vehicle_info: VehicleInfo {
            make: "Honda".to_string(),
            model: "Civic".to_string(),
            year: 2018,
            engine_type: EngineType::GasolineNaturallyAspirated,
            transmission: TransmissionType::Manual,
            fuel_type: FuelType::Regular87,
            vin: None,
        },
        user_preferences: None,
        historical_context: None,
        force_local: false,
        timeout_seconds: 10,
    }
}

#[tokio::test]
async fn test_fuel_coach_good_efficiency() {
    let agent = FuelCoachAgent::new();
    let request = create_test_request(30.0, 2000.0, 0, 55.0);

    let result = agent.analyze(&request).await.unwrap();

    assert_eq!(result.agent_id, "fuel-coach");
    assert!(result.score.unwrap() > 80.0, "Good MPG should have high score");
    assert!(!result.insights.is_empty(), "Should have insights");
    assert!(!result.recommendations.is_empty(), "Should have recommendations");
}

#[tokio::test]
async fn test_fuel_coach_poor_efficiency() {
    let agent = FuelCoachAgent::new();
    let request = create_test_request(15.0, 3500.0, 8, 70.0);

    let result = agent.analyze(&request).await.unwrap();

    assert_eq!(result.agent_id, "fuel-coach");
    assert!(result.score.unwrap() < 60.0, "Poor MPG should have low score");

    // Should have multiple insights for poor driving
    let warning_insights: Vec<_> = result.insights.iter()
        .filter(|i| matches!(i.category, InsightCategory::Warning | InsightCategory::Critical))
        .collect();
    assert!(!warning_insights.is_empty(), "Should have warning insights");
}

#[tokio::test]
async fn test_fuel_coach_high_rpm_penalty() {
    let agent = FuelCoachAgent::new();
    let request = create_test_request(25.0, 4000.0, 0, 60.0);

    let result = agent.analyze(&request).await.unwrap();

    // High RPM should reduce score
    let has_rpm_insight = result.insights.iter()
        .any(|i| i.title.contains("RPM"));
    assert!(has_rpm_insight, "Should have RPM-related insight");

    let has_rpm_recommendation = result.recommendations.iter()
        .any(|r| r.action.contains("Shift") || r.action.contains("RPM"));
    assert!(has_rpm_recommendation, "Should have RPM-related recommendation");
}

#[tokio::test]
async fn test_fuel_coach_rapid_acceleration() {
    let agent = FuelCoachAgent::new();
    let request = create_test_request(22.0, 2500.0, 10, 60.0);

    let result = agent.analyze(&request).await.unwrap();

    let has_accel_insight = result.insights.iter()
        .any(|i| i.title.contains("Acceleration"));
    assert!(has_accel_insight, "Should detect rapid acceleration");

    let has_accel_recommendation = result.recommendations.iter()
        .any(|r| r.action.contains("Accelerate gradually"));
    assert!(has_accel_recommendation, "Should recommend gradual acceleration");
}

#[tokio::test]
async fn test_fuel_coach_metadata() {
    let agent = FuelCoachAgent::new();
    let metadata = agent.metadata();

    assert_eq!(metadata.id, "fuel-coach");
    assert_eq!(metadata.name, "Fuel Efficiency Coach");
    assert_eq!(metadata.domain, AgentDomain::FuelEfficiency);
    assert_eq!(metadata.icon, "⛽");
    assert_eq!(metadata.privacy_level, PrivacyLevel::CloudPreferred);
}

#[tokio::test]
async fn test_fuel_coach_can_handle() {
    let agent = FuelCoachAgent::new();

    // Should handle sessions with fuel data
    let mut request = create_test_request(25.0, 2500.0, 0, 60.0);
    assert!(agent.can_handle(&request), "Should handle normal fuel data");

    // Should not handle sessions without fuel consumption
    request.session_data.fuel_consumed_gallons = 0.0;
    assert!(!agent.can_handle(&request), "Should not handle zero fuel consumption");
}

#[tokio::test]
async fn test_fuel_coach_execution_time() {
    let agent = FuelCoachAgent::new();
    let request = create_test_request(25.0, 2500.0, 5, 60.0);

    let result = agent.analyze(&request).await.unwrap();

    assert!(result.execution_time_ms < 1000, "Should execute quickly");
    assert_eq!(result.metadata.provider, "local-rules");
    assert_eq!(result.metadata.execution_location, ExecutionLocation::Local);
}
