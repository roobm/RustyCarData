use tauri_app_lib::agents::*;
use tauri_app_lib::agents::implementations::DiagnosticAgent;
use chrono::Utc;

fn create_test_request(
    max_coolant_temp: f32,
    avg_oil_temp: f32,
    max_rpm: f32,
    error_codes: Vec<ErrorCode>,
) -> AnalysisRequest {
    AnalysisRequest {
        request_id: "test-diag-001".to_string(),
        session_data: SessionSummary {
            session_id: "session-diag-001".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now(),
            duration_seconds: 1800,
            distance_miles: 25.0,
            fuel_consumed_gallons: 1.0,
            avg_fuel_efficiency_mpg: 25.0,
            avg_speed_mph: 55.0,
            max_speed_mph: 70.0,
            speed_violations_count: 0,
            avg_rpm: 2500.0,
            max_rpm,
            high_rpm_duration_seconds: 0,
            avg_coolant_temp_f: 195.0,
            max_coolant_temp_f: max_coolant_temp,
            avg_oil_temp_f: avg_oil_temp,
            hard_braking_events: vec![],
            rapid_acceleration_events: vec![],
            error_codes,
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
async fn test_diagnostic_healthy_vehicle() {
    let agent = DiagnosticAgent::new();
    let request = create_test_request(205.0, 220.0, 5500.0, vec![]);

    let result = agent.analyze(&request).await.unwrap();

    assert_eq!(result.agent_id, "diagnostic");
    assert!(result.score.unwrap() >= 85.0, "Healthy vehicle should have high score");

    let positive_insights: Vec<_> = result.insights.iter()
        .filter(|i| matches!(i.category, InsightCategory::Positive))
        .collect();
    assert!(!positive_insights.is_empty(), "Should have positive health insight");
}

#[tokio::test]
async fn test_diagnostic_high_coolant_temp() {
    let agent = DiagnosticAgent::new();
    let request = create_test_request(230.0, 220.0, 5500.0, vec![]);

    let result = agent.analyze(&request).await.unwrap();

    assert!(result.score.unwrap() < 85.0, "High coolant temp should reduce score");

    let has_coolant_insight = result.insights.iter()
        .any(|i| i.title.contains("Coolant") && matches!(i.category, InsightCategory::Critical));
    assert!(has_coolant_insight, "Should have critical coolant temperature insight");

    let has_coolant_rec = result.recommendations.iter()
        .any(|r| r.action.contains("cooling system") && r.priority == Priority::High);
    assert!(has_coolant_rec, "Should have high priority cooling system recommendation");
}

#[tokio::test]
async fn test_diagnostic_elevated_coolant_temp() {
    let agent = DiagnosticAgent::new();
    let request = create_test_request(215.0, 220.0, 5500.0, vec![]);

    let result = agent.analyze(&request).await.unwrap();

    let has_warning = result.insights.iter()
        .any(|i| i.title.contains("Coolant") && matches!(i.category, InsightCategory::Warning));
    assert!(has_warning, "Should have warning for elevated coolant temp");
}

#[tokio::test]
async fn test_diagnostic_high_oil_temp() {
    let agent = DiagnosticAgent::new();
    let request = create_test_request(205.0, 260.0, 5500.0, vec![]);

    let result = agent.analyze(&request).await.unwrap();

    let has_oil_insight = result.insights.iter()
        .any(|i| i.title.contains("Oil"));
    assert!(has_oil_insight, "Should detect high oil temperature");

    let has_oil_rec = result.recommendations.iter()
        .any(|r| r.action.contains("synthetic oil") || r.action.contains("oil level"));
    assert!(has_oil_rec, "Should recommend oil-related action");
}

#[tokio::test]
async fn test_diagnostic_excessive_rpm() {
    let agent = DiagnosticAgent::new();
    let request = create_test_request(205.0, 220.0, 6500.0, vec![]);

    let result = agent.analyze(&request).await.unwrap();

    let has_rpm_insight = result.insights.iter()
        .any(|i| i.title.contains("RPM"));
    assert!(has_rpm_insight, "Should detect excessive RPM");
}

#[tokio::test]
async fn test_diagnostic_error_codes() {
    let agent = DiagnosticAgent::new();
    let error_codes = vec![
        ErrorCode {
            code: "P0420".to_string(),
            description: "Catalyst System Efficiency Below Threshold".to_string(),
            severity: ErrorSeverity::Moderate,
            timestamp: Utc::now(),
        },
        ErrorCode {
            code: "P0171".to_string(),
            description: "System Too Lean (Bank 1)".to_string(),
            severity: ErrorSeverity::Moderate,
            timestamp: Utc::now(),
        },
    ];
    let request = create_test_request(205.0, 220.0, 5500.0, error_codes);

    let result = agent.analyze(&request).await.unwrap();

    assert!(result.score.unwrap() < 70.0, "Error codes should significantly reduce score");

    // Should have insights for each error code
    let error_insights: Vec<_> = result.insights.iter()
        .filter(|i| i.title.contains("Code") && matches!(i.category, InsightCategory::Critical))
        .collect();
    assert_eq!(error_insights.len(), 2, "Should have insight for each error code");

    // Should recommend addressing error codes
    let has_error_rec = result.recommendations.iter()
        .any(|r| r.action.contains("diagnostic trouble code") && r.priority == Priority::High);
    assert!(has_error_rec, "Should have high priority recommendation for error codes");
}

#[tokio::test]
async fn test_diagnostic_multiple_issues() {
    let agent = DiagnosticAgent::new();
    let error_codes = vec![
        ErrorCode {
            code: "P0301".to_string(),
            description: "Cylinder 1 Misfire Detected".to_string(),
            severity: ErrorSeverity::Severe,
            timestamp: Utc::now(),
        },
    ];
    let request = create_test_request(235.0, 265.0, 6800.0, error_codes);

    let result = agent.analyze(&request).await.unwrap();

    assert!(result.score.unwrap() < 50.0, "Multiple issues should result in poor score");

    // Should have multiple critical or warning insights
    let serious_insights: Vec<_> = result.insights.iter()
        .filter(|i| matches!(i.category, InsightCategory::Critical | InsightCategory::Warning))
        .collect();
    assert!(serious_insights.len() >= 3, "Should have multiple serious insights");
}

#[tokio::test]
async fn test_diagnostic_metadata() {
    let agent = DiagnosticAgent::new();
    let metadata = agent.metadata();

    assert_eq!(metadata.id, "diagnostic");
    assert_eq!(metadata.name, "Diagnostic Technician");
    assert_eq!(metadata.domain, AgentDomain::Diagnostics);
    assert_eq!(metadata.icon, "🔧");
    assert_eq!(metadata.model_preference, ModelPreference::Medium);
}

#[tokio::test]
async fn test_diagnostic_can_handle_any_session() {
    let agent = DiagnosticAgent::new();
    let request = create_test_request(200.0, 220.0, 5500.0, vec![]);

    // Diagnostic agent can handle any session with sensor data
    assert!(agent.can_handle(&request));
}

#[tokio::test]
async fn test_diagnostic_token_estimation() {
    let agent = DiagnosticAgent::new();
    let request = create_test_request(200.0, 220.0, 5500.0, vec![]);

    let tokens = agent.estimate_tokens(&request);

    assert!(tokens > 0, "Should estimate token usage");
    assert!(tokens >= 1000, "Diagnostic is complex, should estimate higher token usage");
}
