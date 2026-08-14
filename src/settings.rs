use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::models::AppTheme;

#[derive(Serialize, Deserialize)]
struct Settings {
    theme: AppTheme,
    #[serde(default)]
    open_tab_ids: Vec<i64>,
    #[serde(default = "default_font_size")]
    font_size: f32,
    #[serde(default = "default_true")]
    word_wrap: bool,
    #[serde(default)]
    formatting_enabled: bool,
    #[serde(default = "default_true")]
    restore_tabs: bool,
}

fn default_font_size() -> f32 { 14.0 }
fn default_true() -> bool { true }

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: AppTheme::Light,
            open_tab_ids: Vec::new(),
            font_size: 14.0,
            word_wrap: true,
            formatting_enabled: false,
            restore_tabs: true,
        }
    }
}

fn settings_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("uNote-rust"));
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

pub fn load_font_size() -> f32 {
    load_settings().font_size
}

pub fn save_font_size(size: f32) {
    let mut settings = load_settings();
    settings.font_size = size;
    save_settings(&settings);
}

pub fn load_word_wrap() -> bool {
    load_settings().word_wrap
}

pub fn save_word_wrap(wrap: bool) {
    let mut settings = load_settings();
    settings.word_wrap = wrap;
    save_settings(&settings);
}

pub fn load_formatting_enabled() -> bool {
    load_settings().formatting_enabled
}

pub fn save_formatting_enabled(enabled: bool) {
    let mut settings = load_settings();
    settings.formatting_enabled = enabled;
    save_settings(&settings);
}

pub fn load_restore_tabs() -> bool {
    load_settings().restore_tabs
}

pub fn save_restore_tabs(restore: bool) {
    let mut settings = load_settings();
    settings.restore_tabs = restore;
    save_settings(&settings);
}
