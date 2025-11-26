# Implementation Plan: Agent Framework Testing

**Feature**: 009-agent-testing
**Branch**: `009-agent-testing`
**Status**: Completed

## Technical Context

### Existing Code
- **Agent Framework**: Complete implementation in `src-tauri/src/agents/`
  - 3 P1 agents: FuelCoachAgent, SafetyAdvisorAgent, DiagnosticAgent
  - Trait: `AnalysisAgent` with async `analyze()` method
  - Types: AnalysisRequest, AnalysisResult, Insight, Recommendation
  - Infrastructure: Registry, Orchestrator, Cache

### Tech Stack
- **Testing**: Rust native `#[test]` with tokio runtime
- **Async**: `#[tokio::test]` macro for async test functions
- **Location**: `src-tauri/tests/agents/` directory
- **Assertions**: Standard Rust assert macros

### Testing Patterns
```rust
#[tokio::test]
async fn test_scenario_name() {
    // Arrange
    let agent = AgentImpl::new();
    let request = create_test_request(params);

    // Act
    let result = agent.analyze(&request).await.unwrap();

    // Assert
    assert_eq!(result.agent_id, "expected-id");
    assert!(result.score.unwrap() > threshold);
}
```

## Architecture

### Test File Organization
```
src-tauri/tests/agents/
├── fuel_coach_test.rs      # Fuel Efficiency Coach tests
├── safety_advisor_test.rs   # Safety Advisor tests
├── diagnostic_test.rs       # Diagnostic Technician tests
└── mod.rs                   # (future) Test utilities
```

### Test Categories

**1. Scoring Tests**
- Validate score calculations for various scenarios
- Test score boundaries (0-100 range)
- Verify factor weights (RPM, MPG, speed, etc.)

**2. Insight Tests**
- Verify insight categorization (Info/Warning/Critical/Positive)
- Validate insight content and descriptions
- Test supporting data inclusion

**3. Recommendation Tests**
- Validate priority assignment (Low/Medium/High)
- Test effort level classification (Easy/Moderate/Significant)
- Verify actionable recommendations

**4. Metadata Tests**
- Agent ID, name, icon validation
- Domain and privacy level checks
- Version and description verification

**5. Edge Case Tests**
- Boundary values (0 MPG, 999 speed, etc.)
- Missing data handling
- Extreme scenarios

## Constitution Check

✅ **No violations detected**

- Tests do not use `.unwrap()` in production code (only in test assertions)
- All async operations use proper error handling
- Test data is realistic and representative
- No hardcoded secrets or credentials
- Tests are deterministic and repeatable

## Implementation Tasks

### Phase 1: Fuel Coach Tests
- [x] T001: Create `fuel_coach_test.rs`
- [x] T002: Implement `create_test_request()` helper
- [x] T003: Test good efficiency scenarios (score > 80)
- [x] T004: Test poor efficiency scenarios (score < 60)
- [x] T005: Test high RPM penalty
- [x] T006: Test rapid acceleration detection
- [x] T007: Test metadata validation
- [x] T008: Test session compatibility

### Phase 2: Safety Advisor Tests
- [x] T009: Create `safety_advisor_test.rs`
- [x] T010: Test safe driving (score >= 80)
- [x] T011: Test speeding violations
- [x] T012: Test hard braking detection
- [x] T013: Test excessive speed warnings
- [x] T014: Test multiple risk scenarios
- [x] T015: Test metadata and confidence

### Phase 3: Diagnostic Tests
- [x] T016: Create `diagnostic_test.rs`
- [x] T017: Test healthy vehicle (score >= 85)
- [x] T018: Test coolant temperature monitoring
- [x] T019: Test oil temperature warnings
- [x] T020: Test excessive RPM
- [x] T021: Test error code handling
- [x] T022: Test multiple concurrent issues
- [x] T023: Test token estimation

### Phase 4: Documentation
- [x] T024: Create spec.md
- [x] T025: Create plan.md
- [x] T026: Create tasks.md
- [x] T027: Update README if needed

## Test Data Strategy

### Realistic Values
- **Vehicle**: Honda Civic 2018, Manual, Gasoline
- **Good MPG**: 25-35 MPG
- **Poor MPG**: 10-20 MPG
- **Safe Speed**: 55-65 mph
- **Unsafe Speed**: 80-100 mph
- **Normal RPM**: 2000-3000
- **High RPM**: 4000-6000

### Session Duration
- Standard: 1800 seconds (30 minutes)
- Distance: 15-30 miles
- Fuel: 0.5-2.0 gallons

## Success Criteria

✅ **All criteria met:**

1. **Coverage**: All 3 P1 agents have tests
2. **Quantity**: Minimum 8 tests per agent
3. **Quality**: Tests verify behavior, not implementation
4. **Pass Rate**: 100% tests passing
5. **Speed**: All tests complete in < 10 seconds
6. **Maintainability**: Clear test names and assertions

## Risks & Mitigation

| Risk | Impact | Mitigation | Status |
|------|--------|------------|--------|
| Async test complexity | Medium | Use `#[tokio::test]` macro | ✅ Done |
| Test data maintenance | Low | Parameterized helpers | ✅ Done |
| False positives | Medium | Validate against real scenarios | ✅ Done |
| Slow tests | Low | Rule-based agents are fast | ✅ Fast |

## Estimated Effort

- **Actual**: ~2 hours
- **Total Tests**: 28
- **Total Lines**: 550
- **Files Created**: 3

## Dependencies

- ✅ Tokio test runtime
- ✅ Agent framework implementation
- ✅ No external dependencies

## Future Work

- Integration tests for orchestrator
- Cache behavior tests
- Performance benchmarks
- Property-based testing with quickcheck
- Coverage reports with tarpaulin
