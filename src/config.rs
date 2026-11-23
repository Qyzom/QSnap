use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub output_dir: PathBuf,
    pub broll_dir: Option<PathBuf>,
    pub clip_duration_secs: f64,
    pub clips_count: u32,
    pub language: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let default_output = dirs::download_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("QSnap_Clips");

        Self {
            output_dir: default_output,
            broll_dir: None,
            clip_duration_secs: 45.0,
            clips_count: 10,
            language: "ru".to_string(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("qsnap")
            .join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str(&content) {
                    return cfg;
                }
            }
        }
        let default_cfg = Self::default();
        let _ = default_cfg.save();
        default_cfg
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}
