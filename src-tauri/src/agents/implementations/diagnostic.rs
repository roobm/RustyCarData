/// Diagnostic Agent
/// Analyzes vehicle sensor data to detect potential mechanical issues
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::agents::{
    AgentConfig, AgentDomain, AgentError, AgentMetadata, AnalysisAgent, AnalysisRequest,
    AnalysisResult, ExecutionLocation, Insight, InsightCategory, ModelPreference, Priority,
    PrivacyLevel, Recommendation, ResultMetadata, EffortLevel,
    providers::OllamaProvider,
};

/// System prompt for the Diagnostic Agent
const DIAGNOSTIC_SYSTEM_PROMPT: &str = r#"You are an ASE-certified automotive diagnostic technician with 20+ years of experience.
Analyze the OBD-II telemetry data to identify potential mechanical issues and maintenance needs.

ANALYSIS APPROACH:
1. **Data Overview**: Start by understanding the driving session (duration, distance, conditions)
2. **Pattern Recognition**: Identify normal vs. abnormal behavior in sensor readings
3. **Root Cause Analysis**: Connect symptoms to potential mechanical causes
4. **Risk Assessment**: Evaluate severity and urgency of any issues found
5. **Actionable Guidance**: Provide clear, prioritized recommendations

Focus on these key areas:
- Temperature patterns (coolant, oil) and thermal stress indicators
- RPM patterns and engine stress factors
- Diagnostic Trouble Codes (DTCs) and their implications
- Component wear patterns and degradation indicators
- Driving behavior impact on vehicle health

CRITICAL: Provide your analysis in JSON format with the following structure:
{
  "health_score": <number 0-100>,
  "recap": {
    "session_summary": "Brief overview of the driving session (1-2 sentences)",
    "key_findings": [
      "Finding 1: What the data shows",
      "Finding 2: Another important observation"
    ],
    "anomalies_detected": [
      "Anomaly 1: What's unusual and why it matters",
      "Anomaly 2: Another concerning pattern"
    ],
    "overall_assessment": "Summary of vehicle health and main concerns (2-3 sentences)"
  },
  "critical_issues": [
    {
      "title": "Issue name",
      "description": "Detailed explanation with actual sensor values, mechanical reasoning, and potential consequences",
      "severity": "critical|warning|info"
    }
  ],
  "warnings": [
    {
      "title": "Warning name",
      "description": "What was observed, why it's concerning, and what could happen if ignored",
      "severity": "warning"
    }
  ],
  "recommendations": [
    {
      "priority": "high|medium|low",
      "action": "Specific action to take (e.g., 'Inspect cooling system within 100 miles')",
      "expected_benefit": "What will be prevented/improved by taking this action",
      "effort": "easy|moderate|significant"
    }
  ]
}

IMPORTANT GUIDELINES:
- **Be verbose and detailed**: Provide 3-5 sentences for EACH item. This is critical - users want comprehensive explanations, not brief summaries
- **Always cite specific numbers**: Reference actual sensor values from the data (e.g., "coolant reached 225°F, which is 15°F above the safe operating range of 210°F")
- **Explain the mechanical reasoning**: For every observation, explain WHY it matters and WHAT could happen (e.g., "High coolant temps cause the engine oil to break down faster, which reduces lubrication effectiveness and can lead to bearing wear. If left unchecked, this can escalate to head gasket failure, which costs $2000-3000 to repair")
- **Provide context and comparisons**: Compare values to normal ranges (e.g., "Normal coolant temps are 195-210°F, but this session peaked at 225°F")
- **Include preventive advice**: Explain what the driver should watch for and why regular monitoring matters
- **Make it educational**: Help the driver understand their vehicle better - explain how components work and why certain patterns matter
- **Use storytelling**: Connect the dots between different data points (e.g., "The high RPMs combined with elevated oil temps suggest the engine was working hard, possibly from aggressive acceleration or climbing hills")
- **Mention cost implications**: When relevant, include repair cost estimates to emphasize importance (e.g., "Catching this early with a $150 thermostat replacement could prevent a $2500 head gasket job")

CRITICAL: Each description field should be AT LEAST 3-4 sentences. Brief one-sentence explanations are not acceptable. The user wants detailed, comprehensive analysis that teaches them about their vehicle.

If the session shows good vehicle health, still be detailed and educational - explain what the normal values indicate and why they're good signs."#;

/// Parsed response from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ParsedDiagnosticAnalysis {
    health_score: Option<f32>,
    #[serde(default)]
    recap: Option<DiagnosticRecap>,
    #[serde(default)]
    critical_issues: Vec<DiagnosticIssue>,
    #[serde(default)]
    warnings: Vec<DiagnosticIssue>,
    #[serde(default)]
    recommendations: Vec<DiagnosticRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiagnosticRecap {
    session_summary: String,
    #[serde(default)]
    key_findings: Vec<String>,
    #[serde(default)]
    anomalies_detected: Vec<String>,
    overall_assessment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiagnosticIssue {
    title: String,
    description: String,
    #[serde(default = "default_severity")]
    severity: String,
}

fn default_severity() -> String {
    "info".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiagnosticRecommendation {
    priority: String,
    action: String,
    expected_benefit: String,
    #[serde(default = "default_effort")]
    effort: String,
}

fn default_effort() -> String {
    "moderate".to_string()
}

pub struct DiagnosticAgent {
    config: AgentConfig,
    ollama: OllamaProvider,
}

impl DiagnosticAgent {
    pub fn new() -> Self {
        Self {
            config: AgentConfig::default(),
            ollama: OllamaProvider::new(),
        }
    }

    /// Parse the LLM response into structured insights and recommendations
    fn parse_diagnostic_response(&self, analysis: &ParsedDiagnosticAnalysis) -> (Vec<Insight>, Vec<Recommendation>) {
        let mut insights = Vec::new();
        let mut recommendations = Vec::new();
        let mut insight_counter = 0;
        let mut rec_counter = 0;

        // Add recap as a summary insight (if present)
        if let Some(recap) = &analysis.recap {
            let mut recap_description = String::new();

            // Session summary
            recap_description.push_str("**Session Overview:**\n");
            recap_description.push_str(&recap.session_summary);
            recap_description.push_str("\n\n");

            // Key findings
            if !recap.key_findings.is_empty() {
                recap_description.push_str("**Key Findings:**\n");
                for finding in &recap.key_findings {
                    recap_description.push_str(&format!("• {}\n", finding));
                }
                recap_description.push_str("\n");
            }

            // Anomalies detected
            if !recap.anomalies_detected.is_empty() {
                recap_description.push_str("**Anomalies Detected:**\n");
                for anomaly in &recap.anomalies_detected {
                    recap_description.push_str(&format!("⚠️ {}\n", anomaly));
                }
                recap_description.push_str("\n");
            }

            // Overall assessment
            recap_description.push_str("**Overall Assessment:**\n");
            recap_description.push_str(&recap.overall_assessment);

            insight_counter += 1;
            insights.push(Insight {
                id: Some(format!("insight_{}", insight_counter)),
                category: InsightCategory::Info,
                title: "Diagnostic Summary".to_string(),
                description: recap_description,
                supporting_data: HashMap::new(),
                related_insight_ids: vec![],
            });
        }

        // Convert critical issues to insights
        for issue in &analysis.critical_issues {
            insight_counter += 1;
            insights.push(Insight {
                id: Some(format!("insight_{}", insight_counter)),
                category: match issue.severity.to_lowercase().as_str() {
                    "critical" => InsightCategory::Critical,
                    "warning" => InsightCategory::Warning,
                    _ => InsightCategory::Info,
                },
                title: issue.title.clone(),
                description: issue.description.clone(),
                supporting_data: HashMap::new(),
                related_insight_ids: vec![],
            });
        }

        // Convert warnings to insights
        for warning in &analysis.warnings {
            insight_counter += 1;
            insights.push(Insight {
                id: Some(format!("insight_{}", insight_counter)),
                category: InsightCategory::Warning,
                title: warning.title.clone(),
                description: warning.description.clone(),
                supporting_data: HashMap::new(),
                related_insight_ids: vec![],
            });
        }

        // Convert recommendations
        for rec in &analysis.recommendations {
            rec_counter += 1;
            recommendations.push(Recommendation {
                priority: match rec.priority.to_lowercase().as_str() {
                    "high" => Priority::High,
                    "low" => Priority::Low,
                    _ => Priority::Medium,
                },
                action: rec.action.clone(),
                expected_benefit: rec.expected_benefit.clone(),
                effort_level: match rec.effort.to_lowercase().as_str() {
                    "easy" => EffortLevel::Easy,
                    "significant" => EffortLevel::Significant,
                    _ => EffortLevel::Moderate,
                },
                related_insight_id: None,
                estimated_impact: None,
                category: "maintenance".to_string(),
            });
        }

        (insights, recommendations)
    }

    /// Try to perform AI-powered analysis using Ollama
    async fn try_ai_analysis(
        &self,
        request: &AnalysisRequest,
    ) -> Result<(f32, Vec<Insight>, Vec<Recommendation>), AgentError> {
        // Check if Ollama is available
        if !self.ollama.health_check().await {
            return Err(AgentError::ProviderError(
                "Ollama service not available".to_string()
            ));
        }

        // Call Ollama with diagnostic prompt
        let response = self.ollama
            .analyze_session(request, DIAGNOSTIC_SYSTEM_PROMPT)
            .await?;

        // Log the raw Ollama response for debugging
        println!("\n=== OLLAMA RAW RESPONSE ===");
        println!("{}", response);
        println!("=== END OLLAMA RESPONSE ===\n");

        // Parse JSON response
        use crate::agents::providers::ollama::ParsedResponse;
        match self.ollama.parse_json_or_text::<ParsedDiagnosticAnalysis>(&response)? {
            ParsedResponse::Structured(analysis) => {
                let score = analysis.health_score.unwrap_or_else(|| {
                    self.calculate_health_score(request)
                });
                let (insights, recommendations) = self.parse_diagnostic_response(&analysis);
                Ok((score, insights, recommendations))
            }
            ParsedResponse::Freeform(text) => {
                // Fallback: create a single insight from freeform text
                let insights = vec![Insight {
                    id: Some("freeform_1".to_string()),
                    category: InsightCategory::Info,
                    title: "AI Analysis".to_string(),
                    description: text,
                    supporting_data: HashMap::new(),
                    related_insight_ids: vec![],
                }];
                let score = self.calculate_health_score(request);
                Ok((score, insights, vec![]))
            }
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
        let mut counter = 1;

        // Coolant temperature analysis
        if session.max_coolant_temp_f > 220.0 {
            insights.push(Insight {
                id: Some(format!("diagnostic_insight_{}", counter)),
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
            counter += 1;
        } else if session.max_coolant_temp_f > 210.0 {
            insights.push(Insight {
                id: Some(format!("diagnostic_insight_{}", counter)),
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
            counter += 1;
        }

        // Oil temperature analysis
        if session.avg_oil_temp_f > 250.0 {
            insights.push(Insight {
                id: Some(format!("diagnostic_insight_{}", counter)),
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
            counter += 1;
        }

        // RPM analysis
        if session.max_rpm > 6000.0 {
            insights.push(Insight {
                id: Some(format!("diagnostic_insight_{}", counter)),
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
            counter += 1;
        }

        // Error code analysis
        if !session.error_codes.is_empty() {
            for error in &session.error_codes {
                insights.push(Insight {
                    id: Some(format!("diagnostic_insight_{}", counter)),
                    category: InsightCategory::Critical,
                    title: format!("Diagnostic Trouble Code: {}", error.code),
                    description: error.description.clone(),
                    supporting_data: [
                        ("code".to_string(), serde_json::json!(error.code)),
                        ("severity".to_string(), serde_json::json!(format!("{:?}", error.severity))),
                    ].iter().cloned().collect(),
                    related_insight_ids: vec![],
                });
                counter += 1;
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
            id: Some(format!("diagnostic_insight_{}", counter)),
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
                estimated_impact: None,
                category: "maintenance".to_string(),
            });
        } else if session.max_coolant_temp_f > 210.0 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: "Check coolant level and inspect for leaks".to_string(),
                expected_benefit: "Maintain optimal engine temperature and prevent overheating".to_string(),
                effort_level: EffortLevel::Easy,
                related_insight_id: None,
                estimated_impact: None,
                category: "maintenance".to_string(),
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
                estimated_impact: None,
                category: "maintenance".to_string(),
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
                estimated_impact: None,
                category: "maintenance".to_string(),
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
                estimated_impact: None,
                category: "maintenance".to_string(),
            });
        }

        // Preventive maintenance
        recommendations.push(Recommendation {
            priority: Priority::Low,
            action: "Schedule routine maintenance inspection within 1000 miles".to_string(),
            expected_benefit: "Catch potential issues early and maintain reliability".to_string(),
            effort_level: EffortLevel::Easy,
            related_insight_id: None,
            estimated_impact: None,
            category: "maintenance".to_string(),
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
        let start = std::time::Instant::now();

        // Try AI analysis first, fall back to rule-based if unavailable
        let (score, insights, recommendations, model_used, provider) =
            match self.try_ai_analysis(request).await {
                Ok((ai_score, ai_insights, ai_recs)) => {
                    (ai_score, ai_insights, ai_recs, self.ollama.model_name(), "ollama")
                }
                Err(e) => {
                    // Log the error and fall back to rule-based
                    eprintln!("Ollama unavailable ({}), using rule-based analysis", e);
                    let score = self.calculate_health_score(request);
                    let insights = self.generate_insights(request);
                    let recs = self.generate_recommendations(request);
                    (score, insights, recs, "rule-based-v1".to_string(), "local-rules")
                }
            };

        let elapsed = start.elapsed().as_millis() as u64;

        Ok(AnalysisResult {
            agent_id: "diagnostic".to_string(),
            timestamp: Utc::now(),
            score: Some(score),
            confidence: if provider == "ollama" { 0.9 } else { 0.8 },
            insights,
            recommendations,
            execution_time_ms: elapsed,
            model_used,
            metadata: ResultMetadata {
                from_cache: false,
                cache_key: None,
                tokens_consumed: None,
                provider: provider.to_string(),
                execution_location: ExecutionLocation::Local,
            },
        })
    }

    fn metadata(&self) -> AgentMetadata {
        AgentMetadata {
            id: "diagnostic".to_string(),
            name: "AI Diagnostic Technician".to_string(),
            domain: AgentDomain::Diagnostics,
            description: "AI-powered analysis of vehicle sensor data (temperature, RPM, error codes) to detect mechanical issues and predict maintenance needs. Powered by local Llama LLM.".to_string(),
            model_preference: ModelPreference::Medium,
            privacy_level: PrivacyLevel::LocalOnly,
            icon: "🔧".to_string(),
            version: "2.0.0".to_string(),
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
