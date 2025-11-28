// Results display component with agent result cards

import React, { useEffect, useRef } from 'react';
import { AgentAnalysisResult } from '../../types/analysis';
import { AgentResultCard } from './AgentResultCard';
import { AlertCircle, Inbox, Loader2, X } from 'lucide-react';

interface ResultsDisplayProps {
  results: AgentAnalysisResult[];
  loading: boolean;
  error: string | null;
  onClearResults: () => void;
  className?: string;
}

export const ResultsDisplay: React.FC<ResultsDisplayProps> = ({
  results,
  loading,
  error,
  onClearResults,
  className = '',
}) => {
  const resultsRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to results when they arrive
  useEffect(() => {
    if (results.length > 0 && resultsRef.current) {
      resultsRef.current.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  }, [results.length]);

  // Loading state
  if (loading) {
    return (
      <div ref={resultsRef} className={`flex flex-col items-center justify-center py-12 ${className}`}>
        <Loader2 className="text-indigo-600 mb-4 animate-spin" size={48} />
        <p className="text-gray-600 dark:text-gray-400">Analyzing your data...</p>
      </div>
    );
  }

  // Error state
  if (error) {
    return (
      <div ref={resultsRef} className={className}>
        <div className="flex items-start gap-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
          <AlertCircle className="text-red-600 dark:text-red-400 mt-0.5" size={20} />
          <div className="flex-1">
            <h3 className="font-semibold text-red-900 dark:text-red-100 mb-1">
              Analysis Failed
            </h3>
            <p className="text-sm text-red-700 dark:text-red-300">{error}</p>
          </div>
          <button
            onClick={onClearResults}
            className="text-red-600 dark:text-red-400 hover:text-red-800 dark:hover:text-red-200"
            aria-label="Dismiss error"
          >
            <X size={20} />
          </button>
        </div>
      </div>
    );
  }

  // Empty state
  if (results.length === 0) {
    return (
      <div className={`flex flex-col items-center justify-center py-12 text-gray-500 dark:text-gray-400 ${className}`}>
        <Inbox size={48} className="mb-4" />
        <p>No analysis results yet</p>
        <p className="text-sm">Select a session and agents, then click "Analyze"</p>
      </div>
    );
  }

  // Success state - display results
  return (
    <div ref={resultsRef} className={className}>
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
          Analysis Results ({results.length} agent{results.length !== 1 ? 's' : ''})
        </h3>
        <button
          onClick={onClearResults}
          className="flex items-center gap-2 px-3 py-2 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-900 dark:text-white rounded-md transition-colors"
          aria-label="Clear results"
        >
          <X size={16} />
          Clear Results
        </button>
      </div>

      <div className="space-y-4">
        {results.map((result) => (
          <AgentResultCard
            key={result.agentId}
            result={result}
            defaultExpanded={true}
          />
        ))}
      </div>
    </div>
  );
};
