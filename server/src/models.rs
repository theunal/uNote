#[derive(Clone)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub is_locked: bool,
    pub updated_at: String,
}

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppTheme {
    Light,
    Dark,
    System,
}

#[allow(dead_code)]
pub fn note_avatar_color(index: usize) -> String {
    const PALETTE: [&str; 8] = [
        "#C03A9B", "#3A9BC0", "#7B5BA7", "#C07A3A", "#3AC08A", "#C0505A", "#5A9BC0", "#9B7BC0",
    ];
    PALETTE[index % 8].to_string()
}