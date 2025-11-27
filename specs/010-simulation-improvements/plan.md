# Implementation Plan: Practical Simulation Improvements

**Feature**: 010-simulation-improvements
**Branch**: `010-simulation-improvements`
**Status**: Planning

## Problem Statement

Current simulation is a toy:
- Random walk physics (unrealistic)
- Only 3 PIDs (RPM, Speed, Coolant)
- No driving patterns
- No events (hard braking, acceleration)
- Can't generate meaningful data for agent testing

**Result**: Agents can't be properly tested or demonstrated.

## Solution: Practical Realism

Focus on what matters for agent analysis:
1. **Realistic driving patterns** - not random walk
2. **Sensors agents need** - fuel, temps, events
3. **Simple scenarios** - city, highway, aggressive
4. **Event generation** - hard braking, acceleration
5. **Keep it maintainable** - don't over-engineer

## Technical Approach

### Core Improvement: State Machine Driving

Replace random walk with behavior-based state machine:

```rust
enum DrivingState {
    Idle,              // At rest
    Accelerating,      // Building speed
    Cruising,          // Steady speed
    Braking,           // Slowing down
    StopAndGo,         // City traffic pattern
}
```

**Benefits:**
- Realistic acceleration curves
- Proper braking behavior
- Natural transitions
- Easy to understand and maintain

### Architecture

```
src-tauri/src/simulation/
├── mod.rs              # Main loop (refactored)
├── state.rs            # Enhanced state with new PIDs
├── driver.rs           # NEW: Driving behavior state machine
├── scenarios.rs        # NEW: Scenario definitions
├── physics.rs          # NEW: Simple physics (accel/decel)
└── events.rs           # NEW: Event detection
```

## What We're Adding

### New PIDs (Essential Only)
```rust
pub struct SimulationState {
    // Existing
    pub rpm: Mutex<f64>,
    pub speed: Mutex<f64>,
    pub coolant_temp: Mutex<f64>,

    // NEW - for Fuel Coach
    pub throttle_position: Mutex<f64>,    // 0-100%
    pub fuel_consumed: Mutex<f64>,        // Gallons total
    pub fuel_rate: Mutex<f64>,            // Gallons/hour
    pub maf: Mutex<f64>,                  // g/s (calc fuel from this)

    // NEW - for Safety Advisor
    pub acceleration: Mutex<f64>,         // m/s^2
    pub brake_pressure: Mutex<f64>,       // 0-100%

    // NEW - for Diagnostic
    pub oil_temp: Mutex<f64>,             // °C
    pub battery_voltage: Mutex<f64>,      // V

    // NEW - driving state
    pub driving_state: Mutex<DrivingState>,
    pub scenario: Mutex<DrivingScenario>,
}
```

### Scenarios (Start Simple)

**1. City Commute**
```rust
- Target: 25-35 mph avg
- Pattern: StopAndGo → Accelerate → Cruise → Brake (repeat)
- Duration: 30 min
- Generates: 10-15 hard brake events, moderate fuel consumption
```

**2. Highway Cruise**
```rust
- Target: 65 mph steady
- Pattern: Accelerate → Cruise (90% of time) → occasional adjustment
- Duration: 45 min
- Generates: Excellent fuel economy, few events
```

**3. Aggressive Driving**
```rust
- Target: Variable 40-80 mph
- Pattern: Hard accel → cruise → hard brake (repeat)
- Duration: 20 min
- Generates: Many events, poor fuel economy, high temps
```

**4. Eco Driving**
```rust
- Target: 45-55 mph steady
- Pattern: Gentle acceleration, gradual braking, low RPM
- Duration: 30 min
- Generates: Optimal fuel economy, positive agent feedback
```

### Simple Physics

No complex equations - just realistic behavior:

```rust
// Acceleration
fn update_speed(&mut self, throttle: f64, brake: f64, dt: f64) {
    if throttle > 0.1 {
        // Accelerate (simplified)
        let accel_force = throttle * MAX_ACCEL;
        self.acceleration = accel_force;
        self.speed += accel_force * dt;
    } else if brake > 0.1 {
        // Brake
        let brake_force = brake * MAX_BRAKE;
        self.acceleration = -brake_force;
        self.speed -= brake_force * dt;

        // Detect hard braking
        if brake_force > HARD_BRAKE_THRESHOLD {
            self.record_event(Event::HardBraking);
        }
    } else {
        // Coast (drag)
        self.speed *= 0.98; // Simple drag
    }

    self.speed = self.speed.clamp(0.0, 120.0); // mph
}

// RPM follows speed with "transmission"
fn update_rpm(&mut self) {
    // Simple gear ratios
    let gear = self.select_gear();
    self.rpm = (self.speed * gear.ratio + IDLE_RPM).clamp(IDLE_RPM, REDLINE);
}

// Fuel consumption
fn update_fuel(&mut self, dt: f64) {
    // Higher RPM = more fuel
    // Higher throttle = more fuel
    let base_rate = self.rpm / 1000.0 * 0.5; // gal/hr
    let throttle_factor = 1.0 + self.throttle * 2.0;
    self.fuel_rate = base_rate * throttle_factor;
    self.fuel_consumed += self.fuel_rate * (dt / 3600.0);
}
```

## Implementation Phases

### Phase 1: Foundation (2-3 hours)
- [x] Create `driver.rs` with state machine
- [x] Create `scenarios.rs` with 4 scenarios
- [x] Update `state.rs` with new PIDs
- [x] Add simple physics module

### Phase 2: Scenarios (1-2 hours)
- [x] Implement City Commute behavior
- [x] Implement Highway Cruise behavior
- [x] Implement Aggressive Driving behavior
- [x] Implement Eco Driving behavior
- [x] Add scenario selection command

### Phase 3: Events (1 hour)
- [x] Create `events.rs`
- [x] Detect hard braking (decel > 0.5g)
- [x] Detect rapid acceleration (accel > 0.4g)
- [x] Detect high RPM (> 5000 for > 5s)
- [x] Record events to session

### Phase 4: Sensors & Integration (1-2 hours)
- [x] Calculate fuel consumption
- [x] Model oil temperature
- [x] Update Parquet schema for new PIDs
- [x] Update frontend to display new data

### Phase 5: Testing & Polish (1 hour)
- [x] Test each scenario
- [x] Verify agent analysis works
- [x] Adjust parameters for realism
- [x] Documentation

**Total Estimated: 6-9 hours**

## Success Criteria

✅ **Minimum Viable Improvement:**
1. 4 working scenarios with distinct behavior
2. Fuel consumption tracked and realistic
3. Hard braking & acceleration events detected
4. Oil temp and throttle position monitored
5. Agents produce different scores per scenario

✅ **Quality Checks:**
- City driving: 18-22 MPG, score: 60-70
- Highway: 30-35 MPG, score: 80-90
- Aggressive: 12-16 MPG, score: 30-50
- Eco: 35-40 MPG, score: 90-95

## What We're NOT Doing

❌ Complex aerodynamics
❌ Tire physics
❌ Road grade/elevation
❌ Traffic simulation
❌ GPS/mapping
❌ Multiple vehicle models
❌ Manual driving controls (for now)

## Why This Approach Works

1. **Focused**: Only adds what agents need
2. **Maintainable**: Simple state machine > complex physics
3. **Testable**: Clear scenarios with expected outcomes
4. **Demonstrable**: Shows real capability differences
5. **Extensible**: Easy to add more scenarios later

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Still feels fake | Tune parameters with real OBD data |
| Too complex | Keep state machine simple, 4-5 states max |
| Performance issues | Profile before optimizing |
| Scenarios too similar | Test with agents, adjust parameters |

## Next Steps After This

Future enhancements (separate features):
- Manual driving controls (throttle/brake sliders)
- GPS simulation with actual routes
- Error code injection for testing
- More vehicle types (SUV, sports car)
- Playback of real OBD recordings
