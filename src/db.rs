use rusqlite::{params, Connection};

use crate::crypto;
use crate::models::Note;

pub fn get_db_path() -> std::path::PathBuf {
    let mut path =
        std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("uNote-rust"));
    path.set_extension("db");
    path
}

pub fn init_db(db: &Connection) {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            content TEXT,
            is_locked INTEGER DEFAULT 0,
            updated_at TEXT
        )",
    )
    .expect("Failed to init DB");
}

pub fn load_notes(db: &Connection, search_query: &str, master_password: &str) -> Vec<Note> {
    let mut stmt = db
        .prepare(
            "SELECT id, title, content, is_locked, updated_at \
             FROM notes ORDER BY updated_at DESC",
        )
        .expect("query failed");

    stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            is_locked: row.get::<_, i32>(3)? != 0,
            updated_at: row.get(4)?,
        })
    })
    .expect("query_map failed")
    .filter_map(|r| r.ok())
    .filter(|n| {
        if search_query.is_empty() {
            return true;
        }
        let display_title = if n.is_locked {
            crypto::decrypt(&n.title, master_password)
        } else {
            n.title.clone()
        };
        let display_content = if n.is_locked {
            crypto::decrypt(&n.content, master_password)
        } else {
            n.content.clone()
        };
        let q = search_query.to_lowercase();
        display_title.to_lowercase().contains(&q) || display_content.to_lowercase().contains(&q)
    })
    .collect()
}

pub fn insert_note(db: &Connection, title: &str, content: &str, is_locked: bool, updated_at: &str) {
    db.execute(
        "INSERT INTO notes (title, content, is_locked, updated_at) VALUES (?1, ?2, ?3, ?4)",
        params![title, content, is_locked as i32, updated_at],
    )
    .expect("insert failed");
}

pub fn update_note_content(db: &Connection, content: &str, updated_at: &str, id: i64) {
    db.execute(
        "UPDATE notes SET content = ?1, updated_at = ?2 WHERE id = ?3",
        params![content, updated_at, id],
    )
    .expect("update failed");
}

pub fn update_note_title(db: &Connection, title: &str, updated_at: &str, id: i64) {
    db.execute(
        "UPDATE notes SET title = ?1, updated_at = ?2 WHERE id = ?3",
        params![title, updated_at, id],
    )
    .expect("update failed");
}

pub fn delete_note(db: &Connection, id: i64) {
    db.execute("DELETE FROM notes WHERE id = ?1", params![id])
        .expect("delete failed");
}
