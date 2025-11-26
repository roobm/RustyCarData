import React, { useEffect, useState } from "react";
import { useAgentStore } from "../store/agentStore";
import { AgentCard } from "../components/AgentCard";
import "./AgentDashboard.css";

export const AgentDashboard: React.FC = () => {
  const {
    agents,
    loadingAgents,
    agentsError,
    fetchAgents,
  } = useAgentStore();

  const [selectedAgents, setSelectedAgents] = useState<Set<string>>(new Set());

  useEffect(() => {
    fetchAgents();
  }, [fetchAgents]);

  const handleAgentSelect = (agentId: string) => {
    setSelectedAgents((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(agentId)) {
        newSet.delete(agentId);
      } else {
        newSet.add(agentId);
      }
      return newSet;
    });
  };

  const handleSelectAll = () => {
    if (selectedAgents.size === agents.length) {
      setSelectedAgents(new Set());
    } else {
      setSelectedAgents(new Set(agents.map((a) => a.id)));
    }
  };

  if (loadingAgents) {
    return (
      <div className="agent-dashboard">
        <div className="loading-state">
          <div className="spinner"></div>
          <p>Loading agents...</p>
        </div>
      </div>
    );
  }

  if (agentsError) {
    return (
      <div className="agent-dashboard">
        <div className="error-state">
          <p className="error-message">❌ {agentsError}</p>
          <button onClick={fetchAgents} className="btn-retry">
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="agent-dashboard">
      <div className="dashboard-header">
        <div>
          <h1>🤖 AI Agent Dashboard</h1>
          <p className="subtitle">
            Select agents to analyze your driving sessions
          </p>
        </div>
        <div className="header-actions">
          <button onClick={handleSelectAll} className="btn-select-all">
            {selectedAgents.size === agents.length ? "Deselect All" : "Select All"}
          </button>
          <span className="selection-count">
            {selectedAgents.size} of {agents.length} selected
          </span>
        </div>
      </div>

      <div className="agents-grid">
        {agents.map((agent) => (
          <AgentCard
            key={agent.id}
            agent={agent}
            selected={selectedAgents.has(agent.id)}
            onSelect={handleAgentSelect}
          />
        ))}
      </div>

      {agents.length === 0 && (
        <div className="empty-state">
          <p>No agents available</p>
        </div>
      )}

      <div className="dashboard-info">
        <div className="info-box">
          <h3>💡 How it works</h3>
          <ol>
            <li>Select one or more AI agents above</li>
            <li>Navigate to a driving session</li>
            <li>Click "Analyze" to get AI-powered insights</li>
            <li>View recommendations and action items</li>
          </ol>
        </div>

        <div className="info-box">
          <h3>🔒 Privacy</h3>
          <p>
            <strong>LocalOnly</strong> agents process data on your device only.
            <br />
            <strong>CloudPreferred</strong> agents can use cloud AI for better results.
            <br />
            <strong>CloudRequired</strong> agents need internet connection.
          </p>
        </div>
      </div>
    </div>
  );
};
