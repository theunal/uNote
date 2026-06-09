use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::models::AppTheme;

#[derive(Serialize, Deserialize)]
struct Settings {
    theme: AppTheme,
    #[serde(default)]
    open_tab_ids: Vec<i64>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: AppTheme::Light,
            open_tab_ids: Vec::new(),
        }
    }
}

fn settings_path() -> PathBuf {
    let mut path = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("uNote-rust"));
    path.set_extension("json");
    path
}

fn load_settings() -> Settings {
    let path = settings_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_settings(settings: &Settings) {
    let path = settings_path();
    if let Ok(content) = serde_json::to_string_pretty(settings) {
        let _ = std::fs::write(path, content);
    }
}

pub fn load_theme() -> AppTheme {
    load_settings().theme
}

#[allow(dead_code)]
pub fn save_theme(theme: AppTheme) {
    let mut settings = load_settings();
    settings.theme = theme;
    save_settings(&settings);
}

pub fn load_open_tab_ids() -> Vec<i64> {
    load_settings().open_tab_ids
}

pub fn save_open_tab_ids(ids: &[i64]) {
    let mut settings = load_settings();
    settings.open_tab_ids = ids.to_vec();
    save_settings(&settings);
}
