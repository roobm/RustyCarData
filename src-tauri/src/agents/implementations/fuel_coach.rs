/// Fuel Efficiency Coach Agent
/// Analyzes driving sessions to provide fuel-saving recommendations
use async_trait::async_trait;
use chrono::Utc;

use crate::agents::{
    AgentConfig, AgentDomain, AgentError, AgentMetadata, AnalysisAgent, AnalysisRequest,
    AnalysisResult, ExecutionLocation, Insight, InsightCategory, ModelPreference, Priority,
    PrivacyLevel, Recommendation, ResultMetadata, EffortLevel,
};

pub struct FuelCoachAgent {
    config: AgentConfig,
}

impl FuelCoachAgent {
    pub fn new() -> Self {
        Self {
            config: AgentConfig::default(),
        }
    }

    /// Calculate fuel efficiency score based on MPG and driving patterns
    fn calculate_fuel_score(&self, request: &AnalysisRequest) -> f32 {
        let session = &request.session_data;
        let mpg = session.avg_fuel_efficiency_mpg;

        // Base score from fuel efficiency (normalized to 0-100)
        let mut score = (mpg / 50.0 * 100.0).min(100.0);

        // Penalties for inefficient driving
        if session.avg_rpm > 3000.0 {
            score -= 10.0;
        }
        if session.high_rpm_duration_seconds > 300 {
            score -= 15.0;
        }
        if session.rapid_acceleration_events.len() > 5 {
            score -= 10.0;
        }

        score.max(0.0)
    }

    /// Generate fuel-saving insights
    fn generate_insights(&self, request: &AnalysisRequest) -> Vec<Insight> {
        let session = &request.session_data;
        let mut insights = Vec::new();

        // High RPM insight
        if session.avg_rpm > 3000.0 {
            insights.push(Insight {
                category: InsightCategory::Warning,
                title: "High Engine RPM Detected".to_string(),
                description: format!(
                    "Average RPM of {:.0} is higher than optimal. Shifting earlier can improve fuel economy by 10-15%.",
                    session.avg_rpm
                ),
                supporting_data: [
                    ("avg_rpm".to_string(), serde_json::json!(session.avg_rpm)),
                    ("high_rpm_duration".to_string(), serde_json::json!(session.high_rpm_duration_seconds)),
                ].iter().cloned().collect(),
                related_insight_ids: vec![],
            });
        }

        // Rapid acceleration insight
        if !session.rapid_acceleration_events.is_empty() {
            insights.push(Insight {
                category: InsightCategory::Info,
                title: "Frequent Rapid Acceleration".to_string(),
                description: format!(
                    "{} rapid acceleration events detected. Gradual acceleration can save up to 20% fuel.",
                    session.rapid_acceleration_events.len()
                ),
                supporting_data: [
                    ("event_count".to_string(), serde_json::json!(session.rapid_acceleration_events.len())),
                ].iter().cloned().collect(),
                related_insight_ids: vec![],
            });
        }

        // Overall fuel efficiency insight
        let mpg = session.avg_fuel_efficiency_mpg;
        let category = if mpg < 20.0 {
            InsightCategory::Critical
        } else if mpg < 25.0 {
            InsightCategory::Warning
        } else {
            InsightCategory::Positive
        };

        insights.push(Insight {
            category,
            title: "Fuel Efficiency Analysis".to_string(),
            description: format!(
                "Average fuel efficiency: {:.1} MPG over {:.1} miles. {} consumption pattern detected.",
                mpg,
                session.distance_miles,
                if mpg > 25.0 { "Efficient" } else { "Suboptimal" }
            ),
            supporting_data: [
                ("avg_mpg".to_string(), serde_json::json!(mpg)),
                ("distance".to_string(), serde_json::json!(session.distance_miles)),
                ("fuel_consumed".to_string(), serde_json::json!(session.fuel_consumed_gallons)),
            ].iter().cloned().collect(),
            related_insight_ids: vec![],
        });

        insights
    }

    /// Generate actionable recommendations
    fn generate_recommendations(&self, request: &AnalysisRequest) -> Vec<Recommendation> {
        let session = &request.session_data;
        let mut recommendations = Vec::new();

        // High RPM recommendation
        if session.avg_rpm > 3000.0 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: "Shift gears earlier to maintain RPM between 1500-2500".to_string(),
                expected_benefit: "Reduce fuel consumption by 10-15% on highway driving".to_string(),
                effort_level: EffortLevel::Easy,
                related_insight_id: None,
            });
        }

        // Acceleration recommendation
        if !session.rapid_acceleration_events.is_empty() {
            recommendations.push(Recommendation {
                priority: Priority::Medium,
                action: "Accelerate gradually - take 5 seconds to reach cruising speed".to_string(),
                expected_benefit: "Save up to 20% fuel in city driving".to_string(),
                effort_level: EffortLevel::Easy,
                related_insight_id: None,
            });
        }

        // Speed recommendation
        if session.avg_speed_mph > 65.0 {
            recommendations.push(Recommendation {
                priority: Priority::Medium,
                action: "Maintain speed around 55-60 mph on highways".to_string(),
                expected_benefit: "Improve fuel economy by 7-14% at lower highway speeds".to_string(),
                effort_level: EffortLevel::Moderate,
                related_insight_id: None,
            });
        }

        // General efficiency tip
        recommendations.push(Recommendation {
            priority: Priority::Low,
            action: "Use cruise control on highways to maintain consistent speed".to_string(),
            expected_benefit: "Reduce fuel consumption by 3-7% on long trips".to_string(),
            effort_level: EffortLevel::Easy,
            related_insight_id: None,
        });

        recommendations
    }
}

impl Default for FuelCoachAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AnalysisAgent for FuelCoachAgent {
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError> {
        // TODO: Replace with real AI provider call
        // For now, use rule-based analysis

        let start = std::time::Instant::now();

        let score = self.calculate_fuel_score(request);
        let insights = self.generate_insights(request);
        let recommendations = self.generate_recommendations(request);

        let elapsed = start.elapsed().as_millis() as u64;

        Ok(AnalysisResult {
            agent_id: "fuel-coach".to_string(),
            timestamp: Utc::now(),
            score: Some(score),
            confidence: 0.85, // High confidence for rule-based analysis
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
            id: "fuel-coach".to_string(),
            name: "Fuel Efficiency Coach".to_string(),
            domain: AgentDomain::FuelEfficiency,
            description: "Analyzes driving patterns to provide fuel-saving recommendations based on RPM, acceleration, and speed data.".to_string(),
            model_preference: ModelPreference::Lightweight,
            privacy_level: PrivacyLevel::CloudPreferred,
            icon: "⛽".to_string(),
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

    fn can_handle(&self, request: &AnalysisRequest) -> bool {
        // Can handle any session with fuel data
        request.session_data.fuel_consumed_gallons > 0.0
    }

    fn estimate_tokens(&self, _request: &AnalysisRequest) -> u32 {
        // Estimated tokens for AI call (for future use)
        800 // ~500 input + ~300 output
    }
}
