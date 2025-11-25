use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Agent Metadata & Configuration

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub id: String,
    pub name: String,
    pub domain: AgentDomain,
    pub description: String,
    pub model_preference: ModelPreference,
    pub privacy_level: PrivacyLevel,
    pub icon: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentDomain {
    FuelEfficiency,
    Safety,
    Diagnostics,
    EcoRoute,
    Performance,
    DriverProfile,
    CodeArchitecture,
    CodeReview,
    TestGeneration,
    Documentation,
    PerformanceOptimization,
    DependencyManagement,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelPreference {
    Lightweight,
    Medium,
    Powerful,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrivacyLevel {
    LocalOnly,
    CloudPreferred,
    CloudRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq)]
pub struct AgentConfig {
    pub enabled: bool,
    pub verbosity: VerbosityLevel,
    pub thresholds: HashMap<String, f32>,
    pub language: String,
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            verbosity: VerbosityLevel::Detailed,
            thresholds: HashMap::new(),
            language: "en".to_string(),
            custom: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq)]
pub enum VerbosityLevel {
    Concise,
    Detailed,
    Expert,
}

// Analysis Request & Result

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub request_id: String,
    pub session_data: SessionSummary,
    pub vehicle_info: VehicleInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_preferences: Option<UserPreferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_context: Option<Vec<PreviousSession>>,
    #[serde(default)]
    pub force_local: bool,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

fn default_timeout() -> u64 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub agent_id: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    pub confidence: f32,
    pub insights: Vec<Insight>,
    pub recommendations: Vec<Recommendation>,
    pub execution_time_ms: u64,
    pub model_used: String,
    pub metadata: ResultMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub category: InsightCategory,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub supporting_data: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub related_insight_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InsightCategory {
    Info,
    Warning,
    Critical,
    Positive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub priority: Priority,
    pub action: String,
    pub expected_benefit: String,
    pub effort_level: EffortLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_insight_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EffortLevel {
    Easy,
    Moderate,
    Significant,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResultMetadata {
    #[serde(default)]
    pub from_cache: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_consumed: Option<TokenUsage>,
    pub provider: String,
    pub execution_location: ExecutionLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ExecutionLocation {
    Local,
    #[default]
    Cloud,
}

// Session Data

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_seconds: u64,
    pub distance_miles: f32,
    pub fuel_consumed_gallons: f32,
    pub avg_fuel_efficiency_mpg: f32,
    pub avg_speed_mph: f32,
    pub max_speed_mph: f32,
    pub speed_violations_count: u32,
    pub avg_rpm: f32,
    pub max_rpm: f32,
    pub high_rpm_duration_seconds: u64,
    pub avg_coolant_temp_f: f32,
    pub max_coolant_temp_f: f32,
    pub avg_oil_temp_f: f32,
    pub hard_braking_events: Vec<Event>,
    pub rapid_acceleration_events: Vec<Event>,
    pub error_codes: Vec<ErrorCode>,
    #[serde(default)]
    pub sensor_readings: HashMap<String, Vec<SensorReading>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub severity: f32,
    pub value: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GpsCoordinate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    HardBraking,
    RapidAcceleration,
    SharpTurn,
    SpeedViolation,
    IdlingExcessive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorCode {
    pub code: String,
    pub description: String,
    pub severity: ErrorSeverity,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorSeverity {
    Informational,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub timestamp: DateTime<Utc>,
    pub sensor_id: String,
    pub value: f32,
    pub unit: String,
}

// Vehicle & User Context

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleInfo {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub engine_type: EngineType,
    pub transmission: TransmissionType,
    pub fuel_type: FuelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineType {
    GasolineNaturallyAspirated,
    GasolineTurbocharged,
    Diesel,
    Hybrid,
    Electric,
    PluginHybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransmissionType {
    Manual,
    Automatic,
    CVT,
    DualClutch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuelType {
    Regular87,
    Midgrade89,
    Premium91Plus,
    Diesel,
    Electric,
    E85,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub goals: Vec<DrivingGoal>,
    pub analysis_style: AnalysisStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrivingGoal {
    FuelEfficiency,
    Safety,
    Performance,
    Comfort,
    Environmental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisStyle {
    Encouraging,
    Neutral,
    Technical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviousSession {
    pub session_id: String,
    pub date: DateTime<Utc>,
    pub summary_metrics: HashMap<String, f32>,
}
