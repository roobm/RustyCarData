import React, { useState } from 'react';
import { api } from '../services/api';
import { Bot, Sparkles, Loader2, Settings } from 'lucide-react';

export const AiAssistant: React.FC = () => {
  const [analysis, setAnalysis] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [apiKey, setApiKey] = useState('');
  const [showSettings, setShowSettings] = useState(false);

  const handleAnalyze = async () => {
    setLoading(true);
    try {
      // If no API key, simulate delay. If API key, the backend call will take time.
      if (!apiKey) await new Promise(resolve => setTimeout(resolve, 1500));
      
      const result = await api.analyzeCurrentSession(apiKey || undefined);
      setAnalysis(result);
    } catch (error) {
      console.error("Failed to analyze session", error);
      setAnalysis("Error: Failed to generate analysis. Please check console.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-gray-800 rounded-lg p-6 shadow-lg h-full flex flex-col relative">
      <div className="flex justify-between items-center mb-6">
        <h3 className="text-xl font-bold text-purple-400 flex items-center gap-2">
          <Bot size={24} />
          AI Co-Pilot
        </h3>
        <button 
          onClick={() => setShowSettings(!showSettings)}
          className="text-gray-500 hover:text-gray-300 transition-colors"
          title="AI Settings"
        >
          <Settings size={20} />
        </button>
      </div>

      {showSettings && (
        <div className="absolute top-16 right-6 z-10 bg-gray-700 p-4 rounded-lg shadow-xl border border-gray-600 w-64">
          <label className="block text-xs text-gray-400 mb-1">OpenAI API Key (Optional)</label>
          <input 
            type="password" 
            value={apiKey}
            onChange={(e) => setApiKey(e.target.value)}
            placeholder="sk-..."
            className="w-full bg-gray-900 border border-gray-600 rounded px-2 py-1 text-sm text-white focus:border-purple-500 outline-none"
          />
          <p className="text-[10px] text-gray-500 mt-2">
            Leave empty to use local mock analysis. Key is not saved persistently.
          </p>
        </div>
      )}

      <div className="flex-1 flex flex-col">
        {!analysis && !loading && (
          <div className="flex-1 flex flex-col items-center justify-center text-gray-500 text-center p-4">
            <Sparkles size={48} className="mb-4 opacity-50" />
            <p className="mb-6">
              Drive for a while, then ask the AI to analyze your driving style and efficiency.
            </p>
            <button
              onClick={handleAnalyze}
              className="bg-purple-600 hover:bg-purple-700 text-white px-6 py-3 rounded-full font-bold transition-all flex items-center gap-2 shadow-lg hover:shadow-purple-500/20"
            >
              <Sparkles size={18} />
              {apiKey ? 'Analyze with OpenAI' : 'Analyze Session'}
            </button>
          </div>
        )}

        {loading && (
          <div className="flex-1 flex flex-col items-center justify-center text-purple-400">
            <Loader2 size={48} className="animate-spin mb-4" />
            <p>{apiKey ? 'Contacting OpenAI...' : 'Analyzing telemetry data...'}</p>
          </div>
        )}

        {analysis && !loading && (
          <div className="flex-1 overflow-y-auto">
            <div className="bg-gray-900 rounded-lg p-4 text-gray-300 leading-relaxed whitespace-pre-wrap font-sans border border-gray-700 text-sm">
              {analysis}
            </div>
            <button
              onClick={handleAnalyze}
              className="mt-4 w-full bg-gray-700 hover:bg-gray-600 text-white px-4 py-2 rounded-lg transition-colors text-sm"
            >
              Refresh Analysis
            </button>
          </div>
        )}
      </div>
    </div>
  );
};
