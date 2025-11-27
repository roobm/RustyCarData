import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useNavigate } from 'react-router-dom';
import { Session } from '../types/session';
import SessionList from '../components/SessionList';

export default function SessionHistoryPage() {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    loadSessions();
  }, []);

  const loadSessions = async () => {
    try {
      setIsLoading(true);
      setError(null);
      const result = await invoke<Session[]>('list_supabase_sessions', { limit: 50 });
      setSessions(result);
    } catch (err) {
      console.error('Failed to load sessions:', err);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleSelectSession = (sessionId: string) => {
    navigate(`/history/${sessionId}`);
  };

  const handleDeleteSession = async (sessionId: string) => {
    if (!window.confirm('Are you sure you want to delete this session? This cannot be undone.')) {
      return;
    }

    try {
      await invoke('delete_supabase_session', { sessionId });
      // Remove from local state
      setSessions(sessions.filter(s => s.id !== sessionId));
    } catch (err) {
      console.error('Failed to delete session:', err);
      alert(`Failed to delete session: ${err}`);
    }
  };

  const handleUpdateName = async (sessionId: string, name: string) => {
    try {
      await invoke('update_supabase_session_name', { sessionId, name });
      // Update local state
      setSessions(sessions.map(s =>
        s.id === sessionId ? { ...s, session_name: name } : s
      ));
    } catch (err) {
      console.error('Failed to update session name:', err);
      alert(`Failed to update session name: ${err}`);
      throw err; // Re-throw so component can revert
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            Session History
          </h1>
          <p className="mt-2 text-gray-600 dark:text-gray-400">
            Browse and analyze your recorded driving sessions
          </p>
        </div>

        {error && (
          <div className="mb-6 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
            <p className="text-red-800 dark:text-red-200">
              <strong>Error:</strong> {error}
            </p>
            <button
              onClick={loadSessions}
              className="mt-2 text-sm text-red-600 dark:text-red-400 hover:underline"
            >
              Try again
            </button>
          </div>
        )}

        {isLoading ? (
          <div className="flex items-center justify-center py-12">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
          </div>
        ) : sessions.length === 0 ? (
          <div className="text-center py-12 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
            <svg
              className="mx-auto h-12 w-12 text-gray-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
              />
            </svg>
            <h3 className="mt-2 text-sm font-medium text-gray-900 dark:text-white">
              No sessions yet
            </h3>
            <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
              Start recording sessions to see them here.
            </p>
          </div>
        ) : (
          <SessionList
            sessions={sessions}
            onSelectSession={handleSelectSession}
            onDeleteSession={handleDeleteSession}
            onUpdateName={handleUpdateName}
            isLoading={false}
          />
        )}
      </div>
    </div>
  );
}
