use rusqlite::{Connection, params};

use crate::crypto;
use crate::models::Note;

pub fn get_db_path() -> std::path::PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("unote");
    std::fs::create_dir_all(&path).ok();
    path.push("notes.db");
    path
}

pub fn init_db(db: &Connection) -> rusqlite::Result<()> {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            content TEXT,
            is_locked INTEGER DEFAULT 0,
            updated_at TEXT
        )",
    )
}

pub fn load_notes(
    db: &Connection,
    search_query: &str,
    master_password: &str,
) -> rusqlite::Result<Vec<Note>> {
    let mut stmt = db.prepare(
        "SELECT id, title, content, is_locked, updated_at \
         FROM notes ORDER BY updated_at DESC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            is_locked: row.get::<_, i32>(3)? != 0,
            updated_at: row.get(4)?,
        })
    })?;
    let notes: Vec<Note> = rows.filter_map(|r| r.ok()).collect();

    if search_query.is_empty() {
        return Ok(notes);
    }

    let q = search_query.to_lowercase();
    Ok(notes
        .into_iter()
        .filter(|n| {
            let display_title = if n.is_locked {
                crypto::decrypt(&n.title, master_password).unwrap_or_default()
            } else {
                n.title.clone()
            };
            let display_content = if n.is_locked {
                crypto::decrypt(&n.content, master_password).unwrap_or_default()
            } else {
                n.content.clone()
            };
            display_title.to_lowercase().contains(&q)
                || display_content.to_lowercase().contains(&q)
        })
        .collect())
}

pub fn insert_note(
    db: &Connection,
    title: &str,
    content: &str,
    is_locked: bool,
    updated_at: &str,
) -> rusqlite::Result<()> {
    db.execute(
        "INSERT INTO notes (title, content, is_locked, updated_at) VALUES (?1, ?2, ?3, ?4)",
        params![title, content, is_locked as i32, updated_at],
    )?;
    Ok(())
}

pub fn update_note_content(
    db: &Connection,
    content: &str,
    updated_at: &str,
    id: i64,
) -> rusqlite::Result<()> {
    db.execute(
        "UPDATE notes SET content = ?1, updated_at = ?2 WHERE id = ?3",
        params![content, updated_at, id],
    )?;
    Ok(())
}

pub fn update_note_title(
    db: &Connection,
    title: &str,
    updated_at: &str,
    id: i64,
) -> rusqlite::Result<()> {
    db.execute(
        "UPDATE notes SET title = ?1, updated_at = ?2 WHERE id = ?3",
        params![title, updated_at, id],
    )?;
    Ok(())
}

pub fn delete_note(db: &Connection, id: i64) -> rusqlite::Result<()> {
    db.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
    Ok(())
}