// Unified AI Analysis Page - combines session selection, agent selection, and results

import React, { useState, useEffect } from 'react';
import { useAnalysisStore } from '../store/analysisStore';
import { useAgentStore } from '../store/agentStore';
import { CollapsibleSection } from '../components/analysis/CollapsibleSection';
import { SessionSelector } from '../components/analysis/SessionSelector';
import { AgentSelector } from '../components/analysis/AgentSelector';
import { AnalyzeButton } from '../components/analysis/AnalyzeButton';
import { ResultsDisplay } from '../components/analysis/ResultsDisplay';
import { Database, BrainCircuit, BarChart3 } from 'lucide-react';

export const UnifiedAnalysisPage: React.FC = () => {
  const {
    selectedSessionId,
    selectedAgentIds,
    isAnalyzing,
    currentResults,
    error,
    setSelectedSession,
    toggleAgent,
    selectAllAgents,
    deselectAllAgents,
    analyze,
    clearResults,
  } = useAnalysisStore();

  const { agents } = useAgentStore();

  // Section expand/collapse state
  const [expanded, setExpanded] = useState({
    sessions: true,
    agents: true,
    results: false,
  });

  // Auto-expand results after analysis completes
  useEffect(() => {
    if (currentResults && currentResults.length > 0) {
      setExpanded((prev) => ({
        ...prev,
        results: true,
        sessions: false,
        agents: false,
      }));
    }
  }, [currentResults]);

  // Check if we can analyze
  const canAnalyze = selectedSessionId !== null && selectedAgentIds.size > 0;

  const handleSelectAllAgents = () => {
    const allAgentIds = agents.map((agent) => agent.id);
    selectAllAgents(allAgentIds);
  };

  return (
    <div className="max-w-7xl mx-auto p-6 space-y-6">
      {/* Page Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          AI Analysis
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Select a driving session and AI agents to analyze your data
        </p>
      </div>

      {/* Session Selection */}
      <CollapsibleSection
        title="Select Session"
        subtitle="Choose a driving session to analyze"
        icon={<Database size={20} />}
        expanded={expanded.sessions}
        onToggle={(e) => setExpanded({ ...expanded, sessions: e })}
      >
        <SessionSelector
          selectedId={selectedSessionId}
          onSelect={setSelectedSession}
        />
      </CollapsibleSection>

      {/* Agent Selection */}
      <CollapsibleSection
        title="Choose AI Agents"
        subtitle="Select one or more agents to analyze your data"
        icon={<BrainCircuit size={20} />}
        expanded={expanded.agents}
        onToggle={(e) => setExpanded({ ...expanded, agents: e })}
      >
        <AgentSelector
          selectedIds={selectedAgentIds}
          onToggle={toggleAgent}
          onSelectAll={handleSelectAllAgents}
          onDeselectAll={deselectAllAgents}
        />
      </CollapsibleSection>

      {/* Analyze Button */}
      <div className="my-8">
        <AnalyzeButton
          disabled={!canAnalyze}
          loading={isAnalyzing}
          agentCount={selectedAgentIds.size}
          onClick={analyze}
        />
      </div>

      {/* Results Display */}
      <CollapsibleSection
        title="Analysis Results"
        subtitle={currentResults && currentResults.length > 0 ? `${currentResults.length} agent${currentResults.length !== 1 ? 's' : ''} analyzed` : 'Results will appear here'}
        icon={<BarChart3 size={20} />}
        expanded={expanded.results}
        onToggle={(e) => setExpanded({ ...expanded, results: e })}
      >
        <ResultsDisplay
          results={currentResults || []}
          loading={isAnalyzing}
          error={error}
          onClearResults={clearResults}
        />
      </CollapsibleSection>
    </div>
  );
};
