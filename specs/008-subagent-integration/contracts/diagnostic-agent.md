# Agent Contract: Diagnostic Technician

**Agent ID**: `diagnostic-technician`
**Domain**: Diagnostics
**Model Preference**: Powerful (Opus / Claude Sonnet 4)
**Privacy Level**: CloudPreferred
**Priority**: P1 (Core Agent)

---

## Purpose

Perform advanced diagnostic analysis by correlating sensor data, error codes, and operating patterns to predict mechanical failures before they occur. Act as a "Master Technician" identifying anomalies that suggest wear, degradation, or impending component failure.

---

## Input Schema

### Required Fields

```rust
struct DiagnosticInput {
    // Session identification
    session_id: String,
    duration_seconds: u64,
    distance_miles: f32,

    // Engine metrics (time-series)
    coolant_temp_readings: Vec<SensorReading>,
    oil_temp_readings: Vec<SensorReading>,
    intake_air_temp_readings: Vec<SensorReading>,

    // RPM correlation
    rpm_readings: Vec<SensorReading>,

    // Electrical system
    voltage_readings: Vec<SensorReading>,
    battery_voltage_min: f32,
    battery_voltage_max: f32,
    alternator_output_avg: f32,

    // Fuel system
    fuel_pressure_readings: Vec<SensorReading>,
    fuel_trim_short: Vec<SensorReading>,  // Short-term fuel trim %
    fuel_trim_long: Vec<SensorReading>,   // Long-term fuel trim %

    // Engine load
    engine_load_readings: Vec<SensorReading>,
    throttle_position_readings: Vec<SensorReading>,

    // Oxygen sensors (emissions)
    o2_sensor_bank1: Vec<SensorReading>,
    o2_sensor_bank2: Option<Vec<SensorReading>>,

    // Error codes
    error_codes: Vec<ErrorCode>,

    // Vehicle context
    vehicle_make: String,
    vehicle_model: String,
    vehicle_year: u16,
    mileage: u32,
    last_service_miles: Option<u32>,
}

struct SensorReading {
    timestamp: String,
    value: f32,
}

struct ErrorCode {
    code: String,           // e.g., "P0420"
    description: String,
    severity: String,       // "Informational", "Warning", "Critical"
    timestamp: String,
}
```

### Example Input (JSON)

```json
{
  "session_id": "sess-2025-11-25-003",
  "duration_seconds": 1800,
  "distance_miles": 22.1,
  "coolant_temp_readings": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 160},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 185},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 195},
    {"timestamp": "2025-11-25T10:15:00Z", "value": 205},
    {"timestamp": "2025-11-25T10:20:00Z", "value": 218},
    {"timestamp": "2025-11-25T10:25:00Z", "value": 215}
  ],
  "oil_temp_readings": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 170},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 195},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 210},
    {"timestamp": "2025-11-25T10:15:00Z", "value": 220},
    {"timestamp": "2025-11-25T10:20:00Z", "value": 228}
  ],
  "intake_air_temp_readings": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 75},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 82},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 88}
  ],
  "rpm_readings": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 1000},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 2500},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 2800},
    {"timestamp": "2025-11-25T10:15:00Z", "value": 3200},
    {"timestamp": "2025-11-25T10:20:00Z", "value": 3100}
  ],
  "voltage_readings": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 12.2},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 13.8},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 13.6},
    {"timestamp": "2025-11-25T10:15:00Z", "value": 13.5},
    {"timestamp": "2025-11-25T10:20:00Z", "value": 13.3}
  ],
  "battery_voltage_min": 12.2,
  "battery_voltage_max": 13.8,
  "alternator_output_avg": 13.5,
  "fuel_pressure_readings": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 58},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 55},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 53},
    {"timestamp": "2025-11-25T10:15:00Z", "value": 52}
  ],
  "fuel_trim_short": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 2.5},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 3.1},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 4.2}
  ],
  "fuel_trim_long": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 8.5},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 8.7},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 9.2}
  ],
  "engine_load_readings": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 25},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 55},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 62}
  ],
  "throttle_position_readings": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 18},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 45},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 52}
  ],
  "o2_sensor_bank1": [
    {"timestamp": "2025-11-25T10:00:00Z", "value": 0.45},
    {"timestamp": "2025-11-25T10:05:00Z", "value": 0.42},
    {"timestamp": "2025-11-25T10:10:00Z", "value": 0.38}
  ],
  "o2_sensor_bank2": null,
  "error_codes": [
    {
      "code": "P0171",
      "description": "System Too Lean (Bank 1)",
      "severity": "Warning",
      "timestamp": "2025-11-25T10:12:00Z"
    }
  ],
  "vehicle_make": "Honda",
  "vehicle_model": "Accord",
  "vehicle_year": 2015,
  "mileage": 87500,
  "last_service_miles": 85000
}
```

---

## System Prompt

```markdown
You are a Master Automotive Technician with 20+ years of diagnostic experience.

Your expertise is in correlating sensor data to predict mechanical failures before they occur.

Analyze the sensor data and provide:
1. **Vehicle health score** (0-100, where 100 = perfect health, no issues detected)
2. **Anomalies detected** with failure risk assessment (Low/Medium/High/Critical)
3. **Predicted maintenance needs** with urgency timeline (Immediate, < 500 miles, < 2000 miles, Routine)
4. **Correlation analysis** explaining how multiple sensors suggest a specific failure mode

Guidelines:
- Be **specific about failure modes** (e.g., "thermostat stuck closed" not "cooling issue")
- Cite **sensor correlations** (e.g., "Coolant temp rising 15% faster than RPM suggests...")
- Reference **common patterns** (e.g., "This pattern typically indicates worn alternator brushes")
- Provide **urgency assessment** (distinguish between "monitor" and "address immediately")
- Use **technical terminology** but explain it (e.g., "Fuel trim +9% indicates ECU compensating for lean condition")
- Acknowledge **normal variations** (not every reading is a problem)

Do NOT provide driving behavior advice (that's the Safety/Fuel Coach's job).

Output format: Valid JSON matching the schema below.
```

---

## Output Schema

```rust
struct DiagnosticOutput {
    score: u8,                          // 0-100 (vehicle health)
    confidence: f32,                    // 0.0-1.0
    insights: Vec<Insight>,
    recommendations: Vec<Recommendation>,
}

struct Insight {
    category: String,                   // "Info", "Warning", "Critical", "Positive"
    title: String,
    description: String,
    supporting_data: HashMap<String, Value>,
    related_sensors: Vec<String>,       // Which sensors contributed to this insight
}

struct Recommendation {
    priority: String,                   // "Low", "Medium", "High"
    action: String,
    expected_benefit: String,
    effort_level: String,               // "Easy", "Moderate", "Significant"
    urgency: String,                    // "Immediate", "< 500 miles", "< 2000 miles", "Routine"
}
```

### Example Output (JSON)

```json
{
  "score": 72,
  "confidence": 0.88,
  "insights": [
    {
      "category": "Warning",
      "title": "Coolant temperature rising faster than expected",
      "description": "Coolant temperature increased 36% (160°F → 218°F) while RPM increased only 220% (1000 → 3200). Normal correlation ratio is 1:5 (temp:RPM). This suggests restricted coolant flow, possibly from a thermostat stuck partially closed or a failing water pump.",
      "supporting_data": {
        "coolant_temp_start": 160,
        "coolant_temp_peak": 218,
        "coolant_temp_increase_pct": 36.25,
        "rpm_start": 1000,
        "rpm_peak": 3200,
        "rpm_increase_pct": 220,
        "expected_temp_f": 195,
        "actual_temp_f": 218,
        "excess_temp_f": 23
      },
      "related_sensors": ["coolant_temp", "rpm"]
    },
    {
      "category": "Warning",
      "title": "Fuel system running lean (P0171 code)",
      "description": "ECU detected lean condition (P0171). Long-term fuel trim at +9.2% indicates ECU is adding 9% more fuel to compensate. Short-term trim also elevated (+4.2%). Possible causes: vacuum leak, MAF sensor dirty, fuel pressure dropping (52 PSI vs 58 PSI at start suggests weak fuel pump).",
      "supporting_data": {
        "error_code": "P0171",
        "fuel_trim_long": 9.2,
        "fuel_trim_short": 4.2,
        "fuel_pressure_start": 58,
        "fuel_pressure_end": 52,
        "fuel_pressure_drop": 6,
        "normal_fuel_trim_range": "-5% to +5%"
      },
      "related_sensors": ["fuel_trim_long", "fuel_trim_short", "fuel_pressure", "error_codes"]
    },
    {
      "category": "Info",
      "title": "Alternator output slightly below optimal",
      "description": "Average voltage output of 13.5V is within acceptable range (13.5-14.5V) but trending toward the lower end. Voltage dropped from 13.8V to 13.3V during the session. Monitor for further decline, which could indicate worn alternator brushes or diode failure.",
      "supporting_data": {
        "voltage_start": 13.8,
        "voltage_end": 13.3,
        "voltage_avg": 13.5,
        "voltage_drop": 0.5,
        "optimal_range": "13.8V - 14.5V",
        "minimum_acceptable": 13.2
      },
      "related_sensors": ["voltage", "alternator_output"]
    },
    {
      "category": "Positive",
      "title": "O2 sensor responding normally",
      "description": "Oxygen sensor (Bank 1) readings are within normal range (0.1-0.9V) and fluctuating appropriately, indicating healthy sensor and catalytic converter operation.",
      "supporting_data": {
        "o2_avg": 0.42,
        "o2_range": "0.38V - 0.45V",
        "normal_range": "0.1V - 0.9V"
      },
      "related_sensors": ["o2_sensor_bank1"]
    }
  ],
  "recommendations": [
    {
      "priority": "High",
      "action": "Inspect cooling system: Check thermostat operation, water pump condition, and coolant flow. Pressure test system for leaks.",
      "expected_benefit": "Prevents overheating and potential head gasket failure ($2000+ repair). Early thermostat replacement ($150) vs engine damage ($3000+).",
      "effort_level": "Moderate",
      "urgency": "< 500 miles"
    },
    {
      "priority": "High",
      "action": "Diagnose lean condition: Test fuel pressure with gauge (should be 55-60 PSI), inspect for vacuum leaks, clean MAF sensor.",
      "expected_benefit": "Restores proper fuel mixture, improves fuel economy by 5-10%, prevents catalytic converter damage from lean burn.",
      "effort_level": "Moderate",
      "urgency": "< 500 miles"
    },
    {
      "priority": "Medium",
      "action": "Monitor alternator voltage. If it drops below 13.2V, have alternator tested (most auto parts stores test for free).",
      "expected_benefit": "Prevents unexpected battery drain and potential no-start condition. Early alternator replacement ($300) vs roadside breakdown ($150 tow + alternator).",
      "effort_level": "Easy",
      "urgency": "< 2000 miles"
    }
  ]
}
```

---

## Scoring Rubric

The agent calculates a health score (0-100) based on:

| Factor | Weight | Optimal | Penalty |
|--------|--------|---------|---------|
| **Error codes** | 30% | Zero active codes | -20 per Critical, -10 per Warning, -3 per Info |
| **Temperature anomalies** | 25% | Temp correlates with load | -15 if coolant >210°F, -20 if >220°F |
| **Fuel system health** | 20% | Fuel trim ±5%, pressure stable | -10 per 5% fuel trim excess, -5 per 5 PSI drop |
| **Electrical system** | 15% | Voltage 13.8-14.5V | -8 if < 13.5V, -15 if < 13.2V |
| **Sensor correlations** | 10% | All sensors respond logically | -5 per anomaly (e.g., temp rise without load) |

**Example Calculation:**
- Base score: 100
- Error codes: -10 points (1 Warning code P0171)
- Temperature: -12 points (218°F coolant, exceeds 210°F threshold)
- Fuel system: -6 points (fuel trim +9.2%, 4.2% over limit)
- Electrical: -5 points (voltage 13.5V, slightly low)
- Sensor correlations: -5 points (coolant/RPM ratio anomaly)
- **Final Score: 72** (Fair health, maintenance needed soon)

---

## Confidence Calculation

```rust
let mut confidence = 1.0;

// Reduce confidence for sparse sensor data
let sensor_count = count_non_null_sensors(&input);
if sensor_count < 8 {
    confidence *= 0.75; // Incomplete picture
}

// Increase confidence when error codes present (concrete evidence)
if !error_codes.is_empty() {
    confidence *= 1.1;
    confidence = confidence.min(1.0);
}

// Reduce confidence for very short sessions (patterns unclear)
if duration_seconds < 600 {
    confidence *= 0.80; // < 10 minutes
}

// Increase confidence for strong sensor correlations
if strong_correlation_detected {
    confidence *= 1.05;
    confidence = confidence.min(1.0);
}
```

---

## Edge Cases

### Case 1: No Error Codes, All Sensors Normal
- Input: All readings within normal ranges
- Behavior: Score 95-100, provide maintenance reminders based on mileage
- Output: "No issues detected. Next service due at..."

### Case 2: Cold Start Anomalies
- Input: Coolant temp 60°F at start, rises rapidly to 195°F
- Behavior: Recognize this as normal cold start warmup
- Output: No warning about temp rise (expected pattern)

### Case 3: Sparse Sensor Data (Older Vehicle)
- Input: Only 4-5 sensors available (no fuel trim, no intake temp, etc.)
- Behavior: Lower confidence, focus on available data
- Output: Disclaimer about limited sensor coverage

### Case 4: Multiple Correlated Issues
- Input: P0420 (catalyst efficiency), high fuel trim, O2 sensor slow response
- Behavior: Diagnose root cause (likely failing catalytic converter)
- Output: Detailed correlation analysis linking all three symptoms

### Case 5: Sensor Reading Out of Range (Malfunction)
- Input: Coolant temp reads 500°F (physically impossible)
- Behavior: Detect sensor malfunction, don't use for diagnostics
- Output: "Coolant temp sensor malfunction detected (implausible reading)"

---

## Common Diagnostic Patterns

### Pattern 1: Thermostat Failure
**Symptoms:**
- Coolant temp rises too fast relative to RPM
- Temp exceeds 210°F under normal load
- Temp fluctuates wildly (stuck open scenario)

**Diagnosis:** Thermostat stuck closed (overheating) or stuck open (slow warmup)

### Pattern 2: Weak Fuel Pump
**Symptoms:**
- Fuel pressure drops over time (e.g., 58 PSI → 52 PSI)
- Positive fuel trim (ECU compensating)
- Engine hesitation under load (not visible in telemetry but inferred)

**Diagnosis:** Fuel pump wearing out, unable to maintain pressure

### Pattern 3: Alternator Bearing Wear
**Symptoms:**
- Voltage slowly declining session-to-session
- Voltage drop under electrical load
- Voltage < 13.5V average

**Diagnosis:** Alternator brushes worn or bearings failing

### Pattern 4: Vacuum Leak
**Symptoms:**
- P0171 (lean) or P0174 (lean bank 2)
- High positive fuel trim (+10%+)
- Rough idle (inferred from RPM fluctuation)

**Diagnosis:** Vacuum leak (intake manifold gasket, brake booster hose, etc.)

### Pattern 5: Failing Catalytic Converter
**Symptoms:**
- P0420 (catalyst efficiency below threshold)
- O2 sensor slow response (upstream/downstream similar readings)
- Reduced fuel economy

**Diagnosis:** Catalyst substrate degraded, not converting emissions

---

## Testing

### Test Case 1: Healthy Vehicle

**Input:**
```json
{
  "coolant_temp_readings": [/* normal warmup 160→195 */],
  "voltage_readings": [/* stable 14.2V */],
  "fuel_trim_short": [/* ±2% */],
  "fuel_trim_long": [/* ±3% */],
  "error_codes": []
}
```

**Expected Output:**
- Score: 95-100
- Insights: All "Info" or "Positive"
- Recommendations: Routine maintenance only

### Test Case 2: Overheating Risk

**Input:**
```json
{
  "coolant_temp_readings": [/* 160→230 in 10 minutes */],
  "rpm_readings": [/* 1500→2500 (modest increase) */],
  "error_codes": [{"code": "P0128", "description": "Coolant Thermostat"}]
}
```

**Expected Output:**
- Score: 50-60
- Insights: "Critical" - overheating imminent
- Recommendations: "Immediate" - stop driving, inspect coolant system

### Test Case 3: Multiple Correlated Issues

**Input:**
```json
{
  "fuel_pressure_readings": [/* dropping 60→48 PSI */],
  "fuel_trim_long": [/* +12% */],
  "error_codes": [{"code": "P0171"}],
  "o2_sensor_bank1": [/* slow response */]
}
```

**Expected Output:**
- Score: 60-70
- Insights: Link fuel pressure drop → lean condition → O2 sensor compensation
- Recommendations: Replace fuel pump, test fuel filter

---

## Model-Specific Considerations

### For Opus (Powerful Model)
- **Complex reasoning required**: Opus excels at multi-sensor correlation
- **Temperature: 0.4** (balanced - need creativity for rare failure modes)
- **Provide examples** of common diagnostic patterns in system prompt
- **Context window**: Use full 4000 tokens (include detailed sensor histories)

### For Claude Sonnet 4
- Similar to Opus, slightly faster, good cost/performance balance
- **Emphasize pattern recognition** ("This pattern typically indicates...")
- **Provide failure mode database** (pre-loaded common issues for each make/model)

---

## Integration Points

### Dependencies
- **Session Summary** (from feature 006-data-recording): Sensor time-series data
- **AI Provider Layer** (from feature 005-real-ai): OpenAI/Anthropic client

### Consumers
- **Agent Dashboard** (frontend): Displays diagnostic insights
- **Agent Orchestrator** (backend): Dispatches requests

### Related Agents
- **Fuel Efficiency Coach**: May defer lean fuel trim diagnosis to Diagnostic Agent
- **Driver Profile Learner**: May aggregate diagnostic patterns over time

---

## Prompt Engineering Notes

### Token Budget
- System prompt: ~400 tokens
- Session data (JSON with time-series): ~1200 tokens (largest of all agents)
- Diagnostic pattern examples: ~300 tokens
- Few-shot example (optional): ~400 tokens
- Output buffer: ~800 tokens
- **Total**: ~3100 tokens (approaching 4000 limit, but necessary for complexity)

### Prompt Improvements (Iterative)
1. **V1**: Basic sensor analysis → Too generic, missed correlations
2. **V2**: Added correlation prompts → Better multi-sensor analysis
3. **V3**: Added common failure patterns → More specific diagnoses
4. **V4**: Added urgency assessment → Actionable maintenance timeline
5. **V5 (current)**: Added sensor malfunction detection → Handles bad data gracefully

---

## Success Criteria

- ✅ Diagnostic accuracy: 80%+ confirmed by mechanic (in pilot testing)
- ✅ False positives: < 15% (avoid crying wolf)
- ✅ Correlation detection: Identify multi-sensor anomalies in 90%+ of cases
- ✅ Score consistency: ±8 points for same input (more variance allowed due to complexity)
- ✅ Latency: < 5 seconds per analysis (Opus model, more complex than other agents)
- ✅ Cost: < $0.05 per analysis (higher due to Opus usage)

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-11-25 | Initial contract definition |
