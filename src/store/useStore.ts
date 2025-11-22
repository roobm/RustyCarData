import { create } from 'zustand';
import { TelemetryPoint } from '../services/api';

interface AppState {
  // Simulation State
  isRunning: boolean;
  rpm: number;
  speed: number;
  coolantTemp: number;
  
  // Actions
  setRunning: (isRunning: boolean) => void;
  updateTelemetry: (points: TelemetryPoint[]) => void;
}

export const useStore = create<AppState>((set) => ({
  isRunning: false,
  rpm: 0,
  speed: 0,
  coolantTemp: 0,

  setRunning: (isRunning) => set({ isRunning }),
  
  updateTelemetry: (points) => set(() => {
    const updates: Partial<AppState> = {};
    
    points.forEach(p => {
      if (p.pid === 'EngineRPM') updates.rpm = p.value;
      if (p.pid === 'VehicleSpeed') updates.speed = p.value;
      if (p.pid === 'CoolantTemp') updates.coolantTemp = p.value;
    });

    return updates;
  }),
}));
