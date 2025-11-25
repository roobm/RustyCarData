/// Integration tests for the agent framework
/// This tests the complete flow: Registry → Orchestrator → MockAgent
use std::sync::Arc;
use tauri_app_lib::agents::*;
use tauri_app_lib::agents::implementations::MockAgent;
use chrono::Utc;

/// Helper to create a test AnalysisRequest
fn create_test_request() -> AnalysisRequest {
    AnalysisRequest {
        request_id: "test-001".to_string(),
        session_data: SessionSummary {
            session_id: "session-001".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now(),
            duration_seconds: 1800,
            distance_miles: 25.0,
            fuel_consumed_gallons: 1.0,
            avg_fuel_efficiency_mpg: 25.0,
            avg_speed_mph: 45.0,
            max_speed_mph: 70.0,
            speed_violations_count: 0,
            avg_rpm: 2500.0,
            max_rpm: 5000.0,
            high_rpm_duration_seconds: 0,
            avg_coolant_temp_f: 195.0,
            max_coolant_temp_f: 210.0,
            avg_oil_temp_f: 220.0,
            hard_braking_events: vec![],
            rapid_acceleration_events: vec![],
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
async fn test_agent_registry() {
    let mut registry = AgentRegistry::new();

    // Register a mock agent
    let agent = Arc::new(MockAgent::new("test-agent-1".to_string()));
    registry.register(agent.clone());

    // Verify we can retrieve it
    let retrieved = registry.get("test-agent-1");
    assert!(retrieved.is_ok());

    // Verify metadata
    let metadata = retrieved.unwrap().metadata();
    assert_eq!(metadata.id, "test-agent-1");
    assert_eq!(metadata.icon, "🧪");

    // Verify count
    assert_eq!(registry.count(), 1);

    // Test listing all agents
    let all_agents = registry.list();
    assert_eq!(all_agents.len(), 1);
    assert_eq!(all_agents[0].id, "test-agent-1");
}

#[tokio::test]
async fn test_agent_not_found() {
    let registry = AgentRegistry::new();
    let result = registry.get("nonexistent-agent");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AgentError::NotFound(_)));
}

#[tokio::test]
async fn test_mock_agent_analysis() {
    let agent = MockAgent::new("test-agent".to_string());
    let request = create_test_request();

    let result = agent.analyze(&request).await;
    assert!(result.is_ok());

    let analysis = result.unwrap();
    assert_eq!(analysis.agent_id, "test-agent");
    assert_eq!(analysis.score, Some(85.0));
    assert_eq!(analysis.confidence, 0.9);
    assert_eq!(analysis.insights.len(), 1);
    assert_eq!(analysis.recommendations.len(), 1);
    assert_eq!(analysis.model_used, "mock-v1");
}

#[tokio::test]
async fn test_orchestrator_single_agent() {
    // Create registry with one mock agent
    let mut registry = AgentRegistry::new();
    registry.register(Arc::new(MockAgent::new("agent-1".to_string())));

    // Create orchestrator
    let orchestrator = AgentOrchestrator::new(registry);

    // Analyze session
    let request = create_test_request();
    let results = orchestrator
        .analyze_session(&request, vec!["agent-1".to_string()])
        .await;

    // Verify results
    assert_eq!(results.len(), 1);
    assert!(results[0].is_ok());

    let analysis = results[0].as_ref().unwrap();
    assert_eq!(analysis.agent_id, "agent-1");
}

#[tokio::test]
async fn test_orchestrator_parallel_agents() {
    // Create registry with 3 mock agents
    let mut registry = AgentRegistry::new();
    registry.register(Arc::new(MockAgent::new("agent-1".to_string())));
    registry.register(Arc::new(MockAgent::new("agent-2".to_string())));
    registry.register(Arc::new(MockAgent::new("agent-3".to_string())));

    // Create orchestrator
    let orchestrator = AgentOrchestrator::new(registry);

    // Analyze session with all 3 agents in parallel
    let request = create_test_request();
    let start = std::time::Instant::now();

    let results = orchestrator
        .analyze_session(
            &request,
            vec![
                "agent-1".to_string(),
                "agent-2".to_string(),
                "agent-3".to_string(),
            ],
        )
        .await;

    let elapsed = start.elapsed();

    // Verify all 3 agents completed
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.is_ok()));

    // Verify parallel execution (should take ~100ms, not 300ms)
    // Each agent sleeps for 100ms, but they run in parallel
    assert!(
        elapsed.as_millis() < 250,
        "Parallel execution took too long: {:?}",
        elapsed
    );

    // Verify each agent returned correct data
    for (i, result) in results.iter().enumerate() {
        let analysis = result.as_ref().unwrap();
        assert_eq!(analysis.agent_id, format!("agent-{}", i + 1));
        assert_eq!(analysis.score, Some(85.0));
    }
}

#[tokio::test]
async fn test_orchestrator_caching() {
    // Create registry with one mock agent
    let mut registry = AgentRegistry::new();
    registry.register(Arc::new(MockAgent::new("agent-1".to_string())));

    // Create orchestrator
    let orchestrator = AgentOrchestrator::new(registry);
    let request = create_test_request();

    // First call - should execute agent
    let results1 = orchestrator
        .analyze_session(&request, vec!["agent-1".to_string()])
        .await;
    assert!(results1[0].is_ok());
    let first_result = results1[0].as_ref().unwrap();
    assert_eq!(first_result.metadata.from_cache, false);

    // Second call with same request - should return cached result
    let results2 = orchestrator
        .analyze_session(&request, vec!["agent-1".to_string()])
        .await;
    assert!(results2[0].is_ok());
    let second_result = results2[0].as_ref().unwrap();
    assert_eq!(second_result.metadata.from_cache, true);
    assert!(second_result.metadata.cache_key.is_some());

    // Verify results are identical
    assert_eq!(first_result.agent_id, second_result.agent_id);
    assert_eq!(first_result.score, second_result.score);
}

#[tokio::test]
async fn test_cache_invalidation() {
    // Create registry with one mock agent
    let mut registry = AgentRegistry::new();
    registry.register(Arc::new(MockAgent::new("agent-1".to_string())));

    // Create orchestrator
    let orchestrator = AgentOrchestrator::new(registry);
    let request = create_test_request();

    // First call
    let results1 = orchestrator
        .analyze_session(&request, vec!["agent-1".to_string()])
        .await;
    assert!(results1[0].is_ok());

    // Invalidate cache for this session
    orchestrator.invalidate_cache("session-001").await;

    // Second call - should NOT be from cache
    let results2 = orchestrator
        .analyze_session(&request, vec!["agent-1".to_string()])
        .await;
    assert!(results2[0].is_ok());
    let second_result = results2[0].as_ref().unwrap();
    assert_eq!(second_result.metadata.from_cache, false);
}

#[tokio::test]
async fn test_agent_config_changes_invalidate_cache() {
    // Create registry with one mock agent
    let mut registry = AgentRegistry::new();
    registry.register(Arc::new(MockAgent::new("agent-1".to_string())));

    // Create orchestrator
    let orchestrator = AgentOrchestrator::new(registry);
    let request = create_test_request();

    // First call with default config
    let results1 = orchestrator
        .analyze_session(&request, vec!["agent-1".to_string()])
        .await;
    assert!(results1[0].is_ok());

    // TODO: Change agent config and verify cache miss
    // This requires the ability to update agent config in the registry
    // Will be implemented when we add the update_agent_config command
}
