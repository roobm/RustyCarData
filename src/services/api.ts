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

  analyzeCurrentSession: async (apiKey?: string): Promise<string> => {
    return await invoke("analyze_current_session", { apiKey });
  },

  saveExportCsv: async (csvContent: string, filename: string): Promise<string> => {
    return await invoke("save_export_csv", { csvContent, filename });
  },

  importFile: async (filePath: string): Promise<string> => {
    return await invoke("import_file", { filePath });
  },

  listSessions: async (): Promise<string[]> => {
    return await invoke("list_sessions");
  },

  readSession: async (filename: string): Promise<string> => {
    return await invoke("read_session", { filename });
  },
};
