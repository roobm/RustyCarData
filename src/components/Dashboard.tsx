import React, { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { Gauge } from './Gauge';
import { CanMonitor } from './CanMonitor';
import { AiAssistant } from './AiAssistant';
import { TelemetryChart } from './TelemetryChart';
import { useStore } from '../store/useStore';
import { api, TelemetryPoint } from '../services/api';
import { Play, Square } from 'lucide-react';

export const Dashboard: React.FC = () => {
  const { rpm, speed, coolantTemp, isRunning, setRunning, updateTelemetry } = useStore();

  useEffect(() => {
    const unlisten = listen<TelemetryPoint[]>('telemetry-update', (event) => {
      updateTelemetry(event.payload);
    });

    return () => {
      unlisten.then(f => f());
    };
  }, [updateTelemetry]);

  const toggleSimulation = async () => {
    if (isRunning) {
      await api.stopSimulation();
      setRunning(false);
    } else {
      await api.startSimulation();
      setRunning(true);
    }
  };

  return (
    <div className="p-6 bg-gray-900 min-h-screen text-white">
      <header className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold text-blue-400">Rust Car Data</h1>
        <button
          onClick={toggleSimulation}
          className={`flex items-center gap-2 px-6 py-3 rounded-full font-bold transition-colors ${
            isRunning 
              ? 'bg-red-600 hover:bg-red-700' 
              : 'bg-green-600 hover:bg-green-700'
          }`}
        >
          {isRunning ? <><Square size={20} /> Stop Engine</> : <><Play size={20} /> Start Engine</>}
        </button>
      </header>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 flex flex-col gap-6">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <Gauge 
              value={rpm} 
              max={8000} 
              label="Engine RPM" 
              unit="rpm" 
              color="#ef4444" 
            />
            <Gauge 
              value={speed} 
              max={240} 
              label="Vehicle Speed" 
              unit="km/h" 
              color="#3b82f6" 
            />
            <Gauge 
              value={coolantTemp} 
              max={130} 
              label="Coolant Temp" 
              unit="°C" 
              color="#10b981" 
            />
          </div>

          <div className="h-[300px]">
            <TelemetryChart />
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="p-4 bg-gray-800 rounded-lg h-fit">
              <h3 className="text-lg font-semibold mb-2 text-gray-400">System Status</h3>
              <div className="flex gap-4">
                <div className="flex items-center gap-2">
                  <div className={`w-3 h-3 rounded-full ${isRunning ? 'bg-green-500 animate-pulse' : 'bg-red-500'}`} />
                  <span>{isRunning ? 'Simulation Running' : 'Engine Stopped'}</span>
                </div>
              </div>
              <div className="mt-4 text-sm text-gray-500">
                <p>Backend: Rust (Tauri v2)</p>
                <p>Frontend: React + Tailwind</p>
                <p>Protocol: Mock OBD-II / CAN</p>
              </div>
            </div>
            
            <div className="h-[300px]">
              <AiAssistant />
            </div>
          </div>
        </div>

        <div className="lg:col-span-1 h-[600px]">
          <CanMonitor />
        </div>
      </div>
    </div>
  );
};
