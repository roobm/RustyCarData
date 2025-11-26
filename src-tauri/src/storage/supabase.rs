use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

/// Supabase client for interacting with cloud database
pub struct SupabaseClient {
    client: Postgrest,
    anon_key: String,
}

/// Session record in database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRow {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub session_name: String,
    pub vehicle_info: serde_json::Value,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_seconds: Option<i32>,
    pub distance_miles: Option<f64>,
    pub dataset_source: String,
}

/// Telemetry point for bulk insert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryRow {
    pub session_id: Uuid,
    pub timestamp: i64,
    pub pid: String,
    pub value: f64,
    pub unit: String,
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatsRow {
    pub session_id: Uuid,
    pub max_rpm: Option<f64>,
    pub avg_rpm: Option<f64>,
    pub max_speed: Option<f64>,
    pub avg_speed: Option<f64>,
    pub fuel_consumed: Option<f64>,
    pub avg_fuel_efficiency: Option<f64>,
    pub hard_braking_events: Option<i32>,
    pub rapid_accel_events: Option<i32>,
    pub error_codes: Option<serde_json::Value>,
}

impl SupabaseClient {
    /// Create new Supabase client from environment variables
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let url = env::var("SUPABASE_URL")
            .map_err(|_| "SUPABASE_URL not set in environment")?;
        let anon_key = env::var("SUPABASE_ANON_KEY")
            .map_err(|_| "SUPABASE_ANON_KEY not set in environment")?;

        let client = Postgrest::new(format!("{}/rest/v1", url))
            .insert_header("apikey", &anon_key);

        Ok(Self { client, anon_key })
    }

    /// Create a new session
    pub async fn create_session(&self, session: &SessionRow) -> Result<Uuid, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .from("sessions")
            .insert(serde_json::to_string(session)?)
            .execute()
            .await?;

        if resp.status().is_success() {
            let body = resp.text().await?;
            let rows: Vec<SessionRow> = serde_json::from_str(&body)?;

            if let Some(row) = rows.first() {
                if let Some(id) = row.id {
                    return Ok(id);
                }
            }
        }

        Err("Failed to create session".into())
    }

    /// Insert telemetry points in batch
    pub async fn insert_telemetry_batch(
        &self,
        points: &[TelemetryRow],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if points.is_empty() {
            return Ok(());
        }

        let resp = self
            .client
            .from("telemetry_points")
            .insert(serde_json::to_string(points)?)
            .execute()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to insert telemetry: {}", resp.status()).into())
        }
    }

    /// Update or insert session statistics
    pub async fn upsert_session_stats(
        &self,
        stats: &SessionStatsRow,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let resp = self
            .client
            .from("session_stats")
            .upsert(serde_json::to_string(stats)?)
            .execute()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to upsert stats: {}", resp.status()).into())
        }
    }

    /// Get a single session by ID
    pub async fn get_session(&self, id: Uuid) -> Result<Option<SessionRow>, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .from("sessions")
            .eq("id", &id.to_string())
            .select("*")
            .execute()
            .await?;

        if resp.status().is_success() {
            let body = resp.text().await?;
            let rows: Vec<SessionRow> = serde_json::from_str(&body)?;
            Ok(rows.into_iter().next())
        } else {
            Err(format!("Failed to get session: {}", resp.status()).into())
        }
    }

    /// List recent sessions (limit 50)
    pub async fn list_sessions(&self, limit: usize) -> Result<Vec<SessionRow>, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .from("sessions")
            .select("*")
            .order("created_at.desc")
            .limit(limit)
            .execute()
            .await?;

        if resp.status().is_success() {
            let body = resp.text().await?;
            let rows: Vec<SessionRow> = serde_json::from_str(&body)?;
            Ok(rows)
        } else {
            Err(format!("Failed to list sessions: {}", resp.status()).into())
        }
    }

    /// Update session end time and duration
    pub async fn complete_session(
        &self,
        id: Uuid,
        end_time: String,
        duration_seconds: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let update = serde_json::json!({
            "end_time": end_time,
            "duration_seconds": duration_seconds
        });

        let resp = self
            .client
            .from("sessions")
            .eq("id", &id.to_string())
            .update(update.to_string())
            .execute()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("Failed to update session: {}", resp.status()).into())
        }
    }

    /// Get telemetry points for a session
    pub async fn get_session_telemetry(
        &self,
        session_id: Uuid,
        limit: Option<usize>,
    ) -> Result<Vec<TelemetryRow>, Box<dyn std::error::Error>> {
        let mut query = self
            .client
            .from("telemetry_points")
            .eq("session_id", &session_id.to_string())
            .select("*")
            .order("timestamp.asc");

        if let Some(lim) = limit {
            query = query.limit(lim);
        }

        let resp = query.execute().await?;

        if resp.status().is_success() {
            let body = resp.text().await?;
            let rows: Vec<TelemetryRow> = serde_json::from_str(&body)?;
            Ok(rows)
        } else {
            Err(format!("Failed to get telemetry: {}", resp.status()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation_without_env() {
        // Should fail if env vars not set
        let result = SupabaseClient::new();
        assert!(result.is_err());
    }
}
