use eframe::egui::Color32;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub is_locked: bool,
    pub updated_at: String,
}

#[derive(Clone)]
pub struct NoteTab {
    pub note_id: i64,
    pub title: String,
    pub content: String,
    pub is_locked: bool,
}

#[derive(PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppTheme {
    Light,
    Dark,
    System,
}

impl AppTheme {
    pub fn is_dark(&self, ctx: &eframe::egui::Context) -> bool {
        match self {
            AppTheme::Light => false,
            AppTheme::Dark => true,
            AppTheme::System => matches!(ctx.system_theme(), Some(eframe::egui::Theme::Dark)),
        }
    }
}

pub fn note_avatar_color(index: usize, _is_dark: bool) -> (Color32, Color32) {
    const PALETTE: [(Color32, Color32); 8] = [
        (Color32::from_rgb(192, 58, 155), Color32::from_rgb(255, 255, 255)),
        (Color32::from_rgb(58, 155, 192), Color32::from_rgb(255, 255, 255)),
        (Color32::from_rgb(123, 91, 167), Color32::from_rgb(255, 255, 255)),
        (Color32::from_rgb(192, 122, 58), Color32::from_rgb(255, 255, 255)),
        (Color32::from_rgb(58, 192, 138), Color32::from_rgb(255, 255, 255)),
        (Color32::from_rgb(192, 80, 90), Color32::from_rgb(255, 255, 255)),
        (Color32::from_rgb(90, 155, 192), Color32::from_rgb(255, 255, 255)),
        (Color32::from_rgb(155, 123, 192), Color32::from_rgb(255, 255, 255)),
    ];
    PALETTE[index % 8]
}
