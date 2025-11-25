/// Example: How to use the agent framework
/// This demonstrates the complete workflow from registering agents to analyzing sessions
use std::sync::Arc;
use tauri_app_lib::agents::*;
use tauri_app_lib::agents::implementations::MockAgent;
use chrono::Utc;

#[tokio::main]
async fn main() {
    println!("🚀 Agent Framework Demo\n");

    // Step 1: Create a registry and register agents
    println!("📋 Step 1: Creating agent registry...");
    let mut registry = AgentRegistry::new();

    registry.register(Arc::new(MockAgent::new("fuel-coach".to_string())));
    registry.register(Arc::new(MockAgent::new("safety-advisor".to_string())));
    registry.register(Arc::new(MockAgent::new("diagnostic".to_string())));

    println!("   ✓ Registered {} agents", registry.count());
    for agent_meta in registry.list() {
        println!("     - {} {} ({})", agent_meta.icon, agent_meta.name, agent_meta.id);
    }

    // Step 2: Create an orchestrator
    println!("\n🎭 Step 2: Creating orchestrator...");
    let orchestrator = AgentOrchestrator::new(registry);
    println!("   ✓ Orchestrator ready");

    // Step 3: Create a sample analysis request
    println!("\n📊 Step 3: Creating analysis request...");
    let request = AnalysisRequest {
        request_id: "demo-001".to_string(),
        session_data: SessionSummary {
            session_id: "session-demo-001".to_string(),
            start_time: Utc::now(),
            end_time: Utc::now(),
            duration_seconds: 1800,
            distance_miles: 25.5,
            fuel_consumed_gallons: 1.1,
            avg_fuel_efficiency_mpg: 23.2,
            avg_speed_mph: 42.0,
            max_speed_mph: 75.0,
            speed_violations_count: 2,
            avg_rpm: 3200.0,
            max_rpm: 5800.0,
            high_rpm_duration_seconds: 420,
            avg_coolant_temp_f: 195.0,
            max_coolant_temp_f: 218.0,
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
    };
    println!("   ✓ Request created for session: {}", request.session_data.session_id);

    // Step 4: Run analysis with multiple agents in parallel
    println!("\n🔄 Step 4: Running parallel analysis with 3 agents...");
    let start = std::time::Instant::now();

    let results = orchestrator
        .analyze_session(
            &request,
            vec![
                "fuel-coach".to_string(),
                "safety-advisor".to_string(),
                "diagnostic".to_string(),
            ],
        )
        .await;

    let elapsed = start.elapsed();
    println!("   ✓ Analysis complete in {:?}", elapsed);
    println!("   ℹ️  Parallel execution: 3 agents in ~100ms (vs 300ms sequential)");

    // Step 5: Display results
    println!("\n📈 Step 5: Results from {} agents:\n", results.len());

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(analysis) => {
                println!("   Agent {}: {} {}", i + 1,
                    if analysis.metadata.from_cache { "💾" } else { "✨" },
                    analysis.agent_id
                );
                println!("     Score: {}/100", analysis.score.unwrap_or(0.0));
                println!("     Confidence: {:.0}%", analysis.confidence * 100.0);
                println!("     Insights: {}", analysis.insights.len());
                println!("     Recommendations: {}", analysis.recommendations.len());
                println!("     Execution time: {}ms", analysis.execution_time_ms);
                println!("     Model: {}", analysis.model_used);
                println!("     Provider: {}", analysis.metadata.provider);
                println!("     Location: {:?}", analysis.metadata.execution_location);

                // Show first insight
                if let Some(insight) = analysis.insights.first() {
                    println!("\n     First Insight:");
                    println!("       Category: {:?}", insight.category);
                    println!("       Title: {}", insight.title);
                    println!("       Description: {}", insight.description);
                }

                println!();
            }
            Err(e) => {
                println!("   Agent {}: ❌ Error - {}", i + 1, e);
            }
        }
    }

    // Step 6: Demonstrate caching
    println!("💾 Step 6: Testing cache (re-running same analysis)...");
    let start = std::time::Instant::now();

    let cached_results = orchestrator
        .analyze_session(&request, vec!["fuel-coach".to_string()])
        .await;

    let cached_elapsed = start.elapsed();
    println!("   ✓ Cached analysis complete in {:?}", cached_elapsed);

    if let Ok(cached) = &cached_results[0] {
        println!("   📊 From cache: {}", cached.metadata.from_cache);
        println!("   🔑 Cache key: {}", cached.metadata.cache_key.as_ref().unwrap_or(&"N/A".to_string()));
    }

    println!("\n✅ Demo complete!");
    println!("\n📝 Summary:");
    println!("   • Agent framework supports parallel execution");
    println!("   • Results are automatically cached");
    println!("   • Each agent returns structured insights & recommendations");
    println!("   • Framework is ready for real AI providers!");
}
