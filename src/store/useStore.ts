import { create } from 'zustand';
import { TelemetryPoint, CanFrame } from '../services/api';

interface AppState {
  // Simulation State
  isRunning: boolean;
  isSniffing: boolean;
  rpm: number;
  speed: number;
  coolantTemp: number;
  
  // CAN Data
  canFrames: CanFrame[];

  // Actions
  setRunning: (isRunning: boolean) => void;
  setSniffing: (isSniffing: boolean) => void;
  updateTelemetry: (points: TelemetryPoint[]) => void;
  addCanFrame: (frame: CanFrame) => void;
}

export const useStore = create<AppState>((set) => ({
  isRunning: false,
  isSniffing: false,
  rpm: 0,
  speed: 0,
  coolantTemp: 0,
  canFrames: [],

  setRunning: (isRunning) => set({ isRunning }),
  setSniffing: (isSniffing) => set({ isSniffing }),
  
  updateTelemetry: (points) => set(() => {
    const updates: Partial<AppState> = {};
    
    points.forEach(p => {
      if (p.pid === 'EngineRPM') updates.rpm = p.value;
      if (p.pid === 'VehicleSpeed') updates.speed = p.value;
      if (p.pid === 'CoolantTemp') updates.coolantTemp = p.value;
    });

    return updates;
  }),

  addCanFrame: (frame) => set((state) => ({
    canFrames: [frame, ...state.canFrames].slice(0, 50)
  })),
}));
