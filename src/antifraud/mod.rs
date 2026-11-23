use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiFraudConfig {
    pub enabled: bool,
    pub speed_shift: bool,
    pub min_speed: f32,
    pub max_speed: f32,
    pub pitch_shift: bool,
    pub micro_zoom: bool,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub add_grain: bool,
    pub strip_metadata: bool,
}

impl Default for AntiFraudConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            speed_shift: true,
            min_speed: 1.01,
            max_speed: 1.04,
            pitch_shift: true,
            micro_zoom: true,
            min_zoom: 1.02,
            max_zoom: 1.05,
            add_grain: true,
            strip_metadata: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppliedFilters {
    pub speed_factor: f32,
    pub zoom_factor: f32,
    pub pitch_factor: f32,
    pub strip_metadata: bool,
    pub add_grain: bool,
}

impl AntiFraudConfig {
    pub fn generate_instance(&self) -> AppliedFilters {
        if !self.enabled {
            return AppliedFilters {
                speed_factor: 1.0,
                zoom_factor: 1.0,
                pitch_factor: 1.0,
                strip_metadata: false,
                add_grain: false,
            };
        }

        let speed = if self.speed_shift {
            let delta = self.max_speed - self.min_speed;
            self.min_speed + fastrand::f32() * delta
        } else {
            1.0
        };

        let zoom = if self.micro_zoom {
            let delta = self.max_zoom - self.min_zoom;
            self.min_zoom + fastrand::f32() * delta
        } else {
            1.0
        };

        let pitch = if self.pitch_shift {
            0.98 + fastrand::f32() * 0.04
        } else {
            1.0
        };

        AppliedFilters {
            speed_factor: speed,
            zoom_factor: zoom,
            pitch_factor: pitch,
            strip_metadata: self.strip_metadata,
            add_grain: self.add_grain,
        }
    }
}
