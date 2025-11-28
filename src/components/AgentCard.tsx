import React from "react";
import type { AgentMetadata } from "../types/agents";
import "./AgentCard.css";

interface AgentCardProps {
  agent: AgentMetadata;
  selected?: boolean;
  onSelect?: (agentId: string) => void;
  showCheckbox?: boolean; // NEW: Enable checkbox mode
}

export const AgentCard: React.FC<AgentCardProps> = ({
  agent,
  selected = false,
  onSelect,
  showCheckbox = false,
}) => {
  const handleClick = () => {
    if (onSelect) {
      onSelect(agent.id);
    }
  };

  const getDomainColor = (domain: string): string => {
    if (typeof domain === "object" && "Custom" in domain) {
      return "#6b7280";
    }

    const colors: Record<string, string> = {
      FuelEfficiency: "#10b981",
      Safety: "#ef4444",
      Diagnostics: "#f59e0b",
      EcoRoute: "#14b8a6",
      Performance: "#8b5cf6",
      DriverProfile: "#3b82f6",
    };

    return colors[domain as string] || "#6b7280";
  };

  const getPrivacyBadge = (level: string): { icon: string; color: string } => {
    const badges: Record<string, { icon: string; color: string }> = {
      LocalOnly: { icon: "🔒", color: "#10b981" },
      CloudPreferred: { icon: "☁️", color: "#3b82f6" },
      CloudRequired: { icon: "☁️", color: "#ef4444" },
    };

    return badges[level] || { icon: "❓", color: "#6b7280" };
  };

  const privacy = getPrivacyBadge(agent.privacy_level);
  const domainColor = getDomainColor(agent.domain as string);

  return (
    <div
      className={`agent-card ${selected ? "selected" : ""}`}
      onClick={handleClick}
      role={onSelect ? "button" : undefined}
      tabIndex={onSelect ? 0 : undefined}
    >
      <div className="agent-card-header">
        {showCheckbox && (
          <input
            type="checkbox"
            checked={selected}
            onChange={handleClick}
            onClick={(e) => e.stopPropagation()}
            className="agent-checkbox"
            aria-label={`Select ${agent.name}`}
          />
        )}
        <div className="agent-icon">{agent.icon}</div>
        <div className="agent-info">
          <h3 className="agent-name">{agent.name}</h3>
          <span className="agent-version">v{agent.version}</span>
        </div>
      </div>

      <p className="agent-description">{agent.description}</p>

      <div className="agent-badges">
        <span className="badge badge-domain" style={{ borderColor: domainColor }}>
          {typeof agent.domain === "object" && "Custom" in agent.domain
            ? agent.domain.Custom
            : agent.domain}
        </span>
        <span className="badge badge-privacy" title={agent.privacy_level}>
          {privacy.icon} {agent.privacy_level}
        </span>
        <span className="badge badge-model">{agent.model_preference}</span>
      </div>

      {selected && <div className="agent-selected-indicator">✓ Selected</div>}
    </div>
  );
};
