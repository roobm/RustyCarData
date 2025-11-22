import { invoke } from "@tauri-apps/api/core";

export interface TelemetryPoint {
  timestamp: number;
  pid: string; // Enum as string
  value: number;
  unit: string;
}

export interface CanFrame {
  timestamp: number;
  id: number;
  data: number[];
  is_extended: boolean;
}

export const api = {
  startSimulation: async (): Promise<void> => {
    return await invoke("start_simulation");
  },

  stopSimulation: async (): Promise<void> => {
    return await invoke("stop_simulation");
  },

  startSniffing: async (): Promise<void> => {
    return await invoke("start_sniffing");
  },

  stopSniffing: async (): Promise<void> => {
    return await invoke("stop_sniffing");
  },

  getSimulationStatus: async (): Promise<boolean> => {
    return await invoke("get_simulation_status");
  },

  analyzeCurrentSession: async (): Promise<string> => {
    return await invoke("analyze_current_session");
  },
};
