// Session data transformation utilities

import { Session } from '../types/session';
import { SessionCard } from '../types/analysis';

/**
 * Transform Session from Supabase to SessionCard for UI display
 */
export function transformSessionToCard(session: Session): SessionCard {
  const startDate = new Date(session.started_at);

  return {
    id: session.id,
    name: session.session_name || 'Unnamed Session',
    date: startDate.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    }),
    time: startDate.toLocaleTimeString('en-US', {
      hour: 'numeric',
      minute: '2-digit',
    }),
    duration: formatDuration(session.duration_seconds),
    dataPoints: 0, // TODO: Fetch from telemetry count
  };
}

/**
 * Format duration in seconds to human-readable string
 */
function formatDuration(seconds: number | undefined): string {
  if (!seconds) return 'Unknown';

  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;

  if (mins === 0) return `${secs}s`;
  if (secs === 0) return `${mins}m`;
  return `${mins}m ${secs}s`;
}
