import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Legend } from 'recharts';
import { useStore } from '../store/useStore';
import { Download } from 'lucide-react';

export const TelemetryChart: React.FC = () => {
  const { history } = useStore();

  // Format timestamp for XAxis
  const data = history.map(h => ({
    ...h,
    time: new Date(h.timestamp).toLocaleTimeString([], { hour12: false, minute: '2-digit', second: '2-digit' })
  }));

  const handleExport = () => {
    if (history.length === 0) return;

    const headers = ['Timestamp', 'RPM', 'Speed', 'CoolantTemp'];
    const csvContent = [
      headers.join(','),
      ...history.map(h => `${new Date(h.timestamp).toISOString()},${h.rpm.toFixed(2)},${h.speed.toFixed(2)},${h.temp.toFixed(2)}`)
    ].join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.setAttribute('download', `telemetry_export_${new Date().toISOString()}.csv`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  return (
    <div className="bg-gray-800 rounded-lg p-4 shadow-lg h-full flex flex-col">
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-xl font-bold text-gray-300">Live Telemetry</h3>
        <button 
          onClick={handleExport}
          className={`flex items-center gap-2 px-3 py-1 rounded text-sm transition-colors ${
            history.length === 0 
              ? 'bg-gray-700 text-gray-500 cursor-not-allowed' 
              : 'bg-blue-600 hover:bg-blue-700 text-white'
          }`}
          disabled={history.length === 0}
        >
          <Download size={16} />
          Export CSV
        </button>
      </div>
      <div className="flex-1 min-h-[200px]">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data}>
            <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
            <XAxis dataKey="time" stroke="#9ca3af" tick={{fontSize: 12}} interval={19} />
            <YAxis yAxisId="left" stroke="#ef4444" tick={{fontSize: 12}} domain={[0, 8000]} label={{ value: 'RPM', angle: -90, position: 'insideLeft', fill: '#ef4444' }} />
            <YAxis yAxisId="right" orientation="right" stroke="#3b82f6" tick={{fontSize: 12}} domain={[0, 240]} label={{ value: 'Speed', angle: 90, position: 'insideRight', fill: '#3b82f6' }} />
            <Tooltip 
              contentStyle={{ backgroundColor: '#1f2937', border: '1px solid #374151', borderRadius: '8px' }}
              itemStyle={{ color: '#e5e7eb' }}
            />
            <Legend />
            <Line yAxisId="left" type="monotone" dataKey="rpm" stroke="#ef4444" dot={false} strokeWidth={2} name="RPM" isAnimationActive={false} />
            <Line yAxisId="right" type="monotone" dataKey="speed" stroke="#3b82f6" dot={false} strokeWidth={2} name="Speed" isAnimationActive={false} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};
