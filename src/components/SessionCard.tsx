import { useState } from 'react';
import { Session } from '../types/session';
import { Pencil, Trash2, Calendar, Clock, Car, Activity } from 'lucide-react';

interface SessionCardProps {
  session: Session;
  onSelect: () => void;
  onDelete: () => void;
  onUpdateName: (name: string) => Promise<void>;
}

export default function SessionCard({
  session,
  onSelect,
  onDelete,
  onUpdateName,
}: SessionCardProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [editedName, setEditedName] = useState(session.session_name || '');
  const [isSaving, setIsSaving] = useState(false);

  const handleSave = async () => {
    if (editedName.trim() === session.session_name) {
      setIsEditing(false);
      return;
    }

    try {
      setIsSaving(true);
      await onUpdateName(editedName.trim());
      setIsEditing(false);
    } catch (err) {
      // Revert on error
      setEditedName(session.session_name || '');
    } finally {
      setIsSaving(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSave();
    } else if (e.key === 'Escape') {
      setEditedName(session.session_name || '');
      setIsEditing(false);
    }
  };

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    });
  };

  const formatTime = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const formatDuration = (seconds?: number) => {
    if (!seconds) return 'Unknown';
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}m ${secs}s`;
  };

  const getSourceBadgeColor = (source?: string) => {
    switch (source) {
      case 'playback':
        return 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300';
      case 'random_walk':
        return 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-300';
      case 'imported':
        return 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300';
      case 'live':
        return 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300';
      default:
        return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300';
    }
  };

  return (
    <div
      className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 hover:border-blue-500 dark:hover:border-blue-500 transition-colors cursor-pointer group"
      onClick={(e) => {
        // Don't navigate if clicking edit/delete buttons or input
        if ((e.target as HTMLElement).closest('button, input')) return;
        onSelect();
      }}
    >
      <div className="p-4">
        {/* Header with name and actions */}
        <div className="flex items-start justify-between mb-3">
          <div className="flex-1 min-w-0">
            {isEditing ? (
              <input
                type="text"
                value={editedName}
                onChange={(e) => setEditedName(e.target.value)}
                onBlur={handleSave}
                onKeyDown={handleKeyDown}
                disabled={isSaving}
                className="w-full px-2 py-1 text-sm font-semibold text-gray-900 dark:text-white bg-gray-50 dark:bg-gray-700 border border-blue-500 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                autoFocus
                onClick={(e) => e.stopPropagation()}
              />
            ) : (
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white truncate">
                {session.session_name || 'Unnamed Session'}
              </h3>
            )}
          </div>

          <div className="flex items-center gap-1 ml-2">
            <button
              onClick={(e) => {
                e.stopPropagation();
                setIsEditing(true);
              }}
              className="p-1.5 text-gray-400 hover:text-blue-600 dark:hover:text-blue-400 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              title="Edit name"
            >
              <Pencil className="w-4 h-4" />
            </button>
            <button
              onClick={(e) => {
                e.stopPropagation();
                onDelete();
              }}
              className="p-1.5 text-gray-400 hover:text-red-600 dark:hover:text-red-400 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              title="Delete session"
            >
              <Trash2 className="w-4 h-4" />
            </button>
          </div>
        </div>

        {/* Metadata */}
        <div className="space-y-2 text-sm text-gray-600 dark:text-gray-400">
          <div className="flex items-center gap-2">
            <Calendar className="w-4 h-4" />
            <span>{formatDate(session.started_at)}</span>
            <Clock className="w-4 h-4 ml-2" />
            <span>{formatTime(session.started_at)}</span>
          </div>

          <div className="flex items-center gap-2">
            <Activity className="w-4 h-4" />
            <span>Duration: {formatDuration(session.duration_seconds)}</span>
          </div>

          {(session.vehicle_make || session.vehicle_model) && (
            <div className="flex items-center gap-2">
              <Car className="w-4 h-4" />
              <span>
                {session.vehicle_make} {session.vehicle_model}
                {session.vehicle_year && ` (${session.vehicle_year})`}
              </span>
            </div>
          )}

          {session.dataset_source && (
            <div className="flex items-center gap-2 mt-3">
              <span
                className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getSourceBadgeColor(
                  session.dataset_source
                )}`}
              >
                {session.dataset_source}
              </span>
            </div>
          )}
        </div>
      </div>

      {/* Hover overlay */}
      <div className="px-4 py-2 bg-gray-50 dark:bg-gray-700/50 border-t border-gray-200 dark:border-gray-600 opacity-0 group-hover:opacity-100 transition-opacity">
        <p className="text-xs text-gray-500 dark:text-gray-400 text-center">
          Click to view details →
        </p>
      </div>
    </div>
  );
}
