import { useState } from 'react';
import { TelemetryPoint, ChartDataPoint } from '../types/session';
import ChartContainer from './ChartContainer';
import { RotateCcw } from 'lucide-react';

interface TelemetryChartsProps {
  telemetry: TelemetryPoint[];
  sessionName: string;
}

interface ParameterConfig {
  key: keyof TelemetryPoint;
  name: string;
  unit: string;
  color: string;
}

const PARAMETERS: ParameterConfig[] = [
  { key: 'engine_rpm', name: 'Engine RPM', unit: 'rpm', color: '#3b82f6' }, // blue
  { key: 'vehicle_speed', name: 'Vehicle Speed', unit: 'km/h', color: '#10b981' }, // green
  { key: 'coolant_temp', name: 'Coolant Temperature', unit: '°C', color: '#ef4444' }, // red
  { key: 'throttle_position', name: 'Throttle Position', unit: '%', color: '#f59e0b' }, // amber
  { key: 'engine_load', name: 'Engine Load', unit: '%', color: '#8b5cf6' }, // purple
  { key: 'fuel_level', name: 'Fuel Level', unit: '%', color: '#06b6d4' }, // cyan
  { key: 'intake_air_temp', name: 'Intake Air Temperature', unit: '°C', color: '#ec4899' }, // pink
  { key: 'timing_advance', name: 'Timing Advance', unit: '°', color: '#14b8a6' }, // teal
  { key: 'battery_voltage', name: 'Battery Voltage', unit: 'V', color: '#f97316' }, // orange
  { key: 'maf_flow_rate', name: 'MAF Flow Rate', unit: 'g/s', color: '#84cc16' }, // lime
  { key: 'o2_sensor_voltage', name: 'O2 Sensor Voltage', unit: 'V', color: '#a855f7' }, // violet
  { key: 'fuel_pressure', name: 'Fuel Pressure', unit: 'kPa', color: '#06b6d4' }, // cyan
];

export default function TelemetryCharts({ telemetry, sessionName }: TelemetryChartsProps) {
  const [zoomDomain, setZoomDomain] = useState<[number, number] | null>(null);

  // Transform telemetry data to chart format
  const getChartData = (parameterKey: keyof TelemetryPoint): ChartDataPoint[] => {
    return telemetry
      .filter((point) => {
        const value = point[parameterKey];
        return value !== undefined && value !== null;
      })
      .map((point, index) => {
        const value = point[parameterKey];

        // Use index as timestamp if actual timestamp not available
        const timestamp = point.timestamp
          ? new Date(point.timestamp).getTime()
          : index;

        return {
          timestamp,
          value: typeof value === 'number' ? value : 0,
          formattedTime: point.timestamp
            ? new Date(point.timestamp).toLocaleTimeString()
            : `Point ${index}`,
        };
      });
  };

  // Check if parameter has any data
  const hasData = (parameterKey: keyof TelemetryPoint): boolean => {
    return telemetry.some((point) => {
      const value = point[parameterKey];
      return value !== undefined && value !== null;
    });
  };

  // Filter parameters that have data
  const availableParameters = PARAMETERS.filter((param) => hasData(param.key));

  const handleResetZoom = () => {
    setZoomDomain(null);
  };

  if (availableParameters.length === 0) {
    return (
      <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-8 text-center">
        <p className="text-gray-600 dark:text-gray-400">
          No parameters with data available to display.
        </p>
      </div>
    );
  }

  // Unique syncId for all charts to synchronize zoom
  const syncId = `session-${sessionName}`;

  return (
    <div className="space-y-6">
      {/* Header with zoom reset */}
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
          Telemetry Data
        </h2>
        {zoomDomain && (
          <button
            onClick={handleResetZoom}
            className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            <RotateCcw className="w-4 h-4" />
            Reset Zoom
          </button>
        )}
      </div>

      {/* Charts Grid */}
      <div className="space-y-4">
        {availableParameters.map((param, index) => (
          <ChartContainer
            key={param.key}
            data={getChartData(param.key)}
            parameterName={param.name}
            unit={param.unit}
            color={param.color}
            syncId={syncId}
            zoomDomain={zoomDomain}
            onZoomChange={index === 0 ? setZoomDomain : undefined} // Only first chart controls zoom
          />
        ))}
      </div>

      {/* Info footer */}
      <div className="text-sm text-gray-600 dark:text-gray-400 text-center">
        Displaying {availableParameters.length} parameter
        {availableParameters.length !== 1 ? 's' : ''} across {telemetry.length.toLocaleString()}{' '}
        data points
      </div>
    </div>
  );
}
