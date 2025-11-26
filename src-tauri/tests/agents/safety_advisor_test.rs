use tauri_app_lib::agents::*;
use tauri_app_lib::agents::implementations::SafetyAdvisorAgent;
use chrono::Utc;

fn create_test_request(
    speed_violations: u32,
    hard_braking_count: usize,
    rapid_accel_count: usize,
    max_speed: f32,
) -> AnalysisRequest {
    AnalysisRequest {
        request_id: "test-safety-001".to_string(),
        session_data: SessionSummary {
            session_id: "session-safety-001".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now(),
            duration_seconds: 1800,
            distance_miles: 25.0,
            fuel_consumed_gallons: 1.0,
            avg_fuel_efficiency_mpg: 25.0,
            avg_speed_mph: 55.0,
            max_speed_mph: max_speed,
            speed_violations_count: speed_violations,
            avg_rpm: 2500.0,
            max_rpm: 5000.0,
            high_rpm_duration_seconds: 0,
            avg_coolant_temp_f: 195.0,
            max_coolant_temp_f: 210.0,
            avg_oil_temp_f: 220.0,
            hard_braking_events: (0..hard_braking_count)
                .map(|_| Event {
                    timestamp: Utc::now(),
                    event_type: EventType::HardBraking,
                    severity: 0.8,
                    location: None,
                    metadata: Default::default(),
                })
                .collect(),
            rapid_acceleration_events: (0..rapid_accel_count)
                .map(|_| Event {
                    timestamp: Utc::now(),
                    event_type: EventType::RapidAcceleration,
                    severity: 0.7,
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
async fn test_safety_safe_driving() {
    let agent = SafetyAdvisorAgent::new();
    let request = create_test_request(0, 0, 0, 65.0);

    let result = agent.analyze(&request).await.unwrap();

    assert_eq!(result.agent_id, "safety-advisor");
    assert!(result.score.unwrap() >= 80.0, "Safe driving should have high score");

    let positive_insights: Vec<_> = result.insights.iter()
        .filter(|i| matches!(i.category, InsightCategory::Positive))
        .collect();
    assert!(!positive_insights.is_empty(), "Should have positive safety insight");
}

#[tokio::test]
async fn test_safety_speeding_violations() {
    let agent = SafetyAdvisorAgent::new();
    let request = create_test_request(5, 0, 0, 85.0);

    let result = agent.analyze(&request).await.unwrap();

    assert!(result.score.unwrap() < 80.0, "Speeding should reduce score");

    let has_speed_insight = result.insights.iter()
        .any(|i| i.title.contains("Speed"));
    assert!(has_speed_insight, "Should have speeding insight");

    let has_speed_recommendation = result.recommendations.iter()
        .any(|r| r.action.contains("speed limit"));
    assert!(has_speed_recommendation, "Should recommend observing speed limits");
}

#[tokio::test]
async fn test_safety_hard_braking() {
    let agent = SafetyAdvisorAgent::new();
    let request = create_test_request(0, 8, 0, 65.0);

    let result = agent.analyze(&request).await.unwrap();

    let has_braking_insight = result.insights.iter()
        .any(|i| i.title.contains("Braking"));
    assert!(has_braking_insight, "Should detect hard braking");

    let has_following_distance_rec = result.recommendations.iter()
        .any(|r| r.action.contains("following distance"));
    assert!(has_following_distance_rec, "Should recommend maintaining following distance");
}

#[tokio::test]
async fn test_safety_excessive_speed() {
    let agent = SafetyAdvisorAgent::new();
    let request = create_test_request(2, 0, 0, 95.0);

    let result = agent.analyze(&request).await.unwrap();

    let has_excessive_speed_rec = result.recommendations.iter()
        .any(|r| r.priority == Priority::High && r.action.contains("80 mph"));
    assert!(has_excessive_speed_rec, "Should have high priority recommendation for excessive speed");
}

#[tokio::test]
async fn test_safety_multiple_risks() {
    let agent = SafetyAdvisorAgent::new();
    let request = create_test_request(10, 15, 8, 90.0);

    let result = agent.analyze(&request).await.unwrap();

    assert!(result.score.unwrap() < 50.0, "Multiple risks should result in poor score");

    let critical_insights: Vec<_> = result.insights.iter()
        .filter(|i| matches!(i.category, InsightCategory::Critical))
        .collect();
    assert!(!critical_insights.is_empty(), "Should have critical safety insights");

    // Should have multiple high priority recommendations
    let high_priority_recs: Vec<_> = result.recommendations.iter()
        .filter(|r| r.priority == Priority::High)
        .collect();
    assert!(high_priority_recs.len() >= 2, "Should have multiple high priority recommendations");
}

#[tokio::test]
async fn test_safety_metadata() {
    let agent = SafetyAdvisorAgent::new();
    let metadata = agent.metadata();

    assert_eq!(metadata.id, "safety-advisor");
    assert_eq!(metadata.name, "Safety Advisor");
    assert_eq!(metadata.domain, AgentDomain::Safety);
    assert_eq!(metadata.icon, "🛡️");
    assert!(metadata.description.contains("NHTSA"));
}

#[tokio::test]
async fn test_safety_can_handle_any_session() {
    let agent = SafetyAdvisorAgent::new();
    let request = create_test_request(0, 0, 0, 60.0);

    // Safety advisor can handle any driving session
    assert!(agent.can_handle(&request));
}

#[tokio::test]
async fn test_safety_confidence() {
    let agent = SafetyAdvisorAgent::new();
    let request = create_test_request(3, 5, 2, 75.0);

    let result = agent.analyze(&request).await.unwrap();

    assert!(result.confidence >= 0.8, "Rule-based safety analysis should have high confidence");
}
