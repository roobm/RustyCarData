/// Diagnostic Agent
/// Analyzes vehicle sensor data to detect potential mechanical issues
use async_trait::async_trait;
use chrono::Utc;

use crate::agents::{
    AgentConfig, AgentDomain, AgentError, AgentMetadata, AnalysisAgent, AnalysisRequest,
    AnalysisResult, ExecutionLocation, Insight, InsightCategory, ModelPreference, Priority,
    PrivacyLevel, Recommendation, ResultMetadata, EffortLevel,
};

pub struct DiagnosticAgent {
    config: AgentConfig,
}

impl DiagnosticAgent {
    pub fn new() -> Self {
        Self {
            config: AgentConfig::default(),
        }
    }

    /// Calculate vehicle health score based on sensor readings
    fn calculate_health_score(&self, request: &AnalysisRequest) -> f32 {
        let session = &request.session_data;
        let mut score = 100.0;

        // Temperature concerns
        if session.max_coolant_temp_f > 220.0 {
            score -= 15.0;
        } else if session.max_coolant_temp_f > 210.0 {
            score -= 5.0;
        }

        if session.avg_oil_temp_f > 250.0 {
            score -= 10.0;
        }

        // RPM patterns
        if session.max_rpm > 6000.0 {
            score -= 10.0;
        }
        if session.high_rpm_duration_seconds > 600 {
            score -= 8.0;
        }

        // Error codes are critical
        let error_penalty = session.error_codes.len() as f32 * 20.0;
        score -= error_penalty;

        score.max(0.0)
    }

    /// Generate diagnostic insights
    fn generate_insights(&self, request: &AnalysisRequest) -> Vec<Insight> {
        let session = &request.session_data;
        let mut insights = Vec::new();

        // Coolant temperature analysis
        if session.max_coolant_temp_f > 220.0 {
            insights.push(Insight {
                category: InsightCategory::Critical,
                title: "Elevated Coolant Temperature".to_string(),
                description: format!(
                    "Peak coolant temperature of {:.0}°F exceeds normal range (195-220°F). This may indicate cooling system issues: low coolant, failing thermostat, or radiator blockage.",
                    session.max_coolant_temp_f
                ),
                supporting_data: [
                    ("max_coolant_temp".to_string(), serde_json::json!(session.max_coolant_temp_f)),
                    ("avg_coolant_temp".to_string(), serde_json::json!(session.avg_coolant_temp_f)),
                ].iter().cloned().collect(),
                related_insight_ids: vec![],
            });
        } else if session.max_coolant_temp_f > 210.0 {
            insights.push(Insight {
                category: InsightCategory::Warning,
                title: "Coolant Temperature Near Upper Limit".to_string(),
                description: format!(
                    "Coolant temperature reached {:.0}°F. Monitor closely and check coolant level.",
                    session.max_coolant_temp_f
                ),
                supporting_data: [
                    ("max_coolant_temp".to_string(), serde_json::json!(session.max_coolant_temp_f)),
                ].iter().cloned().collect(),
                related_insight_ids: vec![],
            });
        }

        // Oil temperature analysis
        if session.avg_oil_temp_f > 250.0 {
            insights.push(Insight {
                category: InsightCategory::Warning,
                title: "High Oil Temperature".to_string(),
                description: format!(
                    "Average oil temperature of {:.0}°F is elevated. High oil temps (>250°F) accelerate oil breakdown and reduce lubrication effectiveness.",
                    session.avg_oil_temp_f
                ),
                supporting_data: [
                    ("avg_oil_temp".to_string(), serde_json::json!(session.avg_oil_temp_f)),
                ].iter().cloned().collect(),
                related_insight_ids: vec![],
            });
        }

        // RPM analysis
        if session.max_rpm > 6000.0 {
            insights.push(Insight {
                category: InsightCategory::Warning,
                title: "Excessive Engine RPM".to_string(),
                description: format!(
                    "Peak RPM of {:.0} detected. Sustained high RPM accelerates engine wear and increases risk of mechanical failure.",
                    session.max_rpm
                ),
                supporting_data: [
                    ("max_rpm".to_string(), serde_json::json!(session.max_rpm)),
                    ("high_rpm_duration".to_string(), serde_json::json!(session.high_rpm_duration_seconds)),
                ].iter().cloned().collect(),
                related_insight_ids: vec![],
            });
        }

        // Error code analysis
        if !session.error_codes.is_empty() {
            for error in &session.error_codes {
                insights.push(Insight {
                    category: InsightCategory::Critical,
                    title: format!("Diagnostic Trouble Code: {}", error.code),
                    description: error.description.clone(),
                    supporting_data: [
                        ("code".to_string(), serde_json::json!(error.code)),
                        ("severity".to_string(), serde_json::json!(format!("{:?}", error.severity))),
                    ].iter().cloned().collect(),
                    related_insight_ids: vec![],
                });
            }
        }

        // Overall health assessment
        let score = self.calculate_health_score(request);
        let category = if score >= 85.0 {
            InsightCategory::Positive
        } else if score >= 70.0 {
            InsightCategory::Info
        } else if score >= 50.0 {
            InsightCategory::Warning
        } else {
            InsightCategory::Critical
        };

        insights.push(Insight {
            category,
            title: "Vehicle Health Assessment".to_string(),
            description: format!(
                "Overall vehicle health score: {:.0}/100. {} condition detected based on sensor analysis.",
                score,
                if score >= 85.0 { "Excellent" }
                else if score >= 70.0 { "Good" }
                else if score >= 50.0 { "Fair - attention needed" }
                else { "Poor - immediate inspection recommended" }
            ),
            supporting_data: [
                ("health_score".to_string(), serde_json::json!(score)),
                ("error_count".to_string(), serde_json::json!(session.error_codes.len())),
            ].iter().cloned().collect(),
            related_insight_ids: vec![],
        });

        insights
    }

    /// Generate maintenance recommendations
    fn generate_recommendations(&self, request: &AnalysisRequest) -> Vec<Recommendation> {
        let session = &request.session_data;
        let mut recommendations = Vec::new();

        // Cooling system recommendations
        if session.max_coolant_temp_f > 220.0 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: "Inspect cooling system immediately - check coolant level, thermostat, and radiator".to_string(),
                expected_benefit: "Prevent engine overheating and potential head gasket failure ($2000+ repair)".to_string(),
                effort_level: EffortLevel::Moderate,
                related_insight_id: None,
            });
        } else if session.max_coolant_temp_f > 210.0 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: "Check coolant level and inspect for leaks".to_string(),
                expected_benefit: "Maintain optimal engine temperature and prevent overheating".to_string(),
                effort_level: EffortLevel::Easy,
                related_insight_id: None,
            });
        }

        // Oil temperature recommendations
        if session.avg_oil_temp_f > 250.0 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: "Consider synthetic oil upgrade and verify oil level is correct".to_string(),
                expected_benefit: "Better high-temperature protection and extended engine life".to_string(),
                effort_level: EffortLevel::Moderate,
                related_insight_id: None,
            });
        }

        // High RPM recommendations
        if session.max_rpm > 6000.0 || session.high_rpm_duration_seconds > 600 {
            recommendations.push(Recommendation {
                priority: Priority::Medium,
                action: "Reduce high RPM driving - shift earlier and avoid redline operation".to_string(),
                expected_benefit: "Reduce engine wear and extend component lifespan by 30%".to_string(),
                effort_level: EffortLevel::Easy,
                related_insight_id: None,
            });
        }

        // Error code recommendations
        if !session.error_codes.is_empty() {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: format!("Address {} diagnostic trouble code(s) with professional scan and repair", session.error_codes.len()),
                expected_benefit: "Prevent cascading failures and restore optimal vehicle performance".to_string(),
                effort_level: EffortLevel::Significant,
                related_insight_id: None,
            });
        }

        // Preventive maintenance
        recommendations.push(Recommendation {
            priority: Priority::Low,
            action: "Schedule routine maintenance inspection within 1000 miles".to_string(),
            expected_benefit: "Catch potential issues early and maintain reliability".to_string(),
            effort_level: EffortLevel::Easy,
            related_insight_id: None,
        });

        recommendations
    }
}

impl Default for DiagnosticAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AnalysisAgent for DiagnosticAgent {
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError> {
        // TODO: Replace with real AI provider call
        // For now, use rule-based analysis

        let start = std::time::Instant::now();

        let score = self.calculate_health_score(request);
        let insights = self.generate_insights(request);
        let recommendations = self.generate_recommendations(request);

        let elapsed = start.elapsed().as_millis() as u64;

        Ok(AnalysisResult {
            agent_id: "diagnostic".to_string(),
            timestamp: Utc::now(),
            score: Some(score),
            confidence: 0.8, // Good confidence for rule-based diagnostics
            insights,
            recommendations,
            execution_time_ms: elapsed,
            model_used: "rule-based-v1".to_string(),
            metadata: ResultMetadata {
                from_cache: false,
                cache_key: None,
                tokens_consumed: None,
                provider: "local-rules".to_string(),
                execution_location: ExecutionLocation::Local,
            },
        })
    }

    fn metadata(&self) -> AgentMetadata {
        AgentMetadata {
            id: "diagnostic".to_string(),
            name: "Diagnostic Technician".to_string(),
            domain: AgentDomain::Diagnostics,
            description: "Analyzes vehicle sensor data (temperature, RPM, error codes) to detect mechanical issues and predict maintenance needs.".to_string(),
            model_preference: ModelPreference::Medium,
            privacy_level: PrivacyLevel::CloudPreferred,
            icon: "🔧".to_string(),
            version: "1.0.0".to_string(),
        }
    }

    fn config(&self) -> &AgentConfig {
        &self.config
    }

    fn set_config(&mut self, config: AgentConfig) -> Result<(), AgentError> {
        self.config = config;
        Ok(())
    }

    fn can_handle(&self, _request: &AnalysisRequest) -> bool {
        // Can handle any session with sensor data
        true
    }

    fn estimate_tokens(&self, _request: &AnalysisRequest) -> u32 {
        // Estimated tokens for AI call (for future use)
        1200 // ~700 input + ~500 output (more complex than other agents)
    }
}
