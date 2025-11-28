// Validation utilities for analysis workflow

export interface ValidationResult {
  valid: boolean;
  errors: string[];
}

/**
 * Validate analysis request before submission
 */
export function validateAnalysisRequest(
  sessionId: string | null,
  agentIds: Set<string>
): ValidationResult {
  const errors: string[] = [];

  if (!sessionId) {
    errors.push('Please select a session to analyze');
  }

  if (agentIds.size === 0) {
    errors.push('Please select at least one AI agent');
  }

  return {
    valid: errors.length === 0,
    errors,
  };
}
