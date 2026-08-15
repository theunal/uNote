mod crypto;
mod db;
mod models;
mod settings;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

use models::Note;

#[derive(Serialize)]
struct NoteDto {
    id: i64,
    title: String,
    content: String,
    is_locked: bool,
    updated_at: String,
}

#[derive(Deserialize)]
struct SaveContentArgs {
    id: i64,
    content: String,
    is_locked: bool,
    password: String,
}

#[derive(Deserialize)]
struct SaveTitleArgs {
    id: i64,
    title: String,
    is_locked: bool,
    password: String,
}

#[derive(Deserialize)]
struct ListArgs {
    #[serde(default)]
    query: String,
    #[serde(default)]
    password: String,
}

struct AppState {
    db: Mutex<Connection>,
}

fn to_dto(n: &Note, password: &str) -> NoteDto {
    let title = if n.is_locked {
        crypto::decrypt(&n.title, password).unwrap_or_default()
    } else {
        n.title.clone()
    };
    let content = if n.is_locked {
        "* Bu not kilitli.".to_string()
    } else {
        n.content.clone()
    };
    NoteDto {
        id: n.id,
        title,
        content,
        is_locked: n.is_locked,
        updated_at: n.updated_at.clone(),
    }
}

#[tauri::command]
fn list_notes(args: ListArgs, state: State<AppState>) -> Result<Vec<NoteDto>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let notes = db::load_notes(&db, &args.query, &args.password).map_err(|e| e.to_string())?;
    Ok(notes.iter().map(|n| to_dto(n, &args.password)).collect())
}

#[tauri::command]
fn create_note(_password: String, state: State<AppState>) -> Result<i64, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    db::insert_note(&db, "Yeni Not", "", false, &now).map_err(|e| e.to_string())?;
    // Return last inserted id
    db.query_row("SELECT last_insert_rowid()", [], |r| r.get(0))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_note(id: i64, password: String, state: State<AppState>) -> Result<Option<NoteDto>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let notes = db::load_notes(&db, "", &password).map_err(|e| e.to_string())?;
    Ok(notes
        .iter()
        .find(|n| n.id == id)
        .map(|n| to_dto(n, &password)))
}

#[tauri::command]
fn save_note_content(args: SaveContentArgs, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let final_content = if args.is_locked {
        crypto::encrypt(&args.content, &args.password)
    } else {
        args.content
    };
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    db::update_note_content(&db, &final_content, &now, args.id).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_note_title(args: SaveTitleArgs, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let final_title = if args.is_locked {
        crypto::encrypt(&args.title, &args.password)
    } else {
        args.title
    };
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    db::update_note_title(&db, &final_title, &now, args.id).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_note(id: i64, state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db::delete_note(&db, id).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_fonts() -> Vec<String> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::Graphics::DirectWrite::{
            DWRITE_FACTORY_TYPE_SHARED, DWriteCreateFactory, IDWriteFactory,
        };

        let factory: IDWriteFactory =
            match unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED) } {
                Ok(f) => f,
                Err(_) => return Vec::new(),
            };
        let mut collection: Option<windows::Win32::Graphics::DirectWrite::IDWriteFontCollection> =
            None;
        if unsafe { factory.GetSystemFontCollection(&mut collection, true) }.is_err() {
            return Vec::new();
        }
        let Some(collection) = collection else {
            return Vec::new();
        };

        let mut fonts = Vec::new();
        let count = unsafe { collection.GetFontFamilyCount() };
        for i in 0..count {
            let family = match unsafe { collection.GetFontFamily(i) } {
                Ok(f) => f,
                Err(_) => continue,
            };
            let names = match unsafe { family.GetFamilyNames() } {
                Ok(n) => n,
                Err(_) => continue,
            };
            // Prefer English name, fall back to first locale
            let mut index: u32 = 0;
            let mut exists: windows::core::BOOL = false.into();
            unsafe {
                let en = windows::core::PCWSTR(windows::core::w!("en-us").as_ptr());
                let _ = names.FindLocaleName(en, &mut index, &mut exists);
                if !bool::from(exists) {
                    index = 0;
                }
            }
            let len = match unsafe { names.GetStringLength(index) } {
                Ok(l) => l,
                Err(_) => continue,
            };
            if len == 0 {
                continue;
            }
            let mut buf = vec![0u16; len as usize + 1];
            unsafe {
                let _ = names.GetString(index, &mut buf);
            }
            let name = String::from_utf16_lossy(&buf[..len as usize]);
            if !name.trim().is_empty() && !fonts.contains(&name) {
                fonts.push(name);
            }
        }
        fonts.sort();
        fonts
    }
    #[cfg(not(target_os = "windows"))]
    {
        vec![
            "Arial".into(),
            "Calibri".into(),
            "Courier New".into(),
            "Georgia".into(),
            "Segoe UI".into(),
        ]
    }
}

#[tauri::command]
fn get_settings() -> settings::Settings {
    settings::load_settings()
}

#[tauri::command]
fn save_settings(s: settings::Settings) {
    settings::save_settings(&s);
}

fn apply_windows_effects(handle: &tauri::AppHandle, dark: bool) {
    #[cfg(target_os = "windows")]
    {
        use tauri::Manager;
        if let Some(window) = handle.get_webview_window("main") {
            use window_vibrancy::apply_acrylic;
            let alpha: u8 = if dark { 195 } else { 216 };
            let color = if dark {
                (32u8, 32u8, 32u8, alpha)
            } else {
                (243u8, 243u8, 243u8, alpha)
            };
            let _ = apply_acrylic(&window, Some(color));
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = db::get_db_path();
    let conn = Connection::open(&db_path).expect("Failed to open database");
    db::init_db(&conn).expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            db: Mutex::new(conn),
        })
        .setup(|app| {
            let settings = settings::load_settings();
            let dark = matches!(settings.theme, models::AppTheme::Dark);
            apply_windows_effects(app.handle(), dark);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_notes,
            create_note,
            get_note,
            save_note_content,
            save_note_title,
            delete_note,
            get_settings,
            save_settings,
            list_fonts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
