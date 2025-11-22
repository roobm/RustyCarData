// This is a conceptual schema for Tauri Commands (Rust functions callable from JS)

interface TauriCommands {
  /**
   * Starts the CAN bus simulation.
   * @returns Result<void, string>
   */
  start_simulation(): Promise<void>;

  /**
   * Stops the CAN bus simulation.
   * @returns Result<void, string>
   */
  stop_simulation(): Promise<void>;

  /**
   * Gets the current state of the simulation.
   * @returns boolean (true = running)
   */
  get_simulation_status(): Promise<boolean>;

  /**
   * Triggers AI analysis for the current or last session.
   * @returns string (The analysis text)
   */
  analyze_current_session(): Promise<string>;
}

interface TauriEvents {
  /**
   * Real-time data update.
   * Payload: Array of TelemetryPoint
   */
  "telemetry-update": {
    timestamp: number;
    rpm: number;
    speed: number;
    temp: number;
  }[];
}
