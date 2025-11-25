# Agent Contract: Fuel Efficiency Coach

**Agent ID**: `fuel-coach`
**Domain**: Fuel Efficiency
**Model Preference**: Lightweight (Haiku / Llama 3 8B)
**Privacy Level**: CloudPreferred
**Priority**: P1 (Core Agent)

---

## Purpose

Analyze driving session telemetry to identify fuel-wasting behaviors and provide actionable recommendations for improving fuel efficiency. Focus on driver behavior patterns rather than vehicle maintenance issues.

---

## Input Schema

### Required Fields

```rust
struct FuelCoachInput {
    // Session identification
    session_id: String,
    duration_seconds: u64,

    // Distance and fuel
    distance_miles: f32,
    fuel_consumed_gallons: f32,
    avg_fuel_efficiency_mpg: f32,

    // Engine metrics
    avg_rpm: f32,
    max_rpm: f32,
    high_rpm_duration_seconds: u64, // Time above 3000 RPM

    // Speed metrics
    avg_speed_mph: f32,
    max_speed_mph: f32,
    highway_miles: f32,    // Miles at speed > 55 mph
    city_miles: f32,       // Miles at speed < 45 mph

    // Driving events
    hard_acceleration_count: u32,
    hard_braking_count: u32,
    idling_duration_seconds: u64,

    // Vehicle context
    vehicle_make: String,
    vehicle_model: String,
    vehicle_year: u16,
    engine_type: String,    // "gasoline", "diesel", "hybrid", "electric"
    transmission: String,   // "manual", "automatic", "cvt"
}
```

### Example Input (JSON)

```json
{
  "session_id": "sess-2025-11-25-001",
  "duration_seconds": 1800,
  "distance_miles": 25.3,
  "fuel_consumed_gallons": 0.92,
  "avg_fuel_efficiency_mpg": 27.5,
  "avg_rpm": 3200,
  "max_rpm": 5800,
  "high_rpm_duration_seconds": 420,
  "avg_speed_mph": 42.1,
  "max_speed_mph": 78.5,
  "highway_miles": 18.2,
  "city_miles": 7.1,
  "hard_acceleration_count": 6,
  "hard_braking_count": 4,
  "idling_duration_seconds": 180,
  "vehicle_make": "Honda",
  "vehicle_model": "Civic",
  "vehicle_year": 2018,
  "engine_type": "gasoline",
  "transmission": "manual"
}
```

---

## System Prompt

```markdown
You are an expert Fuel Efficiency Coach specializing in automotive telemetry analysis.

Your goal is to help drivers reduce fuel consumption through actionable driving behavior changes.

Analyze the following driving session data and provide:
1. **Fuel efficiency score** (0-100, where 100 = optimal efficiency for this vehicle type)
2. **3-5 specific fuel-wasting behaviors** observed during the session
3. **Actionable recommendations** with estimated fuel savings percentages
4. **Comparison to optimal driving** for this vehicle type and route mix

Guidelines:
- Be specific and quantitative (cite exact metrics like "RPM", "speed", "duration")
- Be encouraging and constructive (avoid negative language)
- Focus on the **top 3 highest-impact improvements** (80/20 rule)
- Reference industry standards (e.g., EPA fuel economy ratings for this vehicle)
- Provide cost estimates for savings (assume $3.50/gallon gas price)
- Distinguish between highway and city driving patterns

Do NOT provide vehicle maintenance advice (that's the Diagnostic Agent's job).

Output format: Valid JSON matching the schema below.
```

---

## Output Schema

```rust
struct FuelCoachOutput {
    score: u8,                          // 0-100
    confidence: f32,                    // 0.0-1.0
    insights: Vec<Insight>,
    recommendations: Vec<Recommendation>,
}

struct Insight {
    category: String,                   // "Info", "Warning", "Critical", "Positive"
    title: String,
    description: String,
    supporting_data: HashMap<String, Value>,
}

struct Recommendation {
    priority: String,                   // "Low", "Medium", "High"
    action: String,
    expected_benefit: String,           // e.g., "8-12% fuel savings ($150/year)"
    effort_level: String,               // "Easy", "Moderate", "Significant"
}
```

### Example Output (JSON)

```json
{
  "score": 72,
  "confidence": 0.85,
  "insights": [
    {
      "category": "Warning",
      "title": "High RPM cruising detected",
      "description": "Average RPM of 3200 is 20% higher than optimal for highway speeds. This increases fuel consumption significantly, especially during the 18.2 highway miles driven.",
      "supporting_data": {
        "avg_rpm": 3200,
        "optimal_rpm": 2500,
        "excess_percentage": 28,
        "highway_miles": 18.2
      }
    },
    {
      "category": "Warning",
      "title": "Frequent hard acceleration",
      "description": "6 hard acceleration events detected during 25.3 miles. Rapid throttle inputs reduce fuel efficiency by 15-30% during acceleration phases.",
      "supporting_data": {
        "acceleration_events": 6,
        "distance_miles": 25.3,
        "events_per_10_miles": 2.4
      }
    },
    {
      "category": "Info",
      "title": "Excessive idling observed",
      "description": "Vehicle idled for 3 minutes (10% of total session time). Idling consumes 0.2-0.3 gallons per hour with zero miles driven.",
      "supporting_data": {
        "idling_seconds": 180,
        "session_duration_seconds": 1800,
        "idle_percentage": 10,
        "estimated_wasted_gallons": 0.01
      }
    },
    {
      "category": "Positive",
      "title": "Good highway fuel efficiency",
      "description": "Highway portion achieved 32 MPG, which is close to EPA rating for this vehicle. Well done!",
      "supporting_data": {
        "highway_mpg": 32,
        "epa_highway_rating": 35,
        "difference_percentage": -8.6
      }
    }
  ],
  "recommendations": [
    {
      "priority": "High",
      "action": "Shift to higher gear earlier (below 2500 RPM)",
      "expected_benefit": "8-12% fuel savings (~$150/year based on 12,000 mi/year)",
      "effort_level": "Easy"
    },
    {
      "priority": "High",
      "action": "Accelerate gradually (5-second rule: 0-60 mph in 15 seconds)",
      "expected_benefit": "10-15% fuel savings in city driving (~$120/year)",
      "effort_level": "Moderate"
    },
    {
      "priority": "Medium",
      "action": "Turn off engine during stops longer than 30 seconds",
      "expected_benefit": "2-5% overall savings (~$40/year)",
      "effort_level": "Easy"
    }
  ]
}
```

---

## Scoring Rubric

The agent calculates a score (0-100) based on:

| Factor | Weight | Optimal | Penalty |
|--------|--------|---------|---------|
| **Fuel efficiency vs EPA** | 30% | Within 5% of EPA combined rating | -10 points per 5% below EPA |
| **RPM management** | 25% | Avg RPM < 2500 (highway), < 2000 (city) | -5 points per 500 RPM excess |
| **Acceleration smoothness** | 20% | < 1 hard acceleration per 10 miles | -3 points per event |
| **Idling time** | 15% | < 5% of session duration | -2 points per 1% excess |
| **Highway speed** | 10% | 55-65 mph (sweet spot) | -1 point per 5 mph over 65 |

**Example Calculation:**
- Base score: 100
- EPA efficiency: -8 points (27.5 MPG actual vs 30 MPG EPA = -8% → -8 points)
- RPM management: -10 points (3200 RPM avg vs 2500 optimal = +700 → -10 points)
- Hard acceleration: -7 points (6 events ÷ 25.3 miles × 10 = 2.4 per 10 mi → -7 points)
- Idling: -3 points (10% idle vs 5% optimal = +5% → -3 points)
- Highway speed: -0 points (78.5 max is acceptable for occasional passing)
- **Final Score: 72**

---

## Confidence Calculation

Confidence depends on data completeness:

```rust
let mut confidence = 1.0;

// Reduce confidence if data is incomplete
if fuel_consumed_gallons == 0.0 {
    confidence *= 0.5; // Can't validate MPG calculation
}

if high_rpm_duration_seconds == 0 {
    confidence *= 0.9; // Missing granular RPM data
}

if hard_acceleration_count == 0 && hard_braking_count == 0 {
    confidence *= 0.8; // Event detection may be disabled
}

// Reduce confidence for short sessions (< 5 miles)
if distance_miles < 5.0 {
    confidence *= 0.7; // Not enough data for reliable patterns
}
```

---

## Edge Cases

### Case 1: Electric Vehicle
- Input: `engine_type: "electric"`
- Behavior: Skip RPM-based insights, focus on acceleration smoothness and regenerative braking
- Output: "Energy efficiency" instead of "fuel efficiency"

### Case 2: Hybrid Vehicle
- Input: `engine_type: "hybrid"`
- Behavior: Acknowledge electric-assist, focus on ICE usage patterns
- Output: Recommend maximizing electric mode usage

### Case 3: Very Short Trip (< 2 miles)
- Input: `distance_miles: 1.5`
- Behavior: Note that cold engine start penalizes efficiency
- Output: Recommend combining short trips, acknowledge score may not reflect driver behavior

### Case 4: Highway-Only Trip
- Input: `city_miles: 0, highway_miles: 50`
- Behavior: Focus on cruise control usage, speed management
- Output: Different optimal RPM (2500-3000 for highway cruising)

### Case 5: Missing Data Fields
- Input: Some optional fields are null/zero
- Behavior: Skip insights that depend on missing data
- Output: Lower confidence score, note data limitations

---

## Testing

### Test Case 1: Optimal Driving

**Input:**
```json
{
  "avg_fuel_efficiency_mpg": 32.0,
  "avg_rpm": 2400,
  "hard_acceleration_count": 0,
  "idling_duration_seconds": 30,
  "max_speed_mph": 65
}
```

**Expected Output:**
- Score: 95-100
- Insights: Mostly "Positive" category
- Recommendations: Minimal (maintain current habits)

### Test Case 2: Aggressive Driving

**Input:**
```json
{
  "avg_fuel_efficiency_mpg": 18.5,
  "avg_rpm": 4200,
  "hard_acceleration_count": 15,
  "idling_duration_seconds": 300,
  "max_speed_mph": 95
}
```

**Expected Output:**
- Score: 30-40
- Insights: Multiple "Warning" and "Critical" insights
- Recommendations: All "High" priority

### Test Case 3: Electric Vehicle

**Input:**
```json
{
  "engine_type": "electric",
  "fuel_consumed_gallons": 0.0,
  "avg_fuel_efficiency_mpg": 0.0,
  "avg_rpm": 0
}
```

**Expected Output:**
- Score: Based on energy efficiency metrics (kWh/mile if available)
- Insights: Focus on regenerative braking, climate control usage
- No RPM-related insights

---

## Model-Specific Considerations

### For Haiku (Lightweight Model)
- Emphasize **structured output format** (provide clear JSON schema)
- Use **few-shot examples** (include 1-2 example inputs/outputs in prompt)
- Keep **context window small** (<1500 tokens total)
- **Validate output** strictly (expect occasional formatting errors)

### For Local Models (Llama 3 8B)
- Similar to Haiku considerations
- **Temperature: 0.3** (reduce creativity, increase consistency)
- **Top-p: 0.9** (nucleus sampling for focused outputs)
- May require **fine-tuning** on automotive data for optimal results

---

## Integration Points

### Dependencies
- **Session Summary** (from feature 006-data-recording): Provides input data
- **AI Provider Layer** (from feature 005-real-ai): OpenAI/Ollama client

### Consumers
- **Agent Dashboard** (frontend): Displays results in UI
- **Agent Orchestrator** (backend): Dispatches requests, handles caching

### Related Agents
- **Safety Advisor**: May reference hard braking/acceleration events
- **Diagnostic Agent**: May defer maintenance recommendations to Diagnostic

---

## Prompt Engineering Notes

### Token Budget
- System prompt: ~300 tokens
- Session data (JSON): ~400 tokens
- Few-shot example (optional): ~200 tokens
- Output buffer: ~500 tokens
- **Total**: ~1400 tokens (within 4000 token limit)

### Prompt Improvements (Iterative)
1. **V1**: Basic prompt with minimal guidelines → Poor scoring consistency
2. **V2**: Added scoring rubric → Better score calibration
3. **V3**: Added few-shot example → Improved JSON formatting
4. **V4**: Added edge case handling → Handles electric vehicles correctly
5. **V5 (current)**: Added comparison to EPA ratings → More contextual insights

---

## Success Criteria

- ✅ Score consistency: ±5 points for same input across multiple runs
- ✅ JSON parsing: 95%+ success rate (valid JSON output)
- ✅ Insight relevance: Human review rates 80%+ of insights as actionable
- ✅ Latency: < 3 seconds per analysis (Haiku model)
- ✅ Cost: < $0.001 per analysis (at $0.25/1M tokens)

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-11-25 | Initial contract definition |
