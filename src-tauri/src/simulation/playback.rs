use crate::models::TelemetryPoint;
use std::sync::{Arc, Mutex};

/// Playback state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Idle,
    Playing,
    Paused,
    Ended,
}

/// Dataset playback engine
pub struct DatasetPlayback {
    /// All telemetry points loaded from dataset
    data: Vec<TelemetryPoint>,
    /// Current playback index
    current_index: usize,
    /// Playback state
    state: Arc<Mutex<PlaybackState>>,
    /// Playback speed multiplier (1.0 = normal, 2.0 = 2x speed)
    speed: Arc<Mutex<f64>>,
    /// Loop playback when reaching end
    loop_enabled: bool,
}

impl DatasetPlayback {
    /// Create new playback engine
    pub fn new(data: Vec<TelemetryPoint>) -> Self {
        Self {
            data,
            current_index: 0,
            state: Arc::new(Mutex::new(PlaybackState::Idle)),
            speed: Arc::new(Mutex::new(1.0)),
            loop_enabled: true,
        }
    }

    /// Start playback
    pub fn play(&mut self) {
        let mut state = self.state.lock().unwrap();
        *state = PlaybackState::Playing;
    }

    /// Pause playback
    pub fn pause(&mut self) {
        let mut state = self.state.lock().unwrap();
        *state = PlaybackState::Paused;
    }

    /// Stop playback and reset to beginning
    pub fn stop(&mut self) {
        let mut state = self.state.lock().unwrap();
        *state = PlaybackState::Idle;
        self.current_index = 0;
    }

    /// Set playback speed (1.0 = normal, 2.0 = 2x, 0.5 = half speed)
    pub fn set_speed(&mut self, speed: f64) {
        let mut current_speed = self.speed.lock().unwrap();
        *current_speed = speed.max(0.1).min(10.0); // Clamp between 0.1x and 10x
    }

    /// Get current playback speed
    pub fn get_speed(&self) -> f64 {
        *self.speed.lock().unwrap()
    }

    /// Enable or disable looping
    pub fn set_loop(&mut self, enabled: bool) {
        self.loop_enabled = enabled;
    }

    /// Get current playback state
    pub fn get_state(&self) -> PlaybackState {
        *self.state.lock().unwrap()
    }

    /// Get next frame of telemetry points
    /// Returns points that share the same timestamp
    pub fn next_frame(&mut self) -> Option<Vec<TelemetryPoint>> {
        let state = *self.state.lock().unwrap();

        // Only advance if playing
        if state != PlaybackState::Playing {
            return None;
        }

        // Check if we've reached the end
        if self.current_index >= self.data.len() {
            if self.loop_enabled {
                // Loop back to beginning
                self.current_index = 0;
            } else {
                // Stop at end
                let mut state = self.state.lock().unwrap();
                *state = PlaybackState::Ended;
                return None;
            }
        }

        // Get all points with the current timestamp
        let mut frame_points = Vec::new();

        if let Some(first_point) = self.data.get(self.current_index) {
            let current_timestamp = first_point.timestamp;

            // Collect all points with same timestamp
            while let Some(point) = self.data.get(self.current_index) {
                if point.timestamp == current_timestamp {
                    frame_points.push(point.clone());
                    self.current_index += 1;
                } else {
                    break;
                }
            }
        }

        if frame_points.is_empty() {
            None
        } else {
            Some(frame_points)
        }
    }

    /// Get playback progress (0.0 to 1.0)
    pub fn get_progress(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        self.current_index as f64 / self.data.len() as f64
    }

    /// Get total duration in seconds
    pub fn get_duration(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }

        if let (Some(first), Some(last)) = (self.data.first(), self.data.last()) {
            (last.timestamp - first.timestamp) as f64 / 1_000_000.0
        } else {
            0.0
        }
    }

    /// Get current playback time in seconds
    pub fn get_current_time(&self) -> f64 {
        if self.data.is_empty() || self.current_index == 0 {
            return 0.0;
        }

        if let (Some(first), Some(current)) = (
            self.data.first(),
            self.data.get(self.current_index.saturating_sub(1)),
        ) {
            (current.timestamp - first.timestamp) as f64 / 1_000_000.0
        } else {
            0.0
        }
    }

    /// Seek to a specific time (in seconds)
    pub fn seek(&mut self, time_seconds: f64) {
        if self.data.is_empty() {
            return;
        }

        let target_timestamp = if let Some(first) = self.data.first() {
            first.timestamp + (time_seconds * 1_000_000.0) as u64
        } else {
            return;
        };

        // Binary search for closest timestamp
        let mut left = 0;
        let mut right = self.data.len();

        while left < right {
            let mid = (left + right) / 2;
            if let Some(point) = self.data.get(mid) {
                if point.timestamp < target_timestamp {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            } else {
                break;
            }
        }

        self.current_index = left.min(self.data.len());
    }

    /// Get total number of data points
    pub fn get_total_points(&self) -> usize {
        self.data.len()
    }

    /// Check if playback is active
    pub fn is_playing(&self) -> bool {
        *self.state.lock().unwrap() == PlaybackState::Playing
    }

    /// Check if playback has ended
    pub fn is_ended(&self) -> bool {
        *self.state.lock().unwrap() == PlaybackState::Ended
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ObdPid;

    fn create_test_data() -> Vec<TelemetryPoint> {
        vec![
            TelemetryPoint {
                timestamp: 0,
                pid: ObdPid::EngineRPM,
                value: 1000.0,
                unit: "rpm".to_string(),
            },
            TelemetryPoint {
                timestamp: 0,
                pid: ObdPid::VehicleSpeed,
                value: 0.0,
                unit: "km/h".to_string(),
            },
            TelemetryPoint {
                timestamp: 1_000_000,
                pid: ObdPid::EngineRPM,
                value: 2000.0,
                unit: "rpm".to_string(),
            },
            TelemetryPoint {
                timestamp: 1_000_000,
                pid: ObdPid::VehicleSpeed,
                value: 20.0,
                unit: "km/h".to_string(),
            },
        ]
    }

    #[test]
    fn test_playback_creation() {
        let data = create_test_data();
        let playback = DatasetPlayback::new(data);

        assert_eq!(playback.get_state(), PlaybackState::Idle);
        assert_eq!(playback.get_total_points(), 4);
    }

    #[test]
    fn test_playback_next_frame() {
        let data = create_test_data();
        let mut playback = DatasetPlayback::new(data);

        playback.play();

        // First frame should have 2 points (timestamp 0)
        let frame1 = playback.next_frame();
        assert!(frame1.is_some());
        assert_eq!(frame1.unwrap().len(), 2);

        // Second frame should have 2 points (timestamp 1_000_000)
        let frame2 = playback.next_frame();
        assert!(frame2.is_some());
        assert_eq!(frame2.unwrap().len(), 2);
    }

    #[test]
    fn test_playback_speed() {
        let data = create_test_data();
        let mut playback = DatasetPlayback::new(data);

        playback.set_speed(2.0);
        assert_eq!(playback.get_speed(), 2.0);

        playback.set_speed(0.5);
        assert_eq!(playback.get_speed(), 0.5);
    }

    #[test]
    fn test_playback_loop() {
        let data = create_test_data();
        let mut playback = DatasetPlayback::new(data);
        playback.set_loop(true);

        playback.play();

        // Consume all frames
        while playback.next_frame().is_some() {}

        // Should loop back and still be playing
        assert_eq!(playback.get_state(), PlaybackState::Playing);
        assert!(playback.next_frame().is_some());
    }

    #[test]
    fn test_playback_progress() {
        let data = create_test_data();
        let mut playback = DatasetPlayback::new(data);

        assert_eq!(playback.get_progress(), 0.0);

        playback.play();
        playback.next_frame();

        assert!(playback.get_progress() > 0.0);
        assert!(playback.get_progress() < 1.0);
    }
}
