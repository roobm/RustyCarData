// Session selection card component

import React from 'react';
import { SessionCard as SessionCardType } from '../../types/analysis';
import { Calendar, Clock, Database } from 'lucide-react';

interface SessionCardProps {
  session: SessionCardType;
  selected: boolean;
  onClick: () => void;
  className?: string;
}

export const SessionCard: React.FC<SessionCardProps> = ({
  session,
  selected,
  onClick,
  className = '',
}) => {
  const handleKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      onClick();
    }
  };

  return (
    <div
      role="button"
      tabIndex={0}
      onClick={onClick}
      onKeyDown={handleKeyDown}
      className={`
        p-4 rounded-lg border-2 cursor-pointer transition-all
        hover:shadow-md hover:-translate-y-0.5
        focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2
        ${
          selected
            ? 'border-indigo-600 bg-indigo-50 dark:bg-indigo-900/20'
            : 'border-gray-300 dark:border-gray-600 hover:border-indigo-400'
        }
        ${className}
      `}
      aria-pressed={selected}
      aria-label={`Session ${session.name}, ${session.date} at ${session.time}`}
    >
      <h3 className="font-semibold text-lg mb-2 text-gray-900 dark:text-white">
        {session.name}
      </h3>

      <div className="space-y-1 text-sm text-gray-600 dark:text-gray-400">
        <div className="flex items-center gap-2">
          <Calendar size={14} />
          <span>{session.date}</span>
        </div>

        <div className="flex items-center gap-2">
          <Clock size={14} />
          <span>{session.time} • {session.duration}</span>
        </div>

        <div className="flex items-center gap-2">
          <Database size={14} />
          <span>{session.dataPoints} data points</span>
        </div>
      </div>
    </div>
  );
};
