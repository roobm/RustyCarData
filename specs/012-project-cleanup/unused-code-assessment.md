# Unused Code Assessment

## Summary
This document identifies files, modules, and functionality that appear to be unused in the current application.

## Unused/Unreachable Frontend Code

### Pages
- **src/pages/AgentDashboard.tsx** - Page exists but not included in App.tsx routing
  - Impact: ~300+ lines of dead code
  - Decision: Remove unless there are plans to use it

### Stores
- **src/store/agentStore.ts** - Only imported by AgentDashboard.tsx (which is unused)
  - Has unused variable warning: `get` parameter never used
  - Impact: Zustand store managing agent state
  - Decision: Remove with AgentDashboard

### Services
- **src/services/agentService.ts** - Only imported by agentStore.ts (which is unused)
  - Impact: API wrapper for agent commands
  - Decision: Remove with agentStore

## Partially Unused Backend Code

### Agents Module (Mostly Unused)
The entire `src-tauri/src/agents/` module appears to be **designed but not actively used**:

**Files in agents module:**
- `src-tauri/src/agents/mod.rs` - Module declaration
- `src-tauri/src/agents/cache.rs` - Agent response cache (ALL methods unused)
- `src-tauri/src/agents/types.rs` - Type definitions (MANY never constructed)
- `src-tauri/src/agents/registry.rs` - Agent registry
- `src-tauri/src/agents/error.rs` - Error types
- `src-tauri/src/agents/orchestrator.rs` - Orchestration logic (partially unused)
- `src-tauri/src/agents/traits.rs` - Agent trait definitions (methods unused)
- `src-tauri/src/agents/implementations/mod.rs`
- `src-tauri/src/agents/implementations/fuel_coach.rs` (never constructed)
- `src-tauri/src/agents/implementations/safety_advisor.rs` (never constructed)
- `src-tauri/src/agents/implementations/diagnostic.rs` (never constructed)
- `src-tauri/src/agents/implementations/mock_agent.rs` (never constructed)

**Commands registered but not used:**
- `list_agents` - Registered in lib.rs but frontend page not routed
- `analyze_session_with_agents` - Registered but no UI calls it
- `get_agent_info` - Registered but no UI calls it

**Analysis:**
- Backend infrastructure is complete
- Commands are registered
- Frontend components exist but are not routed
- This appears to be a **planned feature that was never integrated**

**Options:**
1. **Remove entirely** - Clean option if not needed
2. **Keep for future** - If there are plans to use it
3. **Finish integration** - Complete the feature

**Recommendation:** Remove for now. If needed later, can be restored from git history.

**Impact:** Would remove ~2000+ lines of code and ~30+ warnings

### Storage Module (Partially Unused)
Some components in storage may be obsolete:

- **src-tauri/src/storage/writer.rs** - ParquetWriter (removed in recent commit but file may still exist)
- **src-tauri/src/storage/manager.rs** - SessionManager (may be obsolete after Supabase migration)

**Check:**
```bash
grep -r "ParquetWriter\|SessionManager" src-tauri/src/*.rs
```

### Config Module
- **src-tauri/src/config.rs** - Entire `Config` struct never constructed
  - Associated functions all unused
  - May have been replaced by environment variables
  - Decision: Verify if needed, likely can be removed

## Other Potential Dead Code

### Binaries
- **src-tauri/src/bin/import_datasets.rs** - Utility for one-time import?
  - Decision: Keep if used for maintenance, document usage

- **src-tauri/src/bin/import_sessions.rs** - Utility for one-time import?
  - Decision: Keep if used for maintenance, document usage

### Test Files
- **src-tauri/src/storage/supabase_test.rs** - Test file not in `#[cfg(test)]` module
  - Decision: Move to proper test module or integrate into mod.rs tests

## Cleanup Priority

### High Priority (Remove to clear warnings)
1. Frontend Agent code (AgentDashboard, agentStore, agentService)
2. Backend Agent module (entire `agents/` directory)
3. Agent commands from commands.rs and lib.rs
4. Config module (config.rs)
5. TypeScript unused imports (2 warnings)

**Estimated Impact:** ~40+ warnings removed, ~2500+ lines of code removed

### Medium Priority (Verify then decide)
1. storage/writer.rs and storage/manager.rs
2. Models/types that are only used by agents

### Low Priority (Document and keep)
1. Binary utilities (import_datasets, import_sessions)
2. Test files (move to proper location)

## Verification Steps

Before removing any code, verify:

1. **Grep for usage:**
   ```bash
   rg "AgentDashboard|agentStore|agentService" src/
   rg "agents::" src-tauri/src/ --type rust
   ```

2. **Check git history:**
   ```bash
   git log --all --oneline -- src/pages/AgentDashboard.tsx
   git log --all --oneline -- src-tauri/src/agents/
   ```

3. **Verify imports:**
   ```bash
   cargo check 2>&1 | grep "warning:"
   ```

4. **Test after removal:**
   ```bash
   cargo check
   cargo test
   npm run build
   ```

## Decision Required

Before proceeding with cleanup, confirm with user:
- Is the Agent/AI analysis feature needed for future?
- Should it be removed or completed?
- Any other modules that should be preserved?
