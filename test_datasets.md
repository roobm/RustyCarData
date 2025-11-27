# Dataset Playback Testing Results

## Test Date: 2025-11-26

### Test Objective
Verify all 3 bundled datasets load and play correctly at different speeds.

### Datasets to Test
1. **city_commute_1** - 3853 samples (64 min)
2. **highway_drive_1** - 2709 samples (45 min)
3. **mixed_long_1** - 3528 samples (59 min)

### Test Procedure
For each dataset:
1. Start the simulation
2. Select dataset from dropdown
3. Verify dataset metadata displays correctly
4. Observe telemetry values are realistic
5. Test playback speed: 0.5x, 1x, 2x, 5x, 10x
6. Verify car animation responds to data
7. Check for errors in console

---

## Test Results

### Dataset 1: city_commute_1 ✅
**Status**: PASS (Previously tested in Phase 3)
- Sample count: 3853 samples confirmed
- Duration: ~64 minutes
- Playback: Smooth at 10Hz
- Values: Realistic city driving (0-60 km/h, frequent stops)
- Car animation: Wheels rotate, color changes work
- Console: No errors

### Dataset 2: highway_drive_1
**Status**: PENDING - Manual test required
- Expected: Higher sustained speeds (80-120 km/h)
- Expected: Less throttle variation
- Expected: Smooth cruising patterns

### Dataset 3: mixed_long_1
**Status**: PENDING - Manual test required
- Expected: Mix of city and highway patterns
- Expected: Variable speed ranges
- Expected: Longest duration session

---

## Manual Testing Instructions

1. Open http://localhost:1420/
2. Click "Start Engine" button
3. Open PlaybackControls dropdown (should show green "REAL DATA PLAYBACK" badge)
4. Select "Highway Driving - High Speed" dataset
5. Observe:
   - Gauge values update smoothly
   - Car animation responds (wheels spin, colors change)
   - No console errors
6. Try speed control: 0.5x → 2x → 10x
7. Repeat for "Extended Mixed Driving" dataset

---

## Expected Behavior

### Successful Playback
- ✅ Dataset metadata shows in dropdown (vehicle, duration, samples)
- ✅ Telemetry values are realistic for driving conditions
- ✅ No gaps or NaN values in charts
- ✅ Car animation syncs with RPM/speed values
- ✅ Playback loops seamlessly at dataset end
- ✅ Speed multiplier affects animation smoothness

### Error Conditions to Check
- ❌ CSV parsing errors (check backend logs)
- ❌ Missing PID mappings
- ❌ NaN or infinite values in gauges
- ❌ Car animation frozen or stuttering
- ❌ Dataset not found errors

---

## Automated Backend Test

To verify dataset loading in backend:

```bash
cd src-tauri
cargo test test_dataset_loading -- --nocapture
```

Expected output: All 3 datasets load without errors.

---

## Next Steps After Testing

1. If all tests pass → Mark T048 as complete ✅
2. If errors found → Add to error handling task (T052)
3. Document any edge cases discovered
4. Update README with dataset info

