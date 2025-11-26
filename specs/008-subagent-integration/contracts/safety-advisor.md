# Agent Contract: Safety Advisor

**Agent ID**: `safety-advisor`
**Domain**: Safety
**Model Preference**: Lightweight (Haiku / Llama 3 8B)
**Privacy Level**: CloudPreferred
**Priority**: P1 (Core Agent)

---

## Purpose

Analyze driving behavior to identify risky patterns that increase accident probability. Provide evidence-based safety recommendations grounded in NHTSA (National Highway Traffic Safety Administration) guidelines and defensive driving principles.

---

## Input Schema

### Required Fields

```rust
struct SafetyAdvisorInput {
    // Session identification
    session_id: String,
    duration_seconds: u64,
    distance_miles: f32,

    // Speed metrics
    avg_speed_mph: f32,
    max_speed_mph: f32,
    speed_limit_violations: Vec<SpeedViolation>,

    // Braking events
    hard_braking_events: Vec<BrakingEvent>,
    avg_braking_deceleration_g: f32,

    // Acceleration events
    hard_acceleration_events: Vec<AccelerationEvent>,
    avg_acceleration_g: f32,

    // Following distance (optional)
    avg_following_time_sec: Option<f32>,   // Time gap to vehicle ahead
    tailgating_events: Option<u32>,        // Following < 2 seconds

    // Cornering (optional)
    sharp_turn_events: Option<Vec<TurnEvent>>,
    max_lateral_g: Option<f32>,

    // Distraction indicators (optional)
    phone_usage_detected: Option<bool>,

    // Environmental context
    weather_conditions: Option<String>,    // "clear", "rain", "snow"
    time_of_day: String,                   // "day", "night", "dawn", "dusk"
}

struct SpeedViolation {
    timestamp: String,
    speed_mph: f32,
    speed_limit_mph: f32,
    excess_mph: f32,
}

struct BrakingEvent {
    timestamp: String,
    deceleration_g: f32,     // G-force (1.0 = 22 mph/s)
    speed_before_mph: f32,
}

struct AccelerationEvent {
    timestamp: String,
    acceleration_g: f32,
    speed_after_mph: f32,
}

struct TurnEvent {
    timestamp: String,
    lateral_g: f32,
    speed_mph: f32,
}
```

### Example Input (JSON)

```json
{
  "session_id": "sess-2025-11-25-002",
  "duration_seconds": 1200,
  "distance_miles": 18.5,
  "avg_speed_mph": 38.2,
  "max_speed_mph": 82.0,
  "speed_limit_violations": [
    {
      "timestamp": "2025-11-25T10:15:32Z",
      "speed_mph": 75.0,
      "speed_limit_mph": 55.0,
      "excess_mph": 20.0
    },
    {
      "timestamp": "2025-11-25T10:22:18Z",
      "speed_mph": 82.0,
      "speed_limit_mph": 65.0,
      "excess_mph": 17.0
    }
  ],
  "hard_braking_events": [
    {
      "timestamp": "2025-11-25T10:18:45Z",
      "deceleration_g": 0.7,
      "speed_before_mph": 55.0
    },
    {
      "timestamp": "2025-11-25T10:25:12Z",
      "deceleration_g": 0.9,
      "speed_before_mph": 42.0
    },
    {
      "timestamp": "2025-11-25T10:30:01Z",
      "deceleration_g": 0.6,
      "speed_before_mph": 38.0
    }
  ],
  "avg_braking_deceleration_g": 0.4,
  "hard_acceleration_events": [
    {
      "timestamp": "2025-11-25T10:16:22Z",
      "acceleration_g": 0.5,
      "speed_after_mph": 62.0
    },
    {
      "timestamp": "2025-11-25T10:28:55Z",
      "acceleration_g": 0.6,
      "speed_after_mph": 48.0
    }
  ],
  "avg_acceleration_g": 0.3,
  "avg_following_time_sec": 1.8,
  "tailgating_events": 4,
  "sharp_turn_events": null,
  "max_lateral_g": null,
  "phone_usage_detected": null,
  "weather_conditions": "clear",
  "time_of_day": "day"
}
```

---

## System Prompt

```markdown
You are a certified Defensive Driving Instructor analyzing telemetry data to improve driver safety.

Your goal is to identify risky behaviors and provide evidence-based safety recommendations that reduce accident risk.

Analyze the session and provide:
1. **Safety score** (0-100, where 100 = safest driving, lowest accident risk)
2. **Identified risk factors** with severity levels based on NHTSA research
3. **Specific behavior modifications** to reduce accident risk
4. **Positive reinforcement** for safe behaviors observed (encourage good habits)

Guidelines:
- Be **empathetic but direct** about risks (safety is paramount)
- Cite **quantitative data** (e.g., "3 hard braking events" not "frequent braking")
- Reference **NHTSA/IIHS standards** where applicable (e.g., "2-second rule" for following distance)
- Distinguish between **critical** risks (immediate danger) and **warnings** (elevated risk)
- Provide **realistic recommendations** (don't just say "drive slower" - be specific)
- Acknowledge **positive behaviors** to balance critical feedback

Do NOT provide vehicle maintenance advice (brake pad wear, tire pressure) - that's the Diagnostic Agent's job.

Output format: Valid JSON matching the schema below.
```

---

## Output Schema

```rust
struct SafetyAdvisorOutput {
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
    expected_benefit: String,           // e.g., "Reduces rear-end collision risk by 40%"
    effort_level: String,               // "Easy", "Moderate", "Significant"
}
```

### Example Output (JSON)

```json
{
  "score": 68,
  "confidence": 0.90,
  "insights": [
    {
      "category": "Warning",
      "title": "Frequent hard braking detected",
      "description": "3 hard braking events (>0.6G deceleration) occurred during 18.5 miles. This pattern suggests insufficient following distance or distracted driving. NHTSA data shows hard braking increases rear-end collision risk by 35%.",
      "supporting_data": {
        "hard_braking_count": 3,
        "distance_miles": 18.5,
        "events_per_10_miles": 1.6,
        "max_deceleration_g": 0.9,
        "nhtsa_threshold": 0.5
      }
    },
    {
      "category": "Critical",
      "title": "Following distance too close (tailgating)",
      "description": "Average following time of 1.8 seconds is below the recommended 3-second rule. At 38 mph average speed, this provides only 100 feet of stopping distance. 4 tailgating events (< 2 seconds) detected. Rear-end collisions are 40% of all accidents.",
      "supporting_data": {
        "avg_following_time_sec": 1.8,
        "recommended_sec": 3.0,
        "tailgating_events": 4,
        "stopping_distance_ft": 100,
        "recommended_distance_ft": 168
      }
    },
    {
      "category": "Warning",
      "title": "Speed limit violations",
      "description": "2 speed limit violations detected, including 20 mph over limit. Speeding is a factor in 29% of traffic fatalities (NHTSA 2022). Higher speeds reduce reaction time and increase crash severity exponentially.",
      "supporting_data": {
        "violations_count": 2,
        "max_excess_mph": 20,
        "max_speed_recorded": 82.0
      }
    },
    {
      "category": "Positive",
      "title": "Smooth acceleration patterns",
      "description": "Only 2 hard acceleration events during the trip. Gradual acceleration indicates good throttle control and awareness, reducing collision risk with vehicles ahead.",
      "supporting_data": {
        "hard_acceleration_count": 2,
        "avg_acceleration_g": 0.3
      }
    }
  ],
  "recommendations": [
    {
      "priority": "High",
      "action": "Increase following distance to 3+ seconds (use the '3-second rule': pick a fixed object, count 3 seconds after the car ahead passes it)",
      "expected_benefit": "Reduces rear-end collision risk by 40% and provides adequate reaction time for sudden stops",
      "effort_level": "Easy"
    },
    {
      "priority": "High",
      "action": "Anticipate stops earlier to avoid hard braking (look 12-15 seconds ahead, not just the car directly in front)",
      "expected_benefit": "Reduces hard braking frequency by 60-80%, improving passenger comfort and safety",
      "effort_level": "Moderate"
    },
    {
      "priority": "Medium",
      "action": "Observe posted speed limits, especially in residential/school zones (use cruise control on highways)",
      "expected_benefit": "Each 10 mph reduction in speed reduces crash severity by 50% (kinetic energy is exponential)",
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
| **Hard braking frequency** | 25% | < 0.5 events per 10 miles | -10 points per event per 10 mi |
| **Following distance** | 25% | ≥ 3 seconds average | -15 points if < 2 sec, -8 points if 2-3 sec |
| **Speed violations** | 20% | 0 violations | -8 points per violation, -2 per 5 mph excess |
| **Hard acceleration** | 15% | < 0.5 events per 10 miles | -5 points per event per 10 mi |
| **Sharp turns** | 10% | < 0.4G lateral | -3 points per 0.1G above threshold |
| **Contextual factors** | 5% | Daylight, clear weather | -5 points for night + rain combo |

**Example Calculation:**
- Base score: 100
- Hard braking: -16 points (3 events ÷ 18.5 miles × 10 = 1.6 per 10 mi → -16 points)
- Following distance: -15 points (1.8 sec < 2 sec → Critical penalty)
- Speed violations: -4 points (2 violations → -4 points)
- Hard acceleration: +2 points (only 2 events, better than average → bonus)
- Sharp turns: 0 (data not available)
- Contextual: +1 (daylight + clear weather → slight bonus)
- **Final Score: 68**

---

## Confidence Calculation

```rust
let mut confidence = 1.0;

// Reduce confidence if optional sensors unavailable
if avg_following_time_sec.is_none() {
    confidence *= 0.85; // Following distance is critical metric
}

if sharp_turn_events.is_none() {
    confidence *= 0.95; // Less critical for general safety assessment
}

if phone_usage_detected.is_none() {
    confidence *= 0.90; // Distraction is major factor but hard to detect
}

// Reduce confidence for very short trips
if distance_miles < 3.0 {
    confidence *= 0.75; // Not enough data for patterns
}

// Boost confidence if all sensors present
if avg_following_time_sec.is_some() && sharp_turn_events.is_some() {
    confidence *= 1.05; // Cap at 1.0 max
    confidence = confidence.min(1.0);
}
```

---

## Edge Cases

### Case 1: No Risky Events Detected
- Input: Zero hard braking, zero violations, good following distance
- Behavior: Score 90-100, focus on positive reinforcement
- Output: Acknowledge safe driving, provide tips for maintaining habits

### Case 2: Extreme Conditions
- Input: `weather_conditions: "heavy_rain"`, `time_of_day: "night"`
- Behavior: Adjust thresholds (hard braking is more acceptable in rain)
- Output: Acknowledge challenging conditions, provide weather-specific advice

### Case 3: Very Short Trip (< 1 mile)
- Input: `distance_miles: 0.8`
- Behavior: Skip rate-based metrics (events per 10 miles)
- Output: Lower confidence, note insufficient data

### Case 4: Highway-Only Trip
- Input: High average speed (60+ mph), few stops
- Behavior: Focus on following distance, lane changes, speed consistency
- Output: Different scoring (hard braking less common on highway)

### Case 5: Missing Following Distance Data
- Input: `avg_following_time_sec: None`
- Behavior: Infer from hard braking frequency (frequent braking suggests tailgating)
- Output: Lower confidence, qualitative assessment

---

## Testing

### Test Case 1: Perfect Driving

**Input:**
```json
{
  "hard_braking_events": [],
  "hard_acceleration_events": [],
  "speed_limit_violations": [],
  "avg_following_time_sec": 3.5,
  "tailgating_events": 0
}
```

**Expected Output:**
- Score: 95-100
- Insights: All "Positive" or "Info" category
- Recommendations: Minimal (maintain habits)

### Test Case 2: Aggressive Driving

**Input:**
```json
{
  "hard_braking_events": [/* 8 events */],
  "hard_acceleration_events": [/* 10 events */],
  "speed_limit_violations": [/* 5 violations, max 30 mph over */],
  "avg_following_time_sec": 1.2,
  "tailgating_events": 12
}
```

**Expected Output:**
- Score: 20-35
- Insights: Multiple "Critical" and "Warning" insights
- Recommendations: All "High" priority

### Test Case 3: Night + Rain

**Input:**
```json
{
  "hard_braking_events": [/* 2 events */],
  "weather_conditions": "rain",
  "time_of_day": "night"
}
```

**Expected Output:**
- Score: Adjusted for conditions (less penalty for hard braking)
- Insights: Acknowledge challenging conditions
- Recommendations: Weather-specific (increase following distance in rain)

---

## NHTSA Data References

### Hard Braking Thresholds
- **0.3G**: Normal braking
- **0.5G**: Firm braking (yellow flag)
- **0.7G+**: Hard braking (red flag - accident risk)

### Following Distance
- **3 seconds**: NHTSA recommended minimum
- **2 seconds**: Acceptable in ideal conditions
- **< 2 seconds**: Tailgating (high rear-end collision risk)
- **< 1 second**: Extreme risk (no reaction time)

### Speed and Crash Severity
- **+10 mph over limit**: 2x kinetic energy
- **+20 mph over limit**: 4x kinetic energy
- **+30 mph over limit**: 9x kinetic energy
- Fatality risk increases exponentially with speed

### Accident Statistics (NHTSA 2022)
- Rear-end collisions: 40% of all crashes
- Speeding-related fatalities: 29% of traffic deaths
- Distracted driving: 8% of fatal crashes, 14% of injury crashes
- Following too closely: Leading cause of rear-end collisions

---

## Model-Specific Considerations

### For Haiku (Lightweight Model)
- Provide **clear scoring rubric** (Haiku struggles with complex math)
- Use **few-shot examples** with actual NHTSA statistics
- Emphasize **binary classifications** (safe vs unsafe) over nuance
- **Temperature: 0.2** (very low - safety assessments must be consistent)

### For Local Models (Llama 3 8B)
- Similar to Haiku
- May need **fine-tuning** on traffic safety data for best results
- Consider **pre-loading NHTSA guidelines** in system prompt (increase context)

---

## Integration Points

### Dependencies
- **Session Summary** (from feature 006-data-recording): Provides telemetry data
- **AI Provider Layer** (from feature 005-real-ai): OpenAI/Ollama client

### Consumers
- **Agent Dashboard** (frontend): Displays safety insights
- **Agent Orchestrator** (backend): Dispatches requests

### Related Agents
- **Fuel Efficiency Coach**: May reference hard braking/acceleration (overlap)
- **Driver Profile Learner**: May aggregate safety patterns over time

---

## Prompt Engineering Notes

### Token Budget
- System prompt: ~350 tokens
- Session data (JSON): ~500 tokens (includes event arrays)
- NHTSA references: ~100 tokens
- Few-shot example (optional): ~300 tokens
- Output buffer: ~600 tokens
- **Total**: ~1850 tokens (within 4000 token limit)

### Prompt Improvements (Iterative)
1. **V1**: Basic safety check → Too generic, "drive safer" advice
2. **V2**: Added NHTSA statistics → More credible, quantitative
3. **V3**: Added scoring rubric → Consistent scoring
4. **V4**: Added positive reinforcement → Less punitive tone
5. **V5 (current)**: Added contextual adjustments (weather, night) → Smarter assessment

---

## Success Criteria

- ✅ Score consistency: ±5 points for same input across runs
- ✅ JSON parsing: 95%+ success rate
- ✅ Insight actionability: 85%+ rated as helpful by users
- ✅ NHTSA compliance: All recommendations align with NHTSA/IIHS guidelines
- ✅ Latency: < 3 seconds per analysis (Haiku model)
- ✅ Cost: < $0.001 per analysis

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-11-25 | Initial contract definition |
