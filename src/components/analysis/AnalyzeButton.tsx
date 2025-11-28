// Analyze button component with dynamic text and validation

import React from 'react';
import { PlayCircle, Loader2 } from 'lucide-react';

interface AnalyzeButtonProps {
  disabled: boolean;
  loading: boolean;
  agentCount: number;
  onClick: () => void;
  className?: string;
}

export const AnalyzeButton: React.FC<AnalyzeButtonProps> = ({
  disabled,
  loading,
  agentCount,
  onClick,
  className = '',
}) => {
  // Generate dynamic button text based on state
  const getButtonText = (): string => {
    if (loading) {
      return 'Analyzing...';
    }

    if (agentCount === 0) {
      return 'Select agents to analyze';
    }

    if (agentCount === 1) {
      return 'Analyze with 1 agent';
    }

    return `Analyze with ${agentCount} agents`;
  };

  // Get tooltip text explaining why button is disabled
  const getTooltipText = (): string | undefined => {
    if (!disabled) return undefined;

    if (agentCount === 0) {
      return 'Please select at least one AI agent';
    }

    return 'Please select a session and at least one agent';
  };

  const tooltipText = getTooltipText();

  return (
    <div className={`relative ${className}`}>
      <button
        onClick={onClick}
        disabled={disabled || loading}
        className={`
          w-full flex items-center justify-center gap-3 px-6 py-4 rounded-lg font-semibold text-lg
          transition-all duration-200
          ${
            disabled || loading
              ? 'bg-gray-300 dark:bg-gray-700 text-gray-500 dark:text-gray-400 cursor-not-allowed'
              : 'bg-indigo-600 hover:bg-indigo-700 text-white shadow-lg hover:shadow-xl hover:-translate-y-0.5'
          }
          focus:outline-none focus:ring-4 focus:ring-indigo-500 focus:ring-offset-2
        `}
        aria-label={getButtonText()}
        aria-disabled={disabled || loading}
        title={tooltipText}
      >
        {loading ? (
          <Loader2 size={24} className="animate-spin" />
        ) : (
          <PlayCircle size={24} />
        )}
        <span>{getButtonText()}</span>
      </button>

      {/* Tooltip for disabled state */}
      {disabled && !loading && tooltipText && (
        <div className="absolute left-1/2 -translate-x-1/2 -top-12 px-3 py-2 bg-gray-900 dark:bg-gray-700 text-white text-sm rounded-md opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap">
          {tooltipText}
          <div className="absolute left-1/2 -translate-x-1/2 top-full w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900 dark:border-t-gray-700"></div>
        </div>
      )}
    </div>
  );
};
