pub mod filtergraph;

use crate::antifraud::{AntiFraudConfig, AppliedFilters};
use crate::templates::schema::LayoutTemplate;
use anyhow::{Context, Result};
use filtergraph::FilterGraphBuilder;
use log::info;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

static HAS_CHECKED_HW: AtomicBool = AtomicBool::new(false);
static USE_NVENC: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
pub struct RenderJob {
    pub main_video_path: PathBuf,
    pub broll_video_path: PathBuf,
    pub start_time_secs: f64,
    pub duration_secs: f64,
    pub output_path: PathBuf,
    pub template: LayoutTemplate,
    pub antifraud: AntiFraudConfig,
    pub font_path: Option<PathBuf>,
}

pub struct RenderEngine;

impl RenderEngine {
    /// Detects if NVIDIA NVENC hardware acceleration is available.
    pub fn is_nvenc_available() -> bool {
        if !HAS_CHECKED_HW.load(Ordering::Relaxed) {
            let ok = Command::new("ffmpeg")
                .args(["-f", "lavfi", "-i", "color=c=black:s=64x64:d=0.1", "-c:v", "h264_nvenc", "-f", "null", "-"])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false);

            USE_NVENC.store(ok, Ordering::Relaxed);
            HAS_CHECKED_HW.store(true, Ordering::Relaxed);

            if ok {
                info!("Hardware acceleration active: NVIDIA NVENC (h264_nvenc)");
            } else {
                info!("Hardware acceleration not found: using CPU libx264");
            }
        }

        USE_NVENC.load(Ordering::Relaxed)
    }

    /// Renders a single 1080x1920 frame preview image to disk in fractions of a second.
    pub fn render_preview_frame(
        main_video: &PathBuf,
        broll_video: &PathBuf,
        timestamp: f64,
        template: &LayoutTemplate,
        font_path: Option<&PathBuf>,
        output_image: &PathBuf,
    ) -> Result<()> {
        let dummy_filters = AntiFraudConfig {
            enabled: false,
            ..Default::default()
        }
        .generate_instance();

        let (v_filter, _) = FilterGraphBuilder::build(
            template,
            &dummy_filters,
            font_path.map(|p| p.as_path()),
        );

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y")
            .args(["-ss", &format!("{:.2}", timestamp)])
            .args(["-i", &main_video.to_string_lossy()])
            .args(["-ss", "0.0"])
            .args(["-i", &broll_video.to_string_lossy()])
            .args(["-filter_complex", &v_filter])
            .args(["-map", "[vout]"])
            .args(["-vframes", "1"])
            .arg(output_image);

        let output = cmd.output().context("Failed to run ffmpeg frame preview")?;
        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("FFmpeg preview error: {}", err);
        }

        Ok(())
    }

    /// Executes the full multi-layer clip render with optional GPU acceleration and anti-fraud filters.
    pub fn render_clip(job: &RenderJob) -> Result<AppliedFilters> {
        let filters = job.antifraud.generate_instance();
        let (v_filter, a_filter) = FilterGraphBuilder::build(
            &job.template,
            &filters,
            job.font_path.as_deref(),
        );

        let complex_filter = format!("{};{}", v_filter, a_filter);
        let nvenc_supported = Self::is_nvenc_available();

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y")
            .args(["-ss", &format!("{:.2}", job.start_time_secs)])
            .args(["-t", &format!("{:.2}", job.duration_secs)])
            .args(["-i", &job.main_video_path.to_string_lossy()])
            // Looping B-roll layer
            .args(["-stream_loop", "-1"])
            .args(["-t", &format!("{:.2}", job.duration_secs)])
            .args(["-i", &job.broll_video_path.to_string_lossy()])
            .args(["-filter_complex", &complex_filter])
            .args(["-map", "[vout]"])
            .args(["-map", "[aout]"]);

        if nvenc_supported {
            cmd.args(["-c:v", "h264_nvenc"])
                .args(["-preset", "p4"])
                .args(["-cq", "23"]);
        } else {
            cmd.args(["-c:v", "libx264"])
                .args(["-preset", "veryfast"])
                .args(["-crf", "22"]);
        }

        cmd.args(["-c:a", "aac"]).args(["-b:a", "192k"]);

        if filters.strip_metadata {
            cmd.args(["-map_metadata", "-1"]);
        }

        cmd.arg(&job.output_path);

        let output = cmd.output().context("Failed to run ffmpeg render job")?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("FFmpeg render error: {}", err);
        }

        Ok(filters)
    }
}
