#!/usr/bin/env bash

# Setup plan script (Bash)
#
# This script prepares the feature directory for planning
#
# Usage: ./setup-plan.sh [OPTIONS]
#
# OPTIONS:
#   --json              Output in JSON format
#   --help, -h          Show help message

set -euo pipefail

# Parse arguments
JSON=false

show_help() {
    cat <<EOF
Usage: setup-plan.sh [OPTIONS]

Setup the feature directory for planning workflow.

OPTIONS:
  --json              Output in JSON format
  --help, -h          Show this help message

EXAMPLES:
  # Setup plan and get paths in JSON
  ./setup-plan.sh --json

EOF
    exit 0
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --json) JSON=true; shift ;;
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

# Ensure feature directory exists
if [[ ! -d "$FEATURE_DIR" ]]; then
    echo "ERROR: Feature directory not found: $FEATURE_DIR" >&2
    echo "Run /speckit-specify first to create the feature structure." >&2
    exit 1
fi

# Ensure spec.md exists
if [[ ! -f "$FEATURE_SPEC" ]]; then
    echo "ERROR: spec.md not found in $FEATURE_DIR" >&2
    echo "Run /speckit-specify first to create the feature specification." >&2
    exit 1
fi

# Copy plan template if it doesn't exist
if [[ ! -f "$IMPL_PLAN" ]]; then
    TEMPLATE="$REPO_ROOT/.specify/templates/plan-template.md"
    if [[ -f "$TEMPLATE" ]]; then
        cp "$TEMPLATE" "$IMPL_PLAN"
        echo "Created plan.md from template" >&2
    else
        echo "Warning: Plan template not found at $TEMPLATE" >&2
        touch "$IMPL_PLAN"
    fi
fi

# Get specs directory (parent of feature dir)
SPECS_DIR="$(dirname "$FEATURE_DIR")"

# Output results
if $JSON; then
    cat <<EOF
{"REPO_ROOT":"$REPO_ROOT","BRANCH":"$CURRENT_BRANCH","SPECS_DIR":"$SPECS_DIR","FEATURE_DIR":"$FEATURE_DIR","FEATURE_SPEC":"$FEATURE_SPEC","IMPL_PLAN":"$IMPL_PLAN"}
EOF
else
    echo "REPO_ROOT: $REPO_ROOT"
    echo "BRANCH: $CURRENT_BRANCH"
    echo "SPECS_DIR: $SPECS_DIR"
    echo "FEATURE_DIR: $FEATURE_DIR"
    echo "FEATURE_SPEC: $FEATURE_SPEC"
    echo "IMPL_PLAN: $IMPL_PLAN"
fi
