import { Session } from '../types/session';
import SessionCard from './SessionCard';

interface SessionListProps {
  sessions: Session[];
  onSelectSession: (sessionId: string) => void;
  onDeleteSession: (sessionId: string) => void;
  onUpdateName: (sessionId: string, name: string) => Promise<void>;
  isLoading: boolean;
}

export default function SessionList({
  sessions,
  onSelectSession,
  onDeleteSession,
  onUpdateName,
  isLoading,
}: SessionListProps) {
  if (isLoading) {
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {[1, 2, 3, 4, 5, 6].map((i) => (
          <div
            key={i}
            className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 animate-pulse"
          >
            <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4 mb-3"></div>
            <div className="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/2 mb-2"></div>
            <div className="h-3 bg-gray-200 dark:bg-gray-700 rounded w-2/3"></div>
          </div>
        ))}
      </div>
    );
  }

  // Sort sessions by created_at descending (newest first)
  const sortedSessions = [...sessions].sort((a, b) => {
    const dateA = new Date(a.created_at).getTime();
    const dateB = new Date(b.created_at).getTime();
    return dateB - dateA;
  });

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {sortedSessions.map((session) => (
        <SessionCard
          key={session.id}
          session={session}
          onSelect={() => onSelectSession(session.id!)}
          onDelete={() => onDeleteSession(session.id!)}
          onUpdateName={(name) => onUpdateName(session.id!, name)}
        />
      ))}
    </div>
  );
}
