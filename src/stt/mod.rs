use anyhow::{Context, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STTConfig {
    pub enabled: bool,
    pub model_path: Option<PathBuf>,
    pub language: String,
    pub font_size: u32,
    pub highlight_current_word: bool,
}

impl Default for STTConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            model_path: None,
            language: "auto".to_string(),
            font_size: 28,
            highlight_current_word: true,
        }
    }
}

pub struct STTManager;

impl STTManager {
    /// Returns default models storage directory in ~/.cache/qsnap/models/
    pub fn default_models_dir() -> PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("qsnap")
            .join("models")
    }

    /// Checks if the tiny model exists in default cache
    pub fn tiny_model_path() -> PathBuf {
        Self::default_models_dir().join("ggml-tiny.bin")
    }

    /// Returns active model path: either custom path or default cached tiny model if present
    pub fn resolve_model_path(config: &STTConfig) -> Option<PathBuf> {
        if let Some(ref p) = config.model_path {
            if p.exists() {
                return Some(p.clone());
            }
        }
        let tiny = Self::tiny_model_path();
        if tiny.exists() {
            Some(tiny)
        } else {
            None
        }
    }

    /// Downloads ggml-tiny.bin (~75MB) asynchronously via curl to the models directory
    pub async fn download_tiny_model() -> Result<PathBuf> {
        let dir = Self::default_models_dir();
        fs::create_dir_all(&dir)?;
        let target_path = dir.join("ggml-tiny.bin");

        if target_path.exists() {
            info!("Model ggml-tiny.bin already downloaded.");
            return Ok(target_path);
        }

        let temp_download = dir.join("ggml-tiny.bin.tmp");
        let url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin";

        info!("Downloading Whisper model: {} -> {}", url, target_path.display());

        let status = tokio::process::Command::new("curl")
            .args(["-L", "-f", "-o", &temp_download.to_string_lossy(), url])
            .status()
            .await
            .context("Failed to run curl for model download")?;

        if !status.success() {
            let _ = fs::remove_file(&temp_download);
            anyhow::bail!("Curl failed to download Whisper model");
        }

        fs::rename(&temp_download, &target_path)?;
        info!("Whisper model successfully installed at {}", target_path.display());
        Ok(target_path)
    }

    /// Extracts clean 16kHz mono WAV audio required by Whisper speech recognition
    pub fn extract_audio_segment(
        video_path: &Path,
        start_secs: f64,
        duration_secs: f64,
        output_wav: &Path,
    ) -> Result<()> {
        let status = Command::new("ffmpeg")
            .arg("-y")
            .args(["-ss", &format!("{:.2}", start_secs)])
            .args(["-t", &format!("{:.2}", duration_secs)])
            .args(["-i", &video_path.to_string_lossy()])
            .args(["-vn", "-acodec", "pcm_s16le", "-ar", "16000", "-ac", "1"])
            .arg(output_wav)
            .status()
            .context("Failed to run ffmpeg audio extractor")?;

        if !status.success() {
            anyhow::bail!("FFmpeg audio extraction failed");
        }

        Ok(())
    }

    /// Generates stylized ASS subtitles file with Unbounded font styling
    pub fn generate_ass_header(font_name: &str, font_size: u32) -> String {
        format!(
            "[Script Info]\n\
             ScriptType: v4.00+\n\
             PlayResX: 1080\n\
             PlayResY: 1920\n\
             ScaledBorderAndShadow: yes\n\n\
             [V4+ Styles]\n\
             Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\n\
             Style: Default,{font},{size},&H00FFFFFF,&H000000FF,&H00000000,&H80000000,-1,0,0,0,100,100,0,0,1,3,1,2,30,30,520,1\n\n\
             [Events]\n\
             Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n",
            font = font_name,
            size = font_size
        )
    }
}
