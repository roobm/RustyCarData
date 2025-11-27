import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Database, Zap, Play, Gauge as GaugeIcon } from 'lucide-react';

interface Dataset {
  id: string;
  name: string;
  vehicle: string;
  style: string;
  duration: string;
  samples: number;
}

export const PlaybackControls: React.FC = () => {
  const [mode, setMode] = useState<string>('Playback');
  const [currentDataset, setCurrentDataset] = useState<string>('city_commute_1');
  const [playbackSpeed, setPlaybackSpeed] = useState<number>(1.0);
  const [datasets, setDatasets] = useState<Dataset[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);

  useEffect(() => {
    loadState();
  }, []);

  const loadState = async () => {
    try {
      setError(null);
      const [modeRes, datasetRes, speedRes, datasetsRes] = await Promise.all([
        invoke<string>('get_simulation_mode'),
        invoke<string | null>('get_current_dataset'),
        invoke<number>('get_playback_speed'),
        invoke<Dataset[]>('list_datasets'),
      ]);

      setMode(modeRes);
      if (datasetRes) setCurrentDataset(datasetRes);
      setPlaybackSpeed(speedRes);
      setDatasets(datasetsRes);
    } catch (error) {
      console.error('Failed to load playback state:', error);
      setError('Failed to load playback controls. Please refresh the page.');
    }
  };

  const handleSpeedChange = async (speed: number) => {
    try {
      await invoke('set_playback_speed', { speed });
      setPlaybackSpeed(speed);
    } catch (error) {
      console.error('Failed to set speed:', error);
    }
  };

  const handleDatasetChange = async (datasetId: string) => {
    try {
      setError(null);
      setIsLoading(true);
      await invoke('set_current_dataset', { datasetId });
      setCurrentDataset(datasetId);
      setIsLoading(false);
    } catch (error) {
      console.error('Failed to set dataset:', error);
      setError(`Failed to load dataset. The file may be missing or corrupted.`);
      setIsLoading(false);
      // Reset to previous dataset
      setTimeout(() => setError(null), 5000);
    }
  };

  const currentDatasetInfo = datasets.find(d => d.id === currentDataset);

  const getModeColor = () => {
    return mode === 'Playback' ? 'bg-green-900/30 border-green-500/50 text-green-400' : 'bg-yellow-900/30 border-yellow-500/50 text-yellow-400';
  };

  const getModeIcon = () => {
    return mode === 'Playback' ? <Database className="w-5 h-5" /> : <Zap className="w-5 h-5" />;
  };

  return (
    <div className="bg-gray-800/50 backdrop-blur-sm border border-gray-700 rounded-lg p-4 mb-6">
      {/* Error Message */}
      {error && (
        <div className="mb-3 p-3 bg-red-900/30 border border-red-500/50 rounded-lg text-red-400 text-sm flex items-center gap-2">
          <span className="text-lg">⚠️</span>
          <span>{error}</span>
        </div>
      )}

      <div className="flex flex-wrap items-center gap-4">
        {/* Mode Indicator */}
        <div className={`flex items-center gap-2 px-4 py-2 rounded-lg border ${getModeColor()}`}>
          {getModeIcon()}
          <span className="font-bold text-sm">
            {mode === 'Playback' ? ' REAL DATA PLAYBACK' : ' SIMULATION MODE'}
          </span>
        </div>

        {/* Dataset Info - Only show in Playback mode */}
        {mode === 'Playback' && currentDatasetInfo && (
          <div className="flex-1 min-w-[300px]">
            <div className="relative">
              <select
                value={currentDataset}
                onChange={(e) => handleDatasetChange(e.target.value)}
                disabled={isLoading}
                className="w-full bg-gray-900 border border-gray-600 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {datasets.map(dataset => (
                  <option key={dataset.id} value={dataset.id}>
                    {dataset.name} - {dataset.vehicle} ({dataset.duration}, {dataset.samples} samples)
                  </option>
                ))}
              </select>
              {isLoading && (
                <div className="absolute right-3 top-1/2 transform -translate-y-1/2">
                  <div className="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />
                </div>
              )}
            </div>
            <div className="mt-1 text-xs text-gray-400">
              {isLoading ? 'Loading dataset...' : `${currentDatasetInfo.style} • ${currentDatasetInfo.samples.toLocaleString()} data points`}
            </div>
          </div>
        )}

        {/* Speed Control - Only show in Playback mode */}
        {mode === 'Playback' && (
          <div className="flex items-center gap-2">
            <GaugeIcon className="w-4 h-4 text-gray-400" />
            <select
              value={playbackSpeed}
              onChange={(e) => handleSpeedChange(parseFloat(e.target.value))}
              disabled={isLoading}
              className="bg-gray-900 border border-gray-600 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value={0.5}>0.5x</option>
              <option value={1.0}>1.0x</option>
              <option value={2.0}>2.0x</option>
              <option value={5.0}>5.0x</option>
              <option value={10.0}>10.0x</option>
            </select>
          </div>
        )}
      </div>
    </div>
  );
};
