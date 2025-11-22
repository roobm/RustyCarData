import React, { useState } from 'react';
import { api } from '../services/api';
import { Bot, Sparkles, Loader2 } from 'lucide-react';

export const AiAssistant: React.FC = () => {
  const [analysis, setAnalysis] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleAnalyze = async () => {
    setLoading(true);
    try {
      // Simulate network delay for "AI thinking" effect
      await new Promise(resolve => setTimeout(resolve, 1500));
      const result = await api.analyzeCurrentSession();
      setAnalysis(result);
    } catch (error) {
      console.error("Failed to analyze session", error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="bg-gray-800 rounded-lg p-6 shadow-lg h-full flex flex-col">
      <div className="flex justify-between items-center mb-6">
        <h3 className="text-xl font-bold text-purple-400 flex items-center gap-2">
          <Bot size={24} />
          AI Co-Pilot
        </h3>
      </div>

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
              Analyze Session
            </button>
          </div>
        )}

        {loading && (
          <div className="flex-1 flex flex-col items-center justify-center text-purple-400">
            <Loader2 size={48} className="animate-spin mb-4" />
            <p>Analyzing telemetry data...</p>
          </div>
        )}

        {analysis && !loading && (
          <div className="flex-1 overflow-y-auto">
            <div className="bg-gray-900 rounded-lg p-4 text-gray-300 leading-relaxed whitespace-pre-wrap font-sans border border-gray-700">
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
