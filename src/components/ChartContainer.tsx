import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Brush,
} from 'recharts';
import { ChartDataPoint } from '../types/session';

interface ChartContainerProps {
  data: ChartDataPoint[];
  parameterName: string;
  unit: string;
  color: string;
  syncId: string;
  zoomDomain: [number, number] | null;
  onZoomChange?: (domain: [number, number] | null) => void;
}

export default function ChartContainer({
  data,
  parameterName,
  unit,
  color,
  syncId,
  zoomDomain,
  onZoomChange,
}: ChartContainerProps) {
  if (data.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-6">
        <h3 className="text-sm font-medium text-gray-900 dark:text-white mb-2">
          {parameterName}
        </h3>
        <p className="text-sm text-gray-500 dark:text-gray-400">
          No data available for this parameter
        </p>
      </div>
    );
  }

  // Calculate domain from data
  const timestamps = data.map((d) => d.timestamp);
  const values = data.map((d) => d.value);
  const minTimestamp = Math.min(...timestamps);
  const maxTimestamp = Math.max(...timestamps);
  const minValue = Math.min(...values);
  const maxValue = Math.max(...values);

  // Add 5% padding to value range for better visibility
  const valuePadding = (maxValue - minValue) * 0.05;
  const yDomain = [
    Math.max(0, minValue - valuePadding),
    maxValue + valuePadding,
  ];

  // Apply zoom domain if set
  const xDomain = zoomDomain || [minTimestamp, maxTimestamp];

  // Format value based on unit
  const formatValue = (value: number): string => {
    if (unit === 'V' || unit === 'g/s') {
      return value.toFixed(2);
    }
    if (unit === '°' || unit === '°C') {
      return value.toFixed(1);
    }
    return value.toFixed(0);
  };

  // Format timestamp for X-axis
  const formatXAxis = (timestamp: number): string => {
    if (timestamp < 1000000000000) {
      // If timestamp is small, it's an index
      return `${timestamp}`;
    }
    // Otherwise it's a real timestamp
    const date = new Date(timestamp);
    return date.toLocaleTimeString('en-US', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  const handleBrushChange = (domain: { startIndex?: number; endIndex?: number }) => {
    if (!onZoomChange) return;

    if (domain.startIndex !== undefined && domain.endIndex !== undefined) {
      const startTimestamp = data[domain.startIndex].timestamp;
      const endTimestamp = data[domain.endIndex].timestamp;
      onZoomChange([startTimestamp, endTimestamp]);
    }
  };

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4">
      <h3 className="text-sm font-medium text-gray-900 dark:text-white mb-3">
        {parameterName} ({unit})
      </h3>

      <ResponsiveContainer width="100%" height={200}>
        <LineChart
          data={data}
          syncId={syncId}
          margin={{ top: 5, right: 20, left: 10, bottom: 5 }}
        >
          <CartesianGrid
            strokeDasharray="3 3"
            stroke="#374151"
            opacity={0.3}
          />
          <XAxis
            dataKey="timestamp"
            domain={xDomain}
            tickFormatter={formatXAxis}
            stroke="#9ca3af"
            style={{ fontSize: '12px' }}
            minTickGap={50}
          />
          <YAxis
            domain={yDomain}
            tickFormatter={formatValue}
            stroke="#9ca3af"
            style={{ fontSize: '12px' }}
            width={60}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: '#1f2937',
              border: '1px solid #374151',
              borderRadius: '0.5rem',
              color: '#fff',
            }}
            labelFormatter={(timestamp) => {
              if (typeof timestamp === 'number' && timestamp >= 1000000000000) {
                return new Date(timestamp).toLocaleString();
              }
              return `Point ${timestamp}`;
            }}
            formatter={(value: number) => [
              `${formatValue(value)} ${unit}`,
              parameterName,
            ]}
          />
          <Line
            type="monotone"
            dataKey="value"
            stroke={color}
            strokeWidth={2}
            dot={false}
            isAnimationActive={false}
          />
          {onZoomChange && (
            <Brush
              dataKey="timestamp"
              height={30}
              stroke={color}
              fill="#1f2937"
              onChange={handleBrushChange}
              tickFormatter={formatXAxis}
            />
          )}
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
