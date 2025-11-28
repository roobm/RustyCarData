import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { ArrowLeft, Calendar, Clock, Activity, Gauge } from 'lucide-react';
import { SessionData } from '../types/session';
import TelemetryCharts from '../components/TelemetryCharts';

export default function SessionDetailPage() {
  const { sessionId } = useParams<{ sessionId: string }>();
  const navigate = useNavigate();
  const [sessionData, setSessionData] = useState<SessionData | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!sessionId) {
      setError('No session ID provided');
      setIsLoading(false);
      return;
    }

    loadSessionData();
  }, [sessionId]);

  const loadSessionData = async () => {
    if (!sessionId) return;

    try {
      setIsLoading(true);
      setError(null);
      const data = await invoke<SessionData>('get_supabase_session_data', { sessionId });
      setSessionData(data);
    } catch (err) {
      console.error('Failed to load session data:', err);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', {
      month: 'long',
      day: 'numeric',
      year: 'numeric',
    });
  };

  const formatTime = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  const formatDuration = (seconds?: number) => {
    if (!seconds) return 'Unknown';
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}m ${secs}s`;
  };

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-50 dark:bg-gray-900 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p className="text-gray-600 dark:text-gray-400">Loading session data...</p>
        </div>
      </div>
    );
  }

  if (error || !sessionData) {
    return (
      <div className="min-h-screen bg-gray-50 dark:bg-gray-900 p-8">
        <div className="max-w-2xl mx-auto">
          <button
            onClick={() => navigate('/history')}
            className="mb-4 flex items-center gap-2 text-blue-600 dark:text-blue-400 hover:underline"
          >
            <ArrowLeft className="w-4 h-4" />
            Back to History
          </button>
          <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-6">
            <h2 className="text-xl font-semibold text-red-800 dark:text-red-200 mb-2">
              Failed to Load Session
            </h2>
            <p className="text-red-700 dark:text-red-300">{error || 'Session not found'}</p>
            <button
              onClick={loadSessionData}
              className="mt-4 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
            >
              Try Again
            </button>
          </div>
        </div>
      </div>
    );
  }

  const { session, telemetry, stats } = sessionData;

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Header */}
      <div className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <button
            onClick={() => navigate('/history')}
            className="mb-4 flex items-center gap-2 text-gray-600 dark:text-gray-400 hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
          >
            <ArrowLeft className="w-4 h-4" />
            Back to History
          </button>

          <div className="flex items-start justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
                {session.session_name || 'Unnamed Session'}
              </h1>
              <div className="mt-2 flex flex-wrap items-center gap-4 text-sm text-gray-600 dark:text-gray-400">
                <div className="flex items-center gap-1.5">
                  <Calendar className="w-4 h-4" />
                  <span>{formatDate(session.started_at)}</span>
                </div>
                <div className="flex items-center gap-1.5">
                  <Clock className="w-4 h-4" />
                  <span>{formatTime(session.started_at)}</span>
                </div>
                <div className="flex items-center gap-1.5">
                  <Activity className="w-4 h-4" />
                  <span>{formatDuration(session.duration_seconds)}</span>
                </div>
                {telemetry.length > 0 && (
                  <div className="flex items-center gap-1.5">
                    <Gauge className="w-4 h-4" />
                    <span>{telemetry.length.toLocaleString()} data points</span>
                  </div>
                )}
              </div>
            </div>

            {stats && (
              <div className="flex gap-4 text-sm">
                {stats.max_speed !== undefined && (
                  <div className="text-center">
                    <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
                      {stats.max_speed.toFixed(0)}
                    </div>
                    <div className="text-gray-600 dark:text-gray-400">Max Speed (km/h)</div>
                  </div>
                )}
                {stats.max_rpm !== undefined && (
                  <div className="text-center">
                    <div className="text-2xl font-bold text-purple-600 dark:text-purple-400">
                      {stats.max_rpm.toFixed(0)}
                    </div>
                    <div className="text-gray-600 dark:text-gray-400">Max RPM</div>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Telemetry Charts */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {telemetry.length === 0 ? (
          <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-12 text-center">
            <Activity className="w-16 h-16 text-gray-400 mx-auto mb-4" />
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-2">
              No Telemetry Data
            </h3>
            <p className="text-gray-600 dark:text-gray-400">
              This session doesn't contain any telemetry data points.
            </p>
          </div>
        ) : (
          <TelemetryCharts
            telemetry={telemetry}
            sessionName={session.session_name || 'Unnamed Session'}
          />
        )}
      </div>
    </div>
  );
}
