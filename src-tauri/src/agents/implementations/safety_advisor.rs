/// Safety Advisor Agent
/// Analyzes driving sessions for risky behaviors and safety concerns
use async_trait::async_trait;
use chrono::Utc;

use crate::agents::{
    AgentConfig, AgentDomain, AgentError, AgentMetadata, AnalysisAgent, AnalysisRequest,
    AnalysisResult, ExecutionLocation, Insight, InsightCategory, ModelPreference, Priority,
    PrivacyLevel, Recommendation, ResultMetadata, EffortLevel,
};

pub struct SafetyAdvisorAgent {
    config: AgentConfig,
}

impl SafetyAdvisorAgent {
    pub fn new() -> Self {
        Self {
            config: AgentConfig::default(),
        }
    }

    /// Calculate safety score based on risky behaviors
    fn calculate_safety_score(&self, request: &AnalysisRequest) -> f32 {
        let session = &request.session_data;
        let mut score = 100.0;

        // Penalties for unsafe behaviors
        let speeding_penalty = session.speed_violations_count as f32 * 10.0;
        let braking_penalty = session.hard_braking_events.len() as f32 * 5.0;
        let acceleration_penalty = session.rapid_acceleration_events.len() as f32 * 3.0;

        score -= speeding_penalty;
        score -= braking_penalty;
        score -= acceleration_penalty;

        // Extra penalty for excessive speed
        if session.max_speed_mph > 85.0 {
            score -= 20.0;
        }

        score.max(0.0)
    }

    /// Generate safety insights
    fn generate_insights(&self, request: &AnalysisRequest) -> Vec<Insight> {
        let session = &request.session_data;
        let mut insights = Vec::new();
        let mut counter = 0;

        // Speed violations
        if session.speed_violations_count > 0 {
            let category = if session.speed_violations_count > 5 {
                InsightCategory::Critical
            } else {
                InsightCategory::Warning
            };

            insights.push(Insight {
                id: Some(format!("safety_insight_{}", counter)),
                category,
                title: "Speed Limit Violations Detected".to_string(),
                description: format!(
                    "{} speed violations detected with max speed of {:.0} mph. Speeding increases crash risk by 5x according to NHTSA.",
                    session.speed_violations_count,
                    session.max_speed_mph
                ),
                supporting_data: [
                    ("violation_count".to_string(), serde_json::json!(session.speed_violations_count)),
                    ("max_speed".to_string(), serde_json::json!(session.max_speed_mph)),
                    ("avg_speed".to_string(), serde_json::json!(session.avg_speed_mph)),
                ].iter().cloned().collect(),
                related_insight_ids: vec![],
            });
            counter += 1;
        }

        // Hard braking events
        if !session.hard_braking_events.is_empty() {
            insights.push(Insight {
                id: Some(format!("safety_insight_{}", counter)),
                category: InsightCategory::Warning,
                title: "Frequent Hard Braking".to_string(),
                description: format!(
                    "{} hard braking events detected. This indicates insufficient following distance or late hazard recognition.",
                    session.hard_braking_events.len()
                ),
                supporting_data: [
                    ("event_count".to_string(), serde_json::json!(session.hard_braking_events.len())),
                ].iter().cloned().collect(),
                related_insight_ids: vec![],
            });
            counter += 1;
        }

        // Rapid acceleration with safety context
        if session.rapid_acceleration_events.len() > 3 {
            insights.push(Insight {
                id: Some(format!("safety_insight_{}", counter)),
                category: InsightCategory::Info,
                title: "Aggressive Acceleration Pattern".to_string(),
                description: format!(
                    "{} rapid acceleration events. Aggressive driving reduces reaction time and increases accident risk.",
                    session.rapid_acceleration_events.len()
                ),
                supporting_data: [
                    ("event_count".to_string(), serde_json::json!(session.rapid_acceleration_events.len())),
                ].iter().cloned().collect(),
                related_insight_ids: vec![],
            });
            counter += 1;
        }

        // Overall safety summary
        let score = self.calculate_safety_score(request);
        let category = if score >= 80.0 {
            InsightCategory::Positive
        } else if score >= 60.0 {
            InsightCategory::Warning
        } else {
            InsightCategory::Critical
        };

        insights.push(Insight {
            id: Some(format!("safety_insight_{}", counter)),
            category,
            title: "Driving Safety Assessment".to_string(),
            description: format!(
                "Safety score: {:.0}/100. {} driving behavior observed during this session.",
                score,
                if score >= 80.0 { "Safe" } else if score >= 60.0 { "Moderately safe" } else { "Risky" }
            ),
            supporting_data: [
                ("safety_score".to_string(), serde_json::json!(score)),
                ("duration_minutes".to_string(), serde_json::json!(session.duration_seconds / 60)),
            ].iter().cloned().collect(),
            related_insight_ids: vec![],
        });

        insights
    }

    /// Generate safety recommendations
    fn generate_recommendations(&self, request: &AnalysisRequest) -> Vec<Recommendation> {
        let session = &request.session_data;
        let mut recommendations = Vec::new();

        // Speed-related recommendations
        if session.speed_violations_count > 0 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: "Strictly observe posted speed limits and adjust speed for conditions".to_string(),
                expected_benefit: "Reduce crash risk by 5x and improve emergency response time by 40%".to_string(),
                effort_level: EffortLevel::Easy,
                category: "safety".to_string(),
                estimated_impact: Some("Crash risk reduction".to_string()),
                related_insight_id: None,
            });
        }

        // Following distance recommendation
        if !session.hard_braking_events.is_empty() {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: "Maintain 3-second following distance (6 seconds in bad weather)".to_string(),
                expected_benefit: "Reduce rear-end collision risk by 75% and eliminate panic braking".to_string(),
                effort_level: EffortLevel::Moderate,
                category: "safety".to_string(),
                estimated_impact: Some("Collision prevention".to_string()),
                related_insight_id: None,
            });
        }

        // Defensive driving recommendation
        if session.rapid_acceleration_events.len() > 3 {
            recommendations.push(Recommendation {
                priority: Priority::Medium,
                action: "Practice smooth, predictable acceleration and braking".to_string(),
                expected_benefit: "Improve vehicle control and reduce accident risk by 30%".to_string(),
                effort_level: EffortLevel::Moderate,
                category: "safety".to_string(),
                estimated_impact: Some("Accident risk reduction".to_string()),
                related_insight_id: None,
            });
        }

        // Excessive speed recommendation
        if session.max_speed_mph > 85.0 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: "Avoid excessive speeds over 80 mph - risk of fatal crash doubles above 75 mph".to_string(),
                expected_benefit: "Dramatically reduce fatal crash risk and maintain vehicle stability".to_string(),
                effort_level: EffortLevel::Easy,
                category: "safety".to_string(),
                estimated_impact: Some("Fatal crash risk reduction".to_string()),
                related_insight_id: None,
            });
        }

        // General safety tip
        recommendations.push(Recommendation {
            priority: Priority::Low,
            action: "Scan 12-15 seconds ahead for potential hazards".to_string(),
            expected_benefit: "Improve hazard detection and reaction time for safer driving".to_string(),
            effort_level: EffortLevel::Moderate,
            category: "safety".to_string(),
            estimated_impact: Some("Hazard detection improvement".to_string()),
            related_insight_id: None,
        });

        recommendations
    }
}

impl Default for SafetyAdvisorAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AnalysisAgent for SafetyAdvisorAgent {
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError> {
        // TODO: Replace with real AI provider call
        // For now, use rule-based analysis

        let start = std::time::Instant::now();

        let score = self.calculate_safety_score(request);
        let insights = self.generate_insights(request);
        let recommendations = self.generate_recommendations(request);

        let elapsed = start.elapsed().as_millis() as u64;

        Ok(AnalysisResult {
            agent_id: "safety-advisor".to_string(),
            timestamp: Utc::now(),
            score: Some(score),
            confidence: 0.9, // High confidence for rule-based safety analysis
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
            id: "safety-advisor".to_string(),
            name: "Safety Advisor".to_string(),
            domain: AgentDomain::Safety,
            description: "Analyzes driving behavior for safety risks including speeding, hard braking, and aggressive maneuvers. References NHTSA safety guidelines.".to_string(),
            model_preference: ModelPreference::Lightweight,
            privacy_level: PrivacyLevel::CloudPreferred,
            icon: "🛡️".to_string(),
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
        // Can handle any driving session
        true
    }

    fn estimate_tokens(&self, _request: &AnalysisRequest) -> u32 {
        // Estimated tokens for AI call (for future use)
        900 // ~550 input + ~350 output
    }
}
