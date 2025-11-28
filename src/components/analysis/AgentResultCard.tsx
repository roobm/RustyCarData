// Agent result card component with collapsible sections

import React, { useState } from 'react';
import { AgentAnalysisResult } from '../../types/analysis';
import { ChevronDown, ChevronUp, AlertCircle, AlertTriangle, CheckCircle, Info, Star } from 'lucide-react';

interface AgentResultCardProps {
  result: AgentAnalysisResult;
  defaultExpanded?: boolean;
  onToggle?: (expanded: boolean) => void;
}

export const AgentResultCard: React.FC<AgentResultCardProps> = ({
  result,
  defaultExpanded = false,
  onToggle,
}) => {
  const [expanded, setExpanded] = useState(defaultExpanded);

  const handleToggle = () => {
    const newExpanded = !expanded;
    setExpanded(newExpanded);
    onToggle?.(newExpanded);
  };

  const getInsightIcon = (type: string) => {
    switch (type) {
      case 'critical':
        return <AlertCircle className="text-red-600" size={16} />;
      case 'warning':
        return <AlertTriangle className="text-yellow-600" size={16} />;
      case 'positive':
        return <CheckCircle className="text-green-600" size={16} />;
      default:
        return <Info className="text-blue-600" size={16} />;
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'high':
        return 'bg-red-100 text-red-900 dark:bg-red-900/20 dark:text-red-100';
      case 'medium':
        return 'bg-yellow-100 text-yellow-900 dark:bg-yellow-900/20 dark:text-yellow-100';
      default:
        return 'bg-gray-100 text-gray-900 dark:bg-gray-700 dark:text-gray-100';
    }
  };

  // Group insights by type
  const groupedInsights = {
    critical: result.insights.filter((i) => i.type === 'critical'),
    warning: result.insights.filter((i) => i.type === 'warning'),
    positive: result.insights.filter((i) => i.type === 'positive'),
    neutral: result.insights.filter((i) => i.type === 'neutral'),
  };

  // Sort recommendations by priority
  const sortedRecommendations = [...result.recommendations].sort((a, b) => {
    const priorityOrder = { high: 0, medium: 1, low: 2 };
    return priorityOrder[a.priority] - priorityOrder[b.priority];
  });

  // Error state
  if (result.error) {
    return (
      <div className="p-4 bg-red-50 dark:bg-red-900/20 border-2 border-red-200 dark:border-red-800 rounded-lg">
        <div className="flex items-start gap-3">
          <AlertCircle className="text-red-600 dark:text-red-400 mt-0.5" size={20} />
          <div>
            <h3 className="font-semibold text-red-900 dark:text-red-100 mb-1">
              {result.agentName} Analysis Failed
            </h3>
            <p className="text-sm text-red-700 dark:text-red-300">{result.error}</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="border-2 border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
      {/* Header */}
      <button
        onClick={handleToggle}
        className="w-full p-4 bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-750 transition-colors flex items-center justify-between"
        aria-expanded={expanded}
      >
        <div className="flex items-center gap-3">
          <h3 className="font-semibold text-lg text-gray-900 dark:text-white">
            {result.agentName}
          </h3>
          {result.score !== undefined && (
            <div className="flex items-center gap-1 text-sm text-gray-600 dark:text-gray-400">
              <Star size={16} className="text-yellow-500" />
              <span>{result.score}/100</span>
            </div>
          )}
        </div>
        <div className="flex items-center gap-3">
          <span className="text-sm text-gray-500 dark:text-gray-400">
            {new Date(result.analyzedAt).toLocaleTimeString()}
          </span>
          {expanded ? <ChevronUp size={20} /> : <ChevronDown size={20} />}
        </div>
      </button>

      {/* Collapsible Content */}
      {expanded && (
        <div className="p-4 bg-white dark:bg-gray-900 space-y-4">
          {/* Insights Section */}
          {result.insights.length > 0 && (
            <div>
              <h4 className="font-semibold text-gray-900 dark:text-white mb-3">
                Insights ({result.insights.length})
              </h4>
              <div className="space-y-2">
                {[...groupedInsights.critical, ...groupedInsights.warning, ...groupedInsights.positive, ...groupedInsights.neutral].map((insight) => (
                  <div
                    key={insight.id}
                    className="flex items-start gap-2 p-3 bg-gray-50 dark:bg-gray-800 rounded-md"
                  >
                    {getInsightIcon(insight.type)}
                    <div className="flex-1">
                      <h5 className="font-medium text-gray-900 dark:text-white text-sm">
                        {insight.title}
                      </h5>
                      <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">
                        {insight.description}
                      </p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Recommendations Section */}
          {result.recommendations.length > 0 && (
            <div>
              <h4 className="font-semibold text-gray-900 dark:text-white mb-3">
                Recommendations ({result.recommendations.length})
              </h4>
              <div className="space-y-2">
                {sortedRecommendations.map((rec) => (
                  <div
                    key={rec.id}
                    className="p-3 bg-gray-50 dark:bg-gray-800 rounded-md"
                  >
                    <div className="flex items-start justify-between gap-2 mb-2">
                      <h5 className="font-medium text-gray-900 dark:text-white text-sm">
                        {rec.action}
                      </h5>
                      <span className={`px-2 py-1 rounded text-xs font-medium ${getPriorityColor(rec.priority)}`}>
                        {rec.priority}
                      </span>
                    </div>
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                      {rec.rationale}
                    </p>
                    {rec.estimatedImpact && (
                      <p className="text-sm text-green-600 dark:text-green-400 mt-1">
                        Impact: {rec.estimatedImpact}
                      </p>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Empty State */}
          {result.insights.length === 0 && result.recommendations.length === 0 && (
            <p className="text-center text-gray-500 dark:text-gray-400 py-4">
              No insights or recommendations available
            </p>
          )}
        </div>
      )}
    </div>
  );
};
