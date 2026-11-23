pub mod schema;

use anyhow::Result;
use schema::LayoutTemplate;
use std::fs;
use std::path::PathBuf;

pub struct TemplateManager {
    storage_dir: PathBuf,
}

impl TemplateManager {
    pub fn new() -> Result<Self> {
        let base_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("qsnap")
            .join("templates");

        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)?;
        }

        Ok(Self {
            storage_dir: base_dir,
        })
    }

    pub fn list_templates(&self) -> Vec<LayoutTemplate> {
        let mut templates = vec![LayoutTemplate::default()];

        if let Ok(entries) = fs::read_dir(&self.storage_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(tmpl) = serde_json::from_str::<LayoutTemplate>(&content) {
                            if tmpl.id != "default_split" {
                                templates.push(tmpl);
                            }
                        }
                    }
                }
            }
        }

        templates
    }

    pub fn save_template(&self, template: &LayoutTemplate) -> Result<()> {
        let file_path = self.storage_dir.join(format!("{}.json", template.id));
        let data = serde_json::to_string_pretty(template)?;
        fs::write(file_path, data)?;
        Ok(())
    }

    pub fn delete_template(&self, id: &str) -> Result<()> {
        let file_path = self.storage_dir.join(format!("{}.json", id));
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }
}
