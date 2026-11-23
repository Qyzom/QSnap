pub mod filtergraph;

use crate::antifraud::{AntiFraudConfig, AppliedFilters};
use crate::templates::schema::LayoutTemplate;
use anyhow::{Context, Result};
use filtergraph::FilterGraphBuilder;
use std::path::PathBuf;
use std::process::Command;

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
    pub fn render_clip(job: &RenderJob) -> Result<AppliedFilters> {
        let filters = job.antifraud.generate_instance();
        let (v_filter, a_filter) = FilterGraphBuilder::build(
            &job.template,
            &filters,
            job.font_path.as_deref(),
        );

        let complex_filter = format!("{};{}", v_filter, a_filter);

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y") // Overwrite output
            .args(["-ss", &format!("{:.2}", job.start_time_secs)])
            .args(["-t", &format!("{:.2}", job.duration_secs)])
            .args(["-i", &job.main_video_path.to_string_lossy()])
            // B-roll looping
            .args(["-stream_loop", "-1"])
            .args(["-t", &format!("{:.2}", job.duration_secs)])
            .args(["-i", &job.broll_video_path.to_string_lossy()])
            .args(["-filter_complex", &complex_filter])
            .args(["-map", "[vout]"])
            .args(["-map", "[aout]"])
            .args(["-c:v", "libx264"])
            .args(["-preset", "veryfast"])
            .args(["-crf", "22"])
            .args(["-c:a", "aac"])
            .args(["-b:a", "192k"]);

        if filters.strip_metadata {
            cmd.args(["-map_metadata", "-1"]);
        }

        cmd.arg(&job.output_path);

        let output = cmd.output().context("Failed to run ffmpeg command")?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("FFmpeg render error: {}", err);
        }

        Ok(filters)
    }
}
