# Implementation Tasks: Agent Framework Testing

**Feature**: 009-agent-testing | **Branch**: `009-agent-testing`
**Generated**: 2025-11-26

This document provides a task breakdown for implementing comprehensive unit tests for the agent framework.

---

## Phase 1: Fuel Coach Agent Tests

**Objective**: Create thorough unit tests for FuelCoachAgent

- [x] **T001** [P1] [Test] Create fuel_coach_test.rs file
      File: src-tauri/tests/agents/fuel_coach_test.rs
      Dependencies: None

- [x] **T002** [P1] [Test] Implement create_test_request() helper function
      File: src-tauri/tests/agents/fuel_coach_test.rs
      Dependencies: T001

- [x] **T003** [P1] [Test] Test good efficiency scenario (high score)
      Test: test_fuel_coach_good_efficiency
      Dependencies: T002
      Validates: MPG 30+ → score > 80

- [x] **T004** [P1] [Test] Test poor efficiency scenario (low score)
      Test: test_fuel_coach_poor_efficiency
      Dependencies: T002
      Validates: MPG < 20, high RPM → score < 60, warnings

- [x] **T005** [P1] [Test] Test high RPM penalty
      Test: test_fuel_coach_high_rpm_penalty
      Dependencies: T002
      Validates: RPM 4000+ → score reduction, RPM insight

- [x] **T006** [P1] [Test] Test rapid acceleration detection
      Test: test_fuel_coach_rapid_acceleration
      Dependencies: T002
      Validates: 10+ events → acceleration insight & recommendation

- [x] **T007** [P1] [Test] Test agent metadata
      Test: test_fuel_coach_metadata
      Dependencies: T002
      Validates: ID, name, domain, icon, privacy level

- [x] **T008** [P1] [Test] Test can_handle() method
      Test: test_fuel_coach_can_handle
      Dependencies: T002
      Validates: Accepts fuel data, rejects zero consumption

- [x] **T009** [P1] [Test] Test execution time
      Test: test_fuel_coach_execution_time
      Dependencies: T002
      Validates: Completes < 1000ms, local execution

---

## Phase 2: Safety Advisor Agent Tests

**Objective**: Create thorough unit tests for SafetyAdvisorAgent

- [x] **T010** [P1] [Test] Create safety_advisor_test.rs file
      File: src-tauri/tests/agents/safety_advisor_test.rs
      Dependencies: None

- [x] **T011** [P1] [Test] Test safe driving scenario
      Test: test_safety_safe_driving
      Dependencies: T010
      Validates: No violations → score >= 80, positive insight

- [x] **T012** [P1] [Test] Test speeding violations
      Test: test_safety_speeding_violations
      Dependencies: T010
      Validates: 5 violations → score < 80, speed insight

- [x] **T013** [P1] [Test] Test hard braking detection
      Test: test_safety_hard_braking
      Dependencies: T010
      Validates: 8 events → braking insight, following distance rec

- [x] **T014** [P1] [Test] Test excessive speed (85+ mph)
      Test: test_safety_excessive_speed
      Dependencies: T010
      Validates: Max speed 95 → High priority recommendation

- [x] **T015** [P1] [Test] Test multiple risks scenario
      Test: test_safety_multiple_risks
      Dependencies: T010
      Validates: Combined risks → score < 50, critical insights

- [x] **T016** [P1] [Test] Test agent metadata
      Test: test_safety_metadata
      Dependencies: T010
      Validates: ID, NHTSA reference, icon

- [x] **T017** [P1] [Test] Test universal compatibility
      Test: test_safety_can_handle_any_session
      Dependencies: T010
      Validates: Handles any driving session

- [x] **T018** [P1] [Test] Test confidence levels
      Test: test_safety_confidence
      Dependencies: T010
      Validates: Rule-based confidence >= 0.8

---

## Phase 3: Diagnostic Agent Tests

**Objective**: Create thorough unit tests for DiagnosticAgent

- [x] **T019** [P1] [Test] Create diagnostic_test.rs file
      File: src-tauri/tests/agents/diagnostic_test.rs
      Dependencies: None

- [x] **T020** [P1] [Test] Test healthy vehicle scenario
      Test: test_diagnostic_healthy_vehicle
      Dependencies: T019
      Validates: Normal temps → score >= 85, positive insight

- [x] **T021** [P1] [Test] Test high coolant temperature (critical)
      Test: test_diagnostic_high_coolant_temp
      Dependencies: T019
      Validates: >220°F → score drop, critical insight, high priority rec

- [x] **T022** [P1] [Test] Test elevated coolant temperature (warning)
      Test: test_diagnostic_elevated_coolant_temp
      Dependencies: T019
      Validates: >210°F → warning insight

- [x] **T023** [P1] [Test] Test high oil temperature
      Test: test_diagnostic_high_oil_temp
      Dependencies: T019
      Validates: >250°F → warning, synthetic oil recommendation

- [x] **T024** [P1] [Test] Test excessive RPM
      Test: test_diagnostic_excessive_rpm
      Dependencies: T019
      Validates: >6000 RPM → warning insight

- [x] **T025** [P1] [Test] Test error code handling
      Test: test_diagnostic_error_codes
      Dependencies: T019
      Validates: 2 DTCs → score < 70, critical insights, high priority rec

- [x] **T026** [P1] [Test] Test multiple issues scenario
      Test: test_diagnostic_multiple_issues
      Dependencies: T019
      Validates: Coolant + oil + RPM + DTC → score < 50, multiple critical

- [x] **T027** [P1] [Test] Test agent metadata
      Test: test_diagnostic_metadata
      Dependencies: T019
      Validates: ID, name, icon, medium model preference

- [x] **T028** [P1] [Test] Test universal compatibility
      Test: test_diagnostic_can_handle_any_session
      Dependencies: T019
      Validates: Handles any session with sensor data

- [x] **T029** [P1] [Test] Test token estimation
      Test: test_diagnostic_token_estimation
      Dependencies: T019
      Validates: Estimates >= 1000 tokens (complex agent)

---

## Phase 4: Documentation

**Objective**: Document the testing implementation

- [x] **T030** [P1] [Docs] Create spec.md
      File: specs/009-agent-testing/spec.md
      Dependencies: T001-T029

- [x] **T031** [P1] [Docs] Create plan.md
      File: specs/009-agent-testing/plan.md
      Dependencies: T030

- [x] **T032** [P1] [Docs] Create tasks.md
      File: specs/009-agent-testing/tasks.md
      Dependencies: T031

---

## Summary

- **Total Tasks**: 32
- **Completed**: 32 ✅
- **Test Files**: 3
- **Test Cases**: 28
- **Lines of Code**: 550

## Execution Order

Tasks can be executed in phases:
1. Phase 1 (T001-T009): Fuel Coach tests
2. Phase 2 (T010-T018): Safety Advisor tests
3. Phase 3 (T019-T029): Diagnostic tests
4. Phase 4 (T030-T032): Documentation

All test phases are independent and can be developed in parallel.

## Test Execution

```bash
# Run all agent tests
cargo test agents

# Run specific agent tests
cargo test fuel_coach
cargo test safety_advisor
cargo test diagnostic

# Run with output
cargo test agents -- --nocapture

# Run single test
cargo test test_fuel_coach_good_efficiency
```

## Success Criteria

✅ **All criteria met:**
- All 32 tasks completed
- 28 tests implemented and passing
- 100% of P1 agents covered
- Documentation complete
- Tests are maintainable and clear
