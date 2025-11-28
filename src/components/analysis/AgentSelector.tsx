// Agent selector component with multi-select checkboxes

import React, { useEffect, useState } from 'react';
import { AgentCard } from '../AgentCard';
import { useAgentStore } from '../../store/agentStore';
import { AlertCircle, RefreshCw, CheckSquare, Square } from 'lucide-react';

interface AgentSelectorProps {
  selectedIds: Set<string>;
  onToggle: (agentId: string) => void;
  onSelectAll: () => void;
  onDeselectAll: () => void;
  className?: string;
}

export const AgentSelector: React.FC<AgentSelectorProps> = ({
  selectedIds,
  onToggle,
  onSelectAll,
  onDeselectAll,
  className = '',
}) => {
  const { agents, fetchAgents, loadingAgents, agentsError } = useAgentStore();
  const [hasLoaded, setHasLoaded] = useState(false);

  useEffect(() => {
    if (!hasLoaded) {
      fetchAgents();
      setHasLoaded(true);
    }
  }, [hasLoaded, fetchAgents]);

  const allSelected = agents.length > 0 && selectedIds.size === agents.length;

  const handleToggleAll = () => {
    if (allSelected) {
      onDeselectAll();
    } else {
      onSelectAll();
    }
  };

  // Loading state
  if (loadingAgents) {
    return (
      <div className={className}>
        <div className="mb-4 flex items-center justify-between">
          <div className="h-6 bg-gray-300 dark:bg-gray-600 rounded w-32 animate-pulse"></div>
          <div className="h-10 bg-gray-300 dark:bg-gray-600 rounded w-28 animate-pulse"></div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {[1, 2, 3].map((i) => (
            <div
              key={i}
              className="p-4 rounded-lg border-2 border-gray-300 dark:border-gray-600 animate-pulse"
            >
              <div className="h-6 bg-gray-300 dark:bg-gray-600 rounded mb-2"></div>
              <div className="space-y-2">
                <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
                <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  // Error state
  if (agentsError) {
    return (
      <div className={className}>
        <div className="flex items-start gap-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
          <AlertCircle className="text-red-600 dark:text-red-400 mt-0.5" size={20} />
          <div className="flex-1">
            <h3 className="font-semibold text-red-900 dark:text-red-100 mb-1">
              Failed to Load Agents
            </h3>
            <p className="text-sm text-red-700 dark:text-red-300">{agentsError}</p>
          </div>
          <button
            onClick={() => fetchAgents()}
            className="flex items-center gap-2 px-3 py-2 bg-red-100 dark:bg-red-800 hover:bg-red-200 dark:hover:bg-red-700 text-red-900 dark:text-red-100 rounded-md transition-colors"
            aria-label="Retry loading agents"
          >
            <RefreshCw size={16} />
            Retry
          </button>
        </div>
      </div>
    );
  }

  // Success state
  return (
    <div className={className}>
      <div className="mb-4 flex items-center justify-between">
        <div className="text-sm text-gray-600 dark:text-gray-400">
          {selectedIds.size} of {agents.length} agent{agents.length !== 1 ? 's' : ''} selected
        </div>
        <button
          onClick={handleToggleAll}
          className="flex items-center gap-2 px-4 py-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-900 dark:text-white rounded-md transition-colors"
          aria-label={allSelected ? 'Deselect all agents' : 'Select all agents'}
        >
          {allSelected ? (
            <>
              <Square size={16} />
              Deselect All
            </>
          ) : (
            <>
              <CheckSquare size={16} />
              Select All
            </>
          )}
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {agents.map((agent) => (
          <AgentCard
            key={agent.id}
            agent={agent}
            selected={selectedIds.has(agent.id)}
            onSelect={onToggle}
            showCheckbox={true}
          />
        ))}
      </div>
    </div>
  );
};
