import { create } from 'zustand';
import { TelemetryPoint, CanFrame } from '../services/api';

export interface HistoryPoint {
  timestamp: number;
  rpm: number;
  speed: number;
  temp: number;
}

interface AppState {
  // Simulation State
  isRunning: boolean;
  isSniffing: boolean;
  rpm: number;
  speed: number;
  coolantTemp: number;

  // New PIDs from real data
  throttlePosition: number;
  engineLoad: number;
  fuelLevel: number;
  intakeAirTemp: number;
  timingAdvance: number;
  batteryVoltage: number;

  // Data History
  history: HistoryPoint[];
  canFrames: CanFrame[];

  // Actions
  setRunning: (isRunning: boolean) => void;
  setSniffing: (isSniffing: boolean) => void;
  updateTelemetry: (points: TelemetryPoint[]) => void;
  addCanFrame: (frame: CanFrame) => void;
  clearHistory: () => void;
}

export const useStore = create<AppState>((set) => ({
  isRunning: false,
  isSniffing: false,
  rpm: 0,
  speed: 0,
  coolantTemp: 0,
  throttlePosition: 0,
  engineLoad: 0,
  fuelLevel: 100,
  intakeAirTemp: 20,
  timingAdvance: 0,
  batteryVoltage: 12.6,
  history: [],
  canFrames: [],

  setRunning: (isRunning) => set({ isRunning }),
  setSniffing: (isSniffing) => set({ isSniffing }),
  clearHistory: () => set({ history: [], canFrames: [] }),

  updateTelemetry: (points) => set((state) => {
    const updates: Partial<AppState> = {};
    let newRpm = state.rpm;
    let newSpeed = state.speed;
    let newTemp = state.coolantTemp;

    points.forEach(p => {
      if (p.pid === 'EngineRPM') { updates.rpm = p.value; newRpm = p.value; }
      if (p.pid === 'VehicleSpeed') { updates.speed = p.value; newSpeed = p.value; }
      if (p.pid === 'CoolantTemp') { updates.coolantTemp = p.value; newTemp = p.value; }
      if (p.pid === 'ThrottlePosition') { updates.throttlePosition = p.value; }
      if (p.pid === 'EngineLoad') { updates.engineLoad = p.value; }
      if (p.pid === 'FuelLevel') { updates.fuelLevel = p.value; }
      if (p.pid === 'IntakeAirTemp') { updates.intakeAirTemp = p.value; }
      if (p.pid === 'TimingAdvance') { updates.timingAdvance = p.value; }
      if (p.pid === 'BatteryVoltage') { updates.batteryVoltage = p.value; }
    });

    // Add to history (limit to 100 points ~ 10 seconds of high res data)
    const newPoint: HistoryPoint = {
      timestamp: Date.now(),
      rpm: newRpm,
      speed: newSpeed,
      temp: newTemp
    };

    const newHistory = [...state.history, newPoint].slice(-100);

    return { ...updates, history: newHistory };
  }),

  addCanFrame: (frame) => set((state) => ({
    canFrames: [frame, ...state.canFrames].slice(0, 50)
  })),
}));
