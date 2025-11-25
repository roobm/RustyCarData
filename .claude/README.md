# Claude Code - Speckit Commands

This directory contains slash commands for Claude Code to work with the Speckit workflow.

## Available Commands

### `/speckit-plan`
Execute the implementation planning workflow using the plan template to generate design artifacts.

**Usage:**
```
/speckit-plan
```

**What it does:**
- Runs setup scripts to get feature paths
- Loads feature spec and constitution
- Generates research.md with technical decisions
- Creates data-model.md with entities and relationships
- Generates API contracts in /contracts/
- Creates quickstart.md with usage examples

**Prerequisites:**
- Must be on a feature branch (e.g., `006-data-recording`)
- Feature directory with spec.md must exist

---

### `/speckit-tasks`
Generate an actionable, dependency-ordered tasks.md for the feature based on available design artifacts.

**Usage:**
```
/speckit-tasks
```

**What it does:**
- Loads plan.md and spec.md
- Extracts user stories with priorities
- Maps entities and API endpoints to user stories
- Generates tasks organized by user story
- Creates parallel execution opportunities
- Outputs tasks.md with checklist format

**Prerequisites:**
- plan.md must exist in feature directory
- spec.md with user stories must exist

---

### `/speckit-implement`
Execute the implementation plan by processing and executing all tasks defined in tasks.md.

**Usage:**
```
/speckit-implement
```

**What it does:**
- Checks checklist status (if checklists exist)
- Loads all design documents
- Verifies project setup (ignore files, etc.)
- Executes tasks phase-by-phase
- Respects dependencies and parallelization
- Marks completed tasks with [X]
- Reports progress and handles errors

**Prerequisites:**
- tasks.md must exist in feature directory
- plan.md must exist in feature directory

---

## Workflow

Typical Speckit workflow with Claude Code:

1. **Create feature spec** (manually or with another tool)
   - Create `specs/00X-feature-name/spec.md`
   - Define user stories with priorities

2. **Generate implementation plan**
   ```
   /speckit-plan
   ```

3. **Generate tasks**
   ```
   /speckit-tasks
   ```

4. **Implement features**
   ```
   /speckit-implement
   ```

---

## Configuration

### Scripts
The commands use bash scripts located in `.specify/scripts/bash/`:
- `common.sh` - Shared functions
- `check-prerequisites.sh` - Validate prerequisites
- `setup-plan.sh` - Setup planning workflow

These scripts work on macOS and Linux. For Windows, use the PowerShell equivalents in `.specify/scripts/powershell/`.

### Templates
Templates are located in `.specify/templates/`:
- `plan-template.md` - Implementation plan template
- `tasks-template.md` - Tasks checklist template

---

## Notes

- All file paths used are absolute paths
- Commands detect git repository and feature branch automatically
- Commands support non-git workflows by detecting latest feature directory
- Tasks use checklist format: `- [ ] [TaskID] [P?] [Story?] Description with file path`
