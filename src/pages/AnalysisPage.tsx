import React from 'react';
import { AiAssistant } from '../components/AiAssistant';

export const AnalysisPage: React.FC = () => {
  return (
    <div className="p-6 h-full flex flex-col">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white">AI Analysis</h2>
        <p className="text-gray-400">Get insights on your driving sessions powered by OpenAI.</p>
      </header>

      <div className="flex-1 max-w-4xl mx-auto w-full">
        <AiAssistant />
      </div>
    </div>
  );
};
