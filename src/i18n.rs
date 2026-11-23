use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub app_title: String,
    pub tab_templates: String,
    pub tab_media: String,
    pub tab_antifraud: String,
    pub tab_generator: String,
    pub tab_logs: String,
    pub select_main_video: String,
    pub select_broll_dir: String,
    pub banner_text: String,
    pub clip_duration: String,
    pub clips_count: String,
    pub start_batch: String,
    pub status_idle: String,
    pub status_rendering: String,
    pub status_done: String,
}

impl Default for Translation {
    fn default() -> Self {
        Self {
            app_title: "QSnap • Viral Split-Screen Video Studio".to_string(),
            tab_templates: "Шаблоны".to_string(),
            tab_media: "Медиатека".to_string(),
            tab_antifraud: "Антифрод".to_string(),
            tab_generator: "Генерация серии".to_string(),
            tab_logs: "Логи".to_string(),
            select_main_video: "Выбрать исходное видео".to_string(),
            select_broll_dir: "Папка с геймплеем (B-roll)".to_string(),
            banner_text: "Текст рекламного баннера".to_string(),
            clip_duration: "Длина ролика (сек)".to_string(),
            clips_count: "Количество клипов".to_string(),
            start_batch: "Запустить нарезку серии".to_string(),
            status_idle: "Готов к работе".to_string(),
            status_rendering: "Рендеринг...".to_string(),
            status_done: "Готово! Все ролики сохранены".to_string(),
        }
    }
}
