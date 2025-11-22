import React from 'react';
import { PieChart, Pie, Cell } from 'recharts';

interface GaugeProps {
  value: number;
  max: number;
  label: string;
  unit: string;
  color?: string;
}

export const Gauge: React.FC<GaugeProps> = ({ value, max, label, unit, color = "#8884d8" }) => {
  const normalizedValue = Math.min(Math.max(value, 0), max);
  
  const data = [
    { name: 'value', value: normalizedValue },
    { name: 'rest', value: max - normalizedValue },
  ];

  return (
    <div className="flex flex-col items-center justify-center p-4 bg-gray-800 rounded-lg shadow-lg">
      <h3 className="text-xl font-bold text-gray-300 mb-2">{label}</h3>
      <div className="relative">
        <PieChart width={200} height={100}>
          <Pie
            data={data}
            cx={100}
            cy={100}
            startAngle={180}
            endAngle={0}
            innerRadius={60}
            outerRadius={80}
            paddingAngle={0}
            dataKey="value"
            stroke="none"
          >
            <Cell key="value" fill={color} />
            <Cell key="rest" fill="#374151" />
          </Pie>
        </PieChart>
        <div className="absolute bottom-0 left-0 right-0 text-center">
          <span className="text-3xl font-bold text-white">{Math.round(value)}</span>
          <span className="text-sm text-gray-400 ml-1">{unit}</span>
        </div>
      </div>
    </div>
  );
};
