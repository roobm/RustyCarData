use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CanFrame {
    pub timestamp: u64,
    pub id: u32,
    pub data: [u8; 8],
    pub is_extended: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ObdPid {
    EngineRPM = 0x0C,
    VehicleSpeed = 0x0D,
    CoolantTemp = 0x05,
    ThrottlePos = 0x11,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPoint {
    pub timestamp: u64,
    pub pid: ObdPid,
    pub value: f64,
    pub unit: String,
}
