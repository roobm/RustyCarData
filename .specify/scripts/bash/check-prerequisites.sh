#!/usr/bin/env bash

# Consolidated prerequisite checking script (Bash)
#
# This script provides unified prerequisite checking for Spec-Driven Development workflow.
#
# Usage: ./check-prerequisites.sh [OPTIONS]
#
# OPTIONS:
#   --json              Output in JSON format
#   --require-tasks     Require tasks.md to exist (for implementation phase)
#   --include-tasks     Include tasks.md in AVAILABLE_DOCS list
#   --paths-only        Only output path variables (no validation)
#   --help, -h          Show help message

set -euo pipefail

# Parse arguments
JSON=false
REQUIRE_TASKS=false
INCLUDE_TASKS=false
PATHS_ONLY=false

show_help() {
    cat <<EOF
Usage: check-prerequisites.sh [OPTIONS]

Consolidated prerequisite checking for Spec-Driven Development workflow.

OPTIONS:
  --json              Output in JSON format
  --require-tasks     Require tasks.md to exist (for implementation phase)
  --include-tasks     Include tasks.md in AVAILABLE_DOCS list
  --paths-only        Only output path variables (no prerequisite validation)
  --help, -h          Show this help message

EXAMPLES:
  # Check task prerequisites (plan.md required)
  ./check-prerequisites.sh --json

  # Check implementation prerequisites (plan.md + tasks.md required)
  ./check-prerequisites.sh --json --require-tasks --include-tasks

  # Get feature paths only (no validation)
  ./check-prerequisites.sh --paths-only

EOF
    exit 0
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --json) JSON=true; shift ;;
        --require-tasks) REQUIRE_TASKS=true; shift ;;
        --include-tasks) INCLUDE_TASKS=true; shift ;;
        --paths-only) PATHS_ONLY=true; shift ;;
        --help|-h) show_help ;;
        *) echo "Unknown option: $1" >&2; exit 1 ;;
    esac
done

# Source common functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

# Get feature paths
eval "$(get_feature_paths_env)"

# Validate branch
if ! test_feature_branch "$CURRENT_BRANCH" "$HAS_GIT"; then
    exit 1
fi

# If paths-only mode, output paths and exit
if $PATHS_ONLY; then
    if $JSON; then
        cat <<EOF
{"REPO_ROOT":"$REPO_ROOT","BRANCH":"$CURRENT_BRANCH","FEATURE_DIR":"$FEATURE_DIR","FEATURE_SPEC":"$FEATURE_SPEC","IMPL_PLAN":"$IMPL_PLAN","TASKS":"$TASKS"}
EOF
    else
        echo "REPO_ROOT: $REPO_ROOT"
        echo "BRANCH: $CURRENT_BRANCH"
        echo "FEATURE_DIR: $FEATURE_DIR"
        echo "FEATURE_SPEC: $FEATURE_SPEC"
        echo "IMPL_PLAN: $IMPL_PLAN"
        echo "TASKS: $TASKS"
    fi
    exit 0
fi

# Validate required directories and files
if [[ ! -d "$FEATURE_DIR" ]]; then
    echo "ERROR: Feature directory not found: $FEATURE_DIR" >&2
    echo "Run /speckit-specify first to create the feature structure." >&2
    exit 1
fi

if [[ ! -f "$IMPL_PLAN" ]]; then
    echo "ERROR: plan.md not found in $FEATURE_DIR" >&2
    echo "Run /speckit-plan first to create the implementation plan." >&2
    exit 1
fi

# Check for tasks.md if required
if $REQUIRE_TASKS && [[ ! -f "$TASKS" ]]; then
    echo "ERROR: tasks.md not found in $FEATURE_DIR" >&2
    echo "Run /speckit-tasks first to create the task list." >&2
    exit 1
fi

# Build list of available documents
DOCS=()

# Always check these optional docs
[[ -f "$RESEARCH" ]] && DOCS+=("research.md")
[[ -f "$DATA_MODEL" ]] && DOCS+=("data-model.md")

# Check contracts directory (only if it exists and has files)
if [[ -d "$CONTRACTS_DIR" ]] && [[ -n "$(find "$CONTRACTS_DIR" -maxdepth 1 -type f 2>/dev/null)" ]]; then
    DOCS+=("contracts/")
fi

[[ -f "$QUICKSTART" ]] && DOCS+=("quickstart.md")

# Include tasks.md if requested and it exists
if $INCLUDE_TASKS && [[ -f "$TASKS" ]]; then
    DOCS+=("tasks.md")
fi

# Output results
if $JSON; then
    # JSON output
    docs_json=$(printf '%s\n' "${DOCS[@]}" | jq -R . | jq -s .)
    cat <<EOF
{"FEATURE_DIR":"$FEATURE_DIR","AVAILABLE_DOCS":$docs_json}
EOF
else
    # Text output
    echo "FEATURE_DIR:$FEATURE_DIR"
    echo "AVAILABLE_DOCS:"

    # Show status of each potential document
    test_file_exists "$RESEARCH" 'research.md' || true
    test_file_exists "$DATA_MODEL" 'data-model.md' || true
    test_dir_has_files "$CONTRACTS_DIR" 'contracts/' || true
    test_file_exists "$QUICKSTART" 'quickstart.md' || true

    if $INCLUDE_TASKS; then
        test_file_exists "$TASKS" 'tasks.md' || true
    fi
fi
