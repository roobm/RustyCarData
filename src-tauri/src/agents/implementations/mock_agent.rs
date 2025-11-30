/// Mock agent for testing the framework
/// This agent doesn't call any AI - it just returns hardcoded results
use async_trait::async_trait;
use chrono::Utc;

use crate::agents::{
    AgentConfig, AgentDomain, AgentError, AgentMetadata, AnalysisAgent, AnalysisRequest,
    AnalysisResult, ExecutionLocation, Insight, InsightCategory, ModelPreference, Priority,
    PrivacyLevel, Recommendation, ResultMetadata, EffortLevel,
};

pub struct MockAgent {
    config: AgentConfig,
    agent_id: String,
}

impl MockAgent {
    pub fn new(agent_id: String) -> Self {
        Self {
            config: AgentConfig::default(),
            agent_id,
        }
    }
}

#[async_trait]
impl AnalysisAgent for MockAgent {
    async fn analyze(&self, request: &AnalysisRequest) -> Result<AnalysisResult, AgentError> {
        // Simulate some work
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        Ok(AnalysisResult {
            agent_id: self.agent_id.clone(),
            timestamp: Utc::now(),
            score: Some(85.0),
            confidence: 0.9,
            insights: vec![Insight {
                id: Some("mock_insight_1".to_string()),
                category: InsightCategory::Info,
                title: format!("Mock insight from {}", self.agent_id),
                description: format!(
                    "This is a test insight for session {}",
                    request.session_data.session_id
                ),
                supporting_data: Default::default(),
                related_insight_ids: vec![],
            }],
            recommendations: vec![Recommendation {
                priority: Priority::Medium,
                action: "This is a mock recommendation".to_string(),
                expected_benefit: "Testing the framework".to_string(),
                effort_level: EffortLevel::Easy,
                related_insight_id: None,
                estimated_impact: None,
                category: "testing".to_string(),
            }],
            execution_time_ms: 100,
            model_used: "mock-v1".to_string(),
            metadata: ResultMetadata {
                from_cache: false,
                cache_key: None,
                tokens_consumed: None,
                provider: "mock".to_string(),
                execution_location: ExecutionLocation::Local,
            },
        })
    }

    fn metadata(&self) -> AgentMetadata {
        AgentMetadata {
            id: self.agent_id.clone(),
            name: format!("Mock Agent ({})", self.agent_id),
            domain: AgentDomain::Custom("testing".to_string()),
            description: "A mock agent for testing the framework".to_string(),
            model_preference: ModelPreference::Lightweight,
            privacy_level: PrivacyLevel::CloudPreferred,
            icon: "🧪".to_string(),
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
        true
    }

    fn estimate_tokens(&self, _request: &AnalysisRequest) -> u32 {
        500 // Mock estimate
    }
}
