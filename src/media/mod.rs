use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFile {
    pub path: PathBuf,
    pub filename: String,
    pub duration_secs: f64,
}

pub struct MediaManager;

impl MediaManager {
    pub fn probe_duration(path: &Path) -> Result<f64> {
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-show_entries",
                "format=duration",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
            ])
            .arg(path)
            .output()?;

        if output.status.success() {
            let str_val = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let duration: f64 = str_val.parse().unwrap_or(0.0);
            Ok(duration)
        } else {
            Ok(0.0)
        }
    }

    pub fn scan_directory(dir: &Path) -> Vec<MediaFile> {
        let mut files = Vec::new();
        let valid_exts = ["mp4", "mkv", "mov", "avi", "webm"];

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                        if valid_exts.contains(&ext.to_lowercase().as_str()) {
                            let duration = Self::probe_duration(&p).unwrap_or(0.0);
                            let filename = p
                                .file_name()
                                .map(|f| f.to_string_lossy().to_string())
                                .unwrap_or_default();
                            files.push(MediaFile {
                                path: p,
                                filename,
                                duration_secs: duration,
                            });
                        }
                    }
                }
            }
        }
        files
    }
}
