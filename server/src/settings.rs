use serde::{Deserialize, Serialize};

use crate::models::AppTheme;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Settings {
    pub theme: AppTheme,
    pub open_tab_ids: Vec<i64>,
    pub font_family: String,
    pub font_style: String,
    pub font_size: f32,
    pub word_wrap: bool,
    pub formatting_enabled: bool,
    pub restore_tabs: bool,
    pub open_files_mode: String,
    pub recent_files: bool,
    pub spell_check: bool,
    pub autocorrect: bool,
    pub writing_tools: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: AppTheme::Light,
            open_tab_ids: Vec::new(),
            font_family: "Space Mono".into(),
            font_style: "Regular".into(),
            font_size: 14.0,
            word_wrap: true,
            formatting_enabled: false,
            restore_tabs: true,
            open_files_mode: "new".into(),
            recent_files: true,
            spell_check: true,
            autocorrect: true,
            writing_tools: true,
        }
    }
}

fn settings_path() -> std::path::PathBuf {
    let mut path =
        std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("unote-tauri"));
    path.set_extension("json");
    path
}

pub fn load_settings() -> Settings {
    let path = settings_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_settings(settings: &Settings) {
    let path = settings_path();
    if let Ok(content) = serde_json::to_string_pretty(settings) {
        let _ = std::fs::write(path, content);
    }
}
