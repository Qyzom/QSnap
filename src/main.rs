use log::{error, info};
use qsnap::antifraud::AntiFraudConfig;
use qsnap::config::AppConfig;
use qsnap::media::MediaManager;
use qsnap::render::{RenderEngine, RenderJob};
use qsnap::templates::schema::LayoutTemplate;
use slint::ComponentHandle;
use std::path::PathBuf;

slint::include_modules!();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting QSnap Video Studio & Anti-Fraud Clipper...");

    let app = AppWindow::new()?;
    let config = AppConfig::load();

    // Default paths
    app.set_output_folder_path(config.output_dir.to_string_lossy().to_string().into());

    let app_weak = app.as_weak();

    // Select Source Video Callback
    let weak_clone = app_weak.clone();
    app.on_select_source_video(move || {
        let handle = weak_clone.clone();
        tokio::spawn(async move {
            let file = rfd::AsyncFileDialog::new()
                .add_filter("Video", &["mp4", "mkv", "mov", "avi", "webm"])
                .set_title("Выберите исходный фильм или мультфильм")
                .pick_file()
                .await;

            if let Some(file) = file {
                let path = file.path().to_string_lossy().to_string();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = handle.upgrade() {
                        ui.set_source_video_path(path.into());
                    }
                });
            }
        });
    });

    // Select B-roll Folder Callback
    let weak_clone = app_weak.clone();
    app.on_select_broll_folder(move || {
        let handle = weak_clone.clone();
        tokio::spawn(async move {
            let folder = rfd::AsyncFileDialog::new()
                .set_title("Выберите папку с фоновыми геймплеями (B-roll)")
                .pick_folder()
                .await;

            if let Some(folder) = folder {
                let path = folder.path().to_string_lossy().to_string();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = handle.upgrade() {
                        ui.set_broll_folder_path(path.into());
                    }
                });
            }
        });
    });

    // Start Batch Render Callback
    let weak_clone = app_weak.clone();
    app.on_start_batch_render(move || {
        let ui = match weak_clone.upgrade() {
            Some(u) => u,
            None => return,
        };

        let source_path_str = ui.get_source_video_path().to_string();
        let broll_folder_str = ui.get_broll_folder_path().to_string();
        let banner_text = ui.get_banner_text().to_string();
        let clip_duration = ui.get_clip_duration_secs() as f64;
        let clips_count = ui.get_clips_count() as usize;

        if source_path_str.trim().is_empty() {
            ui.set_status_text("⚠️ Ошибка: выберите исходный видеофайл!".into());
            return;
        }

        let main_path = PathBuf::from(source_path_str);
        if !main_path.exists() {
            ui.set_status_text("⚠️ Файл не найден на диске!".into());
            return;
        }

        ui.set_is_rendering(true);
        ui.set_progress_percent(0.0);
        ui.set_status_text("Подготовка пакета нарезки...".into());

        let output_dir = dirs::download_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("QSnap_Output");
        let _ = std::fs::create_dir_all(&output_dir);

        let ui_handle = weak_clone.clone();

        tokio::spawn(async move {
            info!("Starting batch generation task...");
            let main_duration = MediaManager::probe_duration(&main_path).unwrap_or(300.0);

            // Scan b-roll folder
            let broll_dir = PathBuf::from(&broll_folder_str);
            let brolls = if broll_dir.exists() {
                MediaManager::scan_directory(&broll_dir)
            } else {
                Vec::new()
            };

            let broll_path = if let Some(first) = brolls.first() {
                first.path.clone()
            } else {
                // If no B-roll provided, reuse main video as retention mirror
                main_path.clone()
            };

            let mut template = LayoutTemplate::default();
            template.banner.text = banner_text;

            let antifraud = AntiFraudConfig::default();
            let font_path = PathBuf::from("ui/fonts/Unbounded.ttf");

            let step = (main_duration - clip_duration) / (clips_count as f64).max(1.0);

            for i in 0..clips_count {
                let start_time = (i as f64) * step.max(0.0);
                let out_file = output_dir.join(format!("qsnap_clip_{:02}.mp4", i + 1));

                let job = RenderJob {
                    main_video_path: main_path.clone(),
                    broll_video_path: broll_path.clone(),
                    start_time_secs: start_time,
                    duration_secs: clip_duration,
                    output_path: out_file.clone(),
                    template: template.clone(),
                    antifraud: antifraud.clone(),
                    font_path: if font_path.exists() {
                        Some(font_path.clone())
                    } else {
                        None
                    },
                };

                let cur_idx = i + 1;
                let h = ui_handle.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = h.upgrade() {
                        ui.set_status_text(
                            format!("Рендеринг клипа {} из {}...", cur_idx, clips_count).into(),
                        );
                        ui.set_progress_percent((cur_idx as f32) / (clips_count as f32));
                    }
                });

                // Execute render
                let res = tokio::task::spawn_blocking(move || RenderEngine::render_clip(&job)).await;

                match res {
                    Ok(Ok(filters)) => {
                        info!("Clip {} rendered: speed={:.2}x", cur_idx, filters.speed_factor);
                    }
                    Ok(Err(e)) => {
                        error!("Clip {} failed: {}", cur_idx, e);
                    }
                    Err(join_err) => {
                        error!("Task join error: {}", join_err);
                    }
                }
            }

            // Finish
            let h = ui_handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = h.upgrade() {
                    ui.set_is_rendering(false);
                    ui.set_progress_percent(1.0);
                    ui.set_status_text(
                        format!("🎉 Готово! Все клипы сохранены в {}", output_dir.display()).into(),
                    );
                }
            });
        });
    });

    app.run()?;
    Ok(())
}
