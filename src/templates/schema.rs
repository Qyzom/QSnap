use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BannerConfig {
    pub text: String,
    pub font_size: u32,
    pub text_color: String,
    pub bg_color: String,
    pub height_px: u32,
    pub border_radius: u32,
}

impl Default for BannerConfig {
    fn default() -> Self {
        Self {
            text: "Follow for Part 2".to_string(),
            font_size: 32,
            text_color: "#FFFFFF".to_string(),
            bg_color: "#1E1E2E".to_string(),
            height_px: 120,
            border_radius: 16,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleConfig {
    pub enabled: bool,
    pub font_name: String,
    pub font_size: u32,
    pub primary_color: String,
    pub outline_color: String,
    pub outline_width: u32,
}

impl Default for SubtitleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            font_name: "Unbounded".to_string(),
            font_size: 28,
            primary_color: "#00F0FF".to_string(),
            outline_color: "#000000".to_string(),
            outline_width: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutTemplate {
    pub id: String,
    pub name: String,
    pub target_width: u32,
    pub target_height: u32,
    pub top_ratio: f32,
    pub bottom_ratio: f32,
    pub banner: BannerConfig,
    pub subtitles: SubtitleConfig,
}

impl Default for LayoutTemplate {
    fn default() -> Self {
        Self {
            id: "default_split".to_string(),
            name: "Classic Split-Screen 50/50".to_string(),
            target_width: 1080,
            target_height: 1920,
            top_ratio: 0.50,
            bottom_ratio: 0.50,
            banner: BannerConfig::default(),
            subtitles: SubtitleConfig::default(),
        }
    }
}
