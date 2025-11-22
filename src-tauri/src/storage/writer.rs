use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use crate::models::TelemetryPoint;

pub struct ParquetWriter {
    file_path: PathBuf,
    buffer: Vec<TelemetryPoint>,
}

impl ParquetWriter {
    pub fn new(path: PathBuf) -> Self {
        Self {
            file_path: path,
            buffer: Vec::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        // For CSV, we don't need to do anything on init
        Ok(())
    }

    pub fn write_point(&mut self, point: &TelemetryPoint) -> Result<(), String> {
        self.buffer.push(point.clone());
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), String> {
        // Write all buffered data as CSV
        let mut file = File::create(&self.file_path).map_err(|e| e.to_string())?;
        
        // Write CSV header
        writeln!(file, "timestamp,pid,value,unit").map_err(|e| e.to_string())?;
        
        // Write data rows
        for point in &self.buffer {
            writeln!(
                file,
                "{},{},{},{}",
                point.timestamp,
                point.pid.as_str(),
                point.value,
                point.unit
            ).map_err(|e| e.to_string())?;
        }
        
        self.buffer.clear();
        Ok(())
    }
}
