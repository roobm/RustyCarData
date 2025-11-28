use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

/// Supabase client for interacting with cloud database
#[derive(Clone)]
pub struct SupabaseClient {
    client: Postgrest,
    anon_key: String,
}

/// Session record in database (matches 001_initial_schema.sql)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub started_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_source: Option<String>, // 'playback', 'random_walk', 'imported', 'live'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_km: Option<f64>,
}

/// Telemetry point for bulk insert (matches 001_initial_schema.sql)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryRow {
    pub session_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    // Core parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_rpm: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coolant_temp: Option<f64>,
    // Extended parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_position: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_load: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_level: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intake_air_temp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_advance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_voltage: Option<f64>,
    // Additional sensors (future)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maf_flow_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o2_sensor_voltage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel_pressure: Option<f64>,
}

/// Session statistics (matches 001_initial_schema.sql)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatsRow {
    pub session_id: Uuid,
    pub avg_speed: Option<f64>,
    pub max_speed: Option<f64>,
    pub avg_rpm: Option<f64>,
    pub max_rpm: Option<f64>,
    pub avg_coolant_temp: Option<f64>,
    pub max_coolant_temp: Option<f64>,
    pub avg_throttle_position: Option<f64>,
    pub avg_engine_load: Option<f64>,
    pub fuel_consumed_liters: Option<f64>,
    pub total_points: i32,
    pub missing_values_count: i32,
    pub updated_at: Option<String>,
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
            .select("*")  // Must explicitly select to get response
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
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());
            return Err(format!("Failed to create session: {} - {}", status, body).into());
        }

        Err("Failed to create session: No ID returned".into())
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
        ended_at: String,
        duration_seconds: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let update = serde_json::json!({
            "ended_at": ended_at,
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

    /// Update session name
    pub async fn update_session_name(
        &self,
        session_id: Uuid,
        name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let update = serde_json::json!({
            "session_name": name
        });

        let resp = self
            .client
            .from("sessions")
            .eq("id", &session_id.to_string())
            .update(update.to_string())
            .execute()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());
            Err(format!("Failed to update session name: {} - {}", status, body).into())
        }
    }

    /// Delete session and all related data (telemetry points and stats)
    pub async fn delete_session(
        &self,
        session_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Delete telemetry points first (foreign key constraint)
        let resp = self
            .client
            .from("telemetry_points")
            .eq("session_id", &session_id.to_string())
            .delete()
            .execute()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());
            return Err(format!("Failed to delete telemetry points: {} - {}", status, body).into());
        }

        // Delete session stats (if exists)
        let resp = self
            .client
            .from("session_stats")
            .eq("session_id", &session_id.to_string())
            .delete()
            .execute()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());
            return Err(format!("Failed to delete session stats: {} - {}", status, body).into());
        }

        // Finally delete the session itself
        let resp = self
            .client
            .from("sessions")
            .eq("id", &session_id.to_string())
            .delete()
            .execute()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "Unable to read response body".to_string());
            Err(format!("Failed to delete session: {} - {}", status, body).into())
        }
    }

    /// Get session statistics
    pub async fn get_session_stats(
        &self,
        session_id: Uuid,
    ) -> Result<Option<SessionStatsRow>, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .from("session_stats")
            .eq("session_id", &session_id.to_string())
            .select("*")
            .execute()
            .await?;

        if resp.status().is_success() {
            let body = resp.text().await?;
            let rows: Vec<SessionStatsRow> = serde_json::from_str(&body)?;
            Ok(rows.into_iter().next())
        } else {
            Err(format!("Failed to get session stats: {}", resp.status()).into())
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
