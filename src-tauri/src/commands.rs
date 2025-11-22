use tauri::State;
use std::sync::Arc;
use crate::simulation::state::SimulationState;
use serde_json::json;

#[tauri::command]
pub fn start_simulation(state: State<'_, Arc<SimulationState>>) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = true;
    println!("Simulation started");
}

#[tauri::command]
pub fn stop_simulation(state: State<'_, Arc<SimulationState>>) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = false;
    println!("Simulation stopped");
}

#[tauri::command]
pub fn start_sniffing(state: State<'_, Arc<SimulationState>>) {
    let mut is_sniffing = state.is_sniffing.lock().unwrap();
    *is_sniffing = true;
    println!("CAN Sniffing started");
}

#[tauri::command]
pub fn stop_sniffing(state: State<'_, Arc<SimulationState>>) {
    let mut is_sniffing = state.is_sniffing.lock().unwrap();
    *is_sniffing = false;
    println!("CAN Sniffing stopped");
}

#[tauri::command]
pub fn get_simulation_status(state: State<'_, Arc<SimulationState>>) -> bool {
    *state.is_running.lock().unwrap()
}

#[tauri::command]
pub async fn analyze_current_session(state: State<'_, Arc<SimulationState>>, api_key: Option<String>) -> Result<String, String> {
    let stats = {
        let stats_guard = state.stats.lock().unwrap();
        // Clone the data we need so we don't hold the lock during the async request
        (
            stats_guard.sample_count,
            stats_guard.total_rpm,
            stats_guard.max_speed,
            stats_guard.high_rpm_events
        )
    };

    let (sample_count, total_rpm, max_speed, high_rpm_events) = stats;

    if sample_count == 0 {
        return Ok("No driving data available yet. Start the engine!".to_string());
    }

    let avg_rpm = total_rpm / sample_count as f64;
    let duration_sec = sample_count as f64 * 0.1; // 10Hz

    // Basic Report (Always generated)
    let mut report = String::new();
    report.push_str(&format!("## Driving Session Analysis\n\n"));
    report.push_str(&format!("- **Duration**: {:.1} seconds\n", duration_sec));
    report.push_str(&format!("- **Max Speed**: {:.1} km/h\n", max_speed));
    report.push_str(&format!("- **Average RPM**: {:.0} rpm\n", avg_rpm));
    report.push_str(&format!("- **High RPM Events**: {}\n\n", high_rpm_events));

    // If API Key is provided, call OpenAI
    if let Some(key) = api_key {
        if !key.trim().is_empty() {
            report.push_str("### 🤖 AI Analysis (OpenAI)\n\n");
            
            let prompt = format!(
                "Analyze this driving session data and give specific advice on fuel efficiency and safety. 
                Data: Duration: {:.1}s, Max Speed: {:.1} km/h, Avg RPM: {:.0}, High RPM Events (>5000): {}.
                Keep it concise and use bullet points.",
                duration_sec, max_speed, avg_rpm, high_rpm_events
            );

            let client = reqwest::Client::new();
            let res = client.post("https://api.openai.com/v1/chat/completions")
                .header("Authorization", format!("Bearer {}", key))
                .header("Content-Type", "application/json")
                .json(&json!({
                    "model": "gpt-3.5-turbo",
                    "messages": [
                        {"role": "system", "content": "You are an expert automotive driving instructor."},
                        {"role": "user", "content": prompt}
                    ],
                    "max_tokens": 150
                }))
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?;

            if res.status().is_success() {
                let body: serde_json::Value = res.json().await.map_err(|e| format!("Parse failed: {}", e))?;
                if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
                    report.push_str(content);
                    return Ok(report);
                }
            } else {
                report.push_str(&format!("⚠️ **AI Error**: Failed to call OpenAI API. Status: {}", res.status()));
            }
        }
    }

    // Fallback to local logic if no key or API failed
    report.push_str("### 💡 Local Insights (Mock)\n");
    
    if max_speed > 120.0 {
        report.push_str("⚠️ **Speed Warning**: You exceeded 120 km/h. Please drive safely and adhere to speed limits.\n");
    } else {
        report.push_str("✅ **Speed**: Your speed was within reasonable limits.\n");
    }

    if avg_rpm > 4000.0 || high_rpm_events > 10 {
        report.push_str("⚠️ **Efficiency**: High RPM detected frequently. Consider shifting gears earlier to save fuel.\n");
    } else {
        report.push_str("✅ **Efficiency**: Good gear management. You are driving efficiently.\n");
    }

    Ok(report)
}
