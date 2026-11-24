use anyhow::Result;
use std::path::Path;
use std::process::Command;

pub struct SceneDetector;

impl SceneDetector {
    /// Detects scene change timestamps (in seconds) in a video file using FFmpeg scene filter.
    pub fn detect_scene_changes(video_path: &Path, threshold: f32) -> Result<Vec<f64>> {
        let thresh = threshold.clamp(0.1, 0.9);
        let filter_str = format!("select='gt(scene,{})',metadata=print", thresh);

        let output = Command::new("ffmpeg")
            .arg("-y")
            .args(["-i", &video_path.to_string_lossy()])
            .args(["-filter:v", &filter_str])
            .args(["-f", "null", "-"])
            .output()?;

        let mut scene_points = Vec::new();
        let stderr_str = String::from_utf8_lossy(&output.stderr);

        for line in stderr_str.lines() {
            if line.contains("pts_time:") {
                if let Some(pos) = line.find("pts_time:") {
                    let sub = &line[pos + 9..];
                    let val_str = sub.split_whitespace().next().unwrap_or("");
                    if let Ok(ts) = val_str.parse::<f64>() {
                        scene_points.push(ts);
                    }
                }
            }
        }

        Ok(scene_points)
    }

    /// Slices a timeline into segment boundaries of target duration, snapping to nearest scene changes if available.
    pub fn calculate_clip_ranges(
        total_duration: f64,
        clip_duration: f64,
        max_clips: usize,
        scenes: &[f64],
    ) -> Vec<(f64, f64)> {
        let mut ranges = Vec::new();
        let mut current_start = 0.0;

        for _ in 0..max_clips {
            if current_start + 5.0 >= total_duration {
                break;
            }

            let ideal_end = (current_start + clip_duration).min(total_duration);

            // Find nearest scene transition near ideal_end within +/- 4.0 seconds
            let snapped_end = scenes
                .iter()
                .filter(|&&s| s > current_start + 10.0 && (s - ideal_end).abs() <= 4.0)
                .min_by(|&&a, &&b| {
                    (a - ideal_end)
                        .abs()
                        .partial_cmp(&(b - ideal_end).abs())
                        .unwrap()
                })
                .copied()
                .unwrap_or(ideal_end);

            let dur = snapped_end - current_start;
            ranges.push((current_start, dur));
            current_start = snapped_end;
        }

        ranges
    }
}
