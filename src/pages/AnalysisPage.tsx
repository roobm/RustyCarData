import React, { useState, useEffect } from 'react';
import { AiAssistant } from '../components/AiAssistant';
import { Upload, FileText } from 'lucide-react';
import { open } from '@tauri-apps/plugin-dialog';
import { api } from '../services/api';

interface TelemetryPoint {
  timestamp: number;
  pid: string;
  value: number;
  unit: string;
}

export const AnalysisPage: React.FC = () => {
  const [importing, setImporting] = useState(false);
  const [sessions, setSessions] = useState<string[]>([]);
  const [selectedSession, setSelectedSession] = useState<string | null>(null);
  const [sessionData, setSessionData] = useState<TelemetryPoint[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadSessions();
  }, []);

  const loadSessions = async () => {
    try {
      const list = await api.listSessions();
      setSessions(list);
    } catch (error) {
      console.error('Failed to load sessions:', error);
    }
  };

  const handleImport = async () => {
    try {
      setImporting(true);
      
      // Open file picker for CSV files
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'CSV',
          extensions: ['csv']
        }]
      });

      if (selected) {
        const filePath = await api.importFile(selected as string);
        alert(`Successfully imported data to: ${filePath}`);
        await loadSessions();
      }
    } catch (error) {
      console.error('Import failed:', error);
      alert('Failed to import file. Check console for details.');
    } finally {
      setImporting(false);
    }
  };

  const handleSelectSession = async (filename: string) => {
    try {
      setLoading(true);
      setSelectedSession(filename);
      const data = await api.readSession(filename);
      
      // Parse CSV data
      const lines = data.trim().split('\n');
      const parsed: TelemetryPoint[] = [];
      
      // Skip header row
      for (let i = 1; i < lines.length; i++) {
        const [timestamp, pid, value, unit] = lines[i].split(',');
        if (timestamp && pid && value && unit) {
          parsed.push({
            timestamp: parseInt(timestamp),
            pid: pid,
            value: parseFloat(value),
            unit: unit
          });
        }
      }
      
      setSessionData(parsed);
    } catch (error) {
      console.error('Failed to load session data:', error);
      alert('Failed to load session data');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="p-6 h-full flex flex-col">
      <header className="mb-8 flex justify-between items-start">
        <div>
          <h2 className="text-2xl font-bold text-white">AI Analysis</h2>
          <p className="text-gray-400">Get insights on your driving sessions powered by OpenAI.</p>
        </div>
        <button
          onClick={handleImport}
          disabled={importing}
          className="flex items-center gap-2 px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Upload size={18} />
          {importing ? 'Importing...' : 'Import CSV'}
        </button>
      </header>

      <div className="flex gap-6 flex-1 overflow-hidden">
        {/* Sessions List */}
        <div className="w-64 bg-gray-800 rounded-lg p-4 overflow-y-auto">
          <h3 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
            <FileText size={20} />
            Sessions
          </h3>
          {sessions.length === 0 ? (
            <p className="text-gray-400 text-sm">No sessions found</p>
          ) : (
            <div className="space-y-2">
              {sessions.map((session) => (
                <button
                  key={session}
                  onClick={() => handleSelectSession(session)}
                  className={`w-full text-left px-3 py-2 rounded text-sm transition-colors ${
                    selectedSession === session
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                  }`}
                >
                  {session}
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Data Table */}
        <div className="flex-1 bg-gray-800 rounded-lg p-4 overflow-hidden flex flex-col">
          {!selectedSession ? (
            <div className="flex items-center justify-center h-full text-gray-400">
              Select a session to view data
            </div>
          ) : loading ? (
            <div className="flex items-center justify-center h-full text-gray-400">
              Loading...
            </div>
          ) : (
            <>
              <h3 className="text-lg font-semibold text-white mb-4">
                {selectedSession} ({sessionData.length} points)
              </h3>
              <div className="overflow-auto flex-1">
                <table className="w-full text-sm">
                  <thead className="sticky top-0 bg-gray-700">
                    <tr>
                      <th className="text-left p-2 text-gray-300">Timestamp</th>
                      <th className="text-left p-2 text-gray-300">PID</th>
                      <th className="text-left p-2 text-gray-300">Value</th>
                      <th className="text-left p-2 text-gray-300">Unit</th>
                    </tr>
                  </thead>
                  <tbody>
                    {sessionData.slice(0, 1000).map((point, idx) => (
                      <tr key={idx} className="border-t border-gray-700 hover:bg-gray-700/50">
                        <td className="p-2 text-gray-300">
                          {new Date(point.timestamp / 1000).toLocaleString()}
                        </td>
                        <td className="p-2 text-gray-300">{point.pid}</td>
                        <td className="p-2 text-gray-300">{point.value.toFixed(2)}</td>
                        <td className="p-2 text-gray-300">{point.unit}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
                {sessionData.length > 1000 && (
                  <p className="text-gray-400 text-center py-4">
                    Showing first 1000 of {sessionData.length} points
                  </p>
                )}
              </div>
            </>
          )}
        </div>
      </div>

      <div className="mt-6 max-w-4xl mx-auto w-full">
        <AiAssistant />
      </div>
    </div>
  );
};
