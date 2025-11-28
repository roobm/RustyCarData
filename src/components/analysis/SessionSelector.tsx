// Session selector component with grid layout

import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Session, SessionCard as SessionCardType } from '../../types/analysis';
import { SessionCard } from './SessionCard';
import { transformSessionToCard } from '../../utils/sessionTransform';
import { AlertCircle, RefreshCw, Download } from 'lucide-react';

interface SessionSelectorProps {
  selectedId: string | null;
  onSelect: (sessionId: string) => void;
  className?: string;
}

export const SessionSelector: React.FC<SessionSelectorProps> = ({
  selectedId,
  onSelect,
  className = '',
}) => {
  const [sessions, setSessions] = useState<SessionCardType[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchSessions = async () => {
    setLoading(true);
    setError(null);

    try {
      const rawSessions = await invoke<Session[]>('get_supabase_sessions');
      const transformedSessions = rawSessions.map(transformSessionToCard);
      setSessions(transformedSessions);
    } catch (err) {
      console.error('Failed to fetch sessions:', err);
      setError(
        err instanceof Error
          ? err.message
          : 'Failed to load sessions. Please try again.'
      );
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchSessions();
  }, []);

  // Loading state - skeleton cards
  if (loading) {
    return (
      <div className={`grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 ${className}`}>
        {[1, 2, 3].map((i) => (
          <div
            key={i}
            className="p-4 rounded-lg border-2 border-gray-300 dark:border-gray-600 animate-pulse"
          >
            <div className="h-6 bg-gray-300 dark:bg-gray-600 rounded mb-2"></div>
            <div className="space-y-2">
              <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
              <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
              <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-2/3"></div>
            </div>
          </div>
        ))}
      </div>
    );
  }

  // Error state - error banner with retry
  if (error) {
    return (
      <div className={`${className}`}>
        <div className="flex items-start gap-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
          <AlertCircle className="text-red-600 dark:text-red-400 mt-0.5" size={20} />
          <div className="flex-1">
            <h3 className="font-semibold text-red-900 dark:text-red-100 mb-1">
              Failed to Load Sessions
            </h3>
            <p className="text-sm text-red-700 dark:text-red-300">{error}</p>
          </div>
          <button
            onClick={fetchSessions}
            className="flex items-center gap-2 px-3 py-2 bg-red-100 dark:bg-red-800 hover:bg-red-200 dark:hover:bg-red-700 text-red-900 dark:text-red-100 rounded-md transition-colors"
            aria-label="Retry loading sessions"
          >
            <RefreshCw size={16} />
            Retry
          </button>
        </div>
      </div>
    );
  }

  // Empty state - no sessions found
  if (sessions.length === 0) {
    return (
      <div className={`${className}`}>
        <div className="flex flex-col items-center justify-center p-12 bg-gray-50 dark:bg-gray-800 border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg">
          <Download className="text-gray-400 mb-4" size={48} />
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">
            No Sessions Found
          </h3>
          <p className="text-sm text-gray-600 dark:text-gray-400 text-center max-w-md">
            You haven't recorded any driving sessions yet. Start a new session from the Dashboard to begin analyzing your driving data.
          </p>
        </div>
      </div>
    );
  }

  // Success state - grid of session cards
  return (
    <div className={`grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 ${className}`}>
      {sessions.map((session) => (
        <SessionCard
          key={session.id}
          session={session}
          selected={selectedId === session.id}
          onClick={() => onSelect(session.id)}
        />
      ))}
    </div>
  );
};
