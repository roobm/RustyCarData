# Project Cleanup - Phase 0 Summary

## Research Completed ✅

I've completed comprehensive research on modern best practices and assessed the current codebase. Here's what I found:

## Key Findings

### 1. Compiler Warnings (46 Rust + 2 TypeScript)

**Root Cause**: Entire AI agents infrastructure (~2000+ lines) was built but never integrated into the application.

**Quick wins:**
- Remove agents module → eliminates ~30 warnings
- Remove unused frontend code (AgentDashboard, agentStore, agentService)
- Fix 2 TypeScript unused imports

### 2. Unused Code Identified

**Backend (Rust):**
- `src-tauri/src/agents/` - Complete AI analysis system (fuel coach, safety advisor, diagnostics)
- `src-tauri/src/config.rs` - Config module never constructed
- Agent commands in `commands.rs` and `lib.rs`

**Frontend (TypeScript):**
- `src/pages/AgentDashboard.tsx` - Page not routed
- `src/store/agentStore.ts` - Only used by AgentDashboard
- `src/services/agentService.ts` - Only used by agentStore

### 3. Missing Infrastructure

**Testing:**
- ❌ No Rust unit tests
- ❌ No frontend testing setup
- ❌ No test coverage tracking

**Project Structure:**
- ❌ No centralized `/types` directory
- ❌ No dedicated `/hooks` directory
- ❌ Types scattered across multiple files

## Recommended Cleanup Plan

### Phase 1: Quick Wins (Clear Warnings)

1. **Remove unused agents code**
   - Delete `src-tauri/src/agents/` directory
   - Remove agent commands from `commands.rs`
   - Remove agent command registration from `lib.rs`
   - Delete frontend: `AgentDashboard.tsx`, `agentStore.ts`, `agentService.ts`
   - **Impact**: ~40 warnings cleared, ~2500 lines removed

2. **Fix TypeScript warnings**
   - Remove unused `Play` import from `PlaybackControls.tsx`
   - Remove unused `get` parameter from `agentStore.ts` (or delete entire file)
   - **Impact**: 2 warnings cleared

3. **Remove config.rs if unused**
   - Verify if needed, likely can be removed
   - **Impact**: ~3-4 warnings cleared

**Total Quick Wins**: Zero warnings, cleaner codebase

### Phase 2: Testing Infrastructure (Your Priority)

Per your request: *"Make sure we have unit tests for everything. Start with that so we can use them later when we are refactoring."*

#### Backend Testing Setup

1. **Add test dependencies** to `Cargo.toml`:
   ```toml
   [dev-dependencies]
   mockall = "0.12"
   tokio-test = "0.4"
   ```

2. **Add unit tests** for key modules:
   - `simulation/state.rs` - State management tests
   - `simulation/playback.rs` - Playback logic tests
   - `storage/supabase.rs` - Storage operations (with mocks)

#### Frontend Testing Setup

1. **Install Vitest + React Testing Library**:
   ```bash
   npm install -D vitest jsdom @testing-library/react @testing-library/dom @testing-library/jest-dom @testing-library/user-event
   ```

2. **Configure Vitest** (`vite.config.ts`, `vitest.setup.ts`)

3. **Add component tests**:
   - `Gauge.test.tsx`
   - `TelemetryChart.test.tsx`
   - `SessionNameDialog.test.tsx`
   - `useStore.test.ts`

### Phase 3: Project Structure Improvements

1. **Create `/types` directory** - Move all type definitions
2. **Create `/hooks` directory** - Extract custom hooks
3. **Organize better** - Follow feature-based structure from research

### Phase 4: Documentation & Polish

1. Add doc comments to public Rust APIs
2. Update README with new structure
3. Add CONTRIBUTING guide with testing requirements

## Decision Needed

**Before I proceed with removal**, please confirm:

1. **Is the AI agents/analysis feature needed?**
   - If YES → We should finish integrating it instead of removing
   - If NO → I'll remove it to clean up warnings

2. **Should I start with removing unused code OR setting up tests first?**
   - Option A: Clean first (less code to test)
   - Option B: Test existing code first (safer but includes dead code)

3. **Any other modules to preserve or remove?**

## Next Steps

Once you confirm the approach, I'll:
1. Execute the cleanup (remove agents or keep)
2. Set up testing infrastructure
3. Write initial test suite
4. Clear all compiler warnings
5. Commit changes in logical chunks

All planning documents are ready in [specs/012-project-cleanup/](specs/012-project-cleanup/)
