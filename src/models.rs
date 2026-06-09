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

pub fn note_avatar_color(index: usize, is_dark: bool) -> (Color32, Color32) {
    const LIGHT: [(Color32, Color32); 8] = [
        (
            Color32::from_rgb(208, 228, 255),
            Color32::from_rgb(26, 61, 124),
        ),
        (
            Color32::from_rgb(208, 255, 208),
            Color32::from_rgb(26, 107, 26),
        ),
        (
            Color32::from_rgb(255, 224, 192),
            Color32::from_rgb(139, 69, 19),
        ),
        (
            Color32::from_rgb(232, 208, 255),
            Color32::from_rgb(74, 26, 124),
        ),
        (
            Color32::from_rgb(255, 208, 208),
            Color32::from_rgb(124, 26, 26),
        ),
        (
            Color32::from_rgb(255, 224, 176),
            Color32::from_rgb(107, 58, 26),
        ),
        (
            Color32::from_rgb(208, 255, 252),
            Color32::from_rgb(26, 92, 90),
        ),
        (
            Color32::from_rgb(255, 208, 232),
            Color32::from_rgb(124, 26, 74),
        ),
    ];
    const DARK: [(Color32, Color32); 8] = [
        (
            Color32::from_rgb(30, 50, 80),
            Color32::from_rgb(150, 200, 255),
        ),
        (
            Color32::from_rgb(26, 60, 26),
            Color32::from_rgb(150, 230, 150),
        ),
        (
            Color32::from_rgb(70, 45, 20),
            Color32::from_rgb(255, 200, 140),
        ),
        (
            Color32::from_rgb(45, 20, 70),
            Color32::from_rgb(200, 150, 255),
        ),
        (
            Color32::from_rgb(70, 20, 20),
            Color32::from_rgb(255, 150, 150),
        ),
        (
            Color32::from_rgb(55, 35, 15),
            Color32::from_rgb(255, 190, 130),
        ),
        (
            Color32::from_rgb(15, 50, 48),
            Color32::from_rgb(150, 230, 225),
        ),
        (
            Color32::from_rgb(60, 20, 40),
            Color32::from_rgb(255, 150, 200),
        ),
    ];
    let palette = if is_dark { &DARK } else { &LIGHT };
    palette[index % 8]
}
