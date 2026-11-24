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
    // Advanced anti-fraud filters
    pub mirror_horizontal: bool,
    pub micro_drift: bool,
    pub micro_rotation: bool,
    pub color_jitter: bool,
    pub ultrasonic_bed: bool,
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
            // Defaults for advanced filters
            mirror_horizontal: false,
            micro_drift: true,
            micro_rotation: true,
            color_jitter: true,
            ultrasonic_bed: true,
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
    pub mirror_horizontal: bool,
    pub micro_drift: bool,
    pub rotation_angle: f32,
    pub contrast: f32,
    pub brightness: f32,
    pub saturation: f32,
    pub ultrasonic_bed: bool,
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
                mirror_horizontal: false,
                micro_drift: false,
                rotation_angle: 0.0,
                contrast: 1.0,
                brightness: 0.0,
                saturation: 1.0,
                ultrasonic_bed: false,
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

        let rotation = if self.micro_rotation {
            0.2 + fastrand::f32() * 0.3
        } else {
            0.0
        };

        let (contrast, brightness, saturation) = if self.color_jitter {
            (
                1.01 + fastrand::f32() * 0.04,
                -0.01 + fastrand::f32() * 0.02,
                1.02 + fastrand::f32() * 0.05,
            )
        } else {
            (1.0, 0.0, 1.0)
        };

        AppliedFilters {
            speed_factor: speed,
            zoom_factor: zoom,
            pitch_factor: pitch,
            strip_metadata: self.strip_metadata,
            add_grain: self.add_grain,
            mirror_horizontal: self.mirror_horizontal,
            micro_drift: self.micro_drift,
            rotation_angle: rotation,
            contrast,
            brightness,
            saturation,
            ultrasonic_bed: self.ultrasonic_bed,
        }
    }
}
