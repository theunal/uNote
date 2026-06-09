# uNote Rust — Agent Guide

## Project
Single-crate Rust desktop app. A secure, encrypted note-taking vault with egui GUI.

## Key Commands
- **Run:** `cargo run --release`
- **Build (debug):** `cargo build`
- **Build (release):** `cargo build --release`

No test, lint, CI, or formatter config exists. The only local config is `.gitignore` (just `/target`).

## Architecture
- **Entrypoint:** `src/main.rs` — just `main()`, starts eframe
- **Modules:**
  - `src/app.rs` — `NoteApp` struct, state, `eframe::App::update()` with all UI panels
  - `src/db.rs` — SQLite CRUD (init, load, insert, update, delete)
  - `src/crypto.rs` — AES-256-GCM encrypt/decrypt
  - `src/models.rs` — `Note`, `NoteTab`, `AppTheme` data types
- **GUI:** egui/eframe 0.31. Menu bar (File/View/Help) + SidePanel (note list) + TopBottomPanel (tab bar) + CentralPanel (editor) + StatusBar
- **Theme:** Light/Dark toggle via View menu. Default light (Notepad-style).
- **DB:** SQLite via rusqlite (bundled feature compiles SQLite from source). DB file is `<exe_name>.db` next to the binary.
- **Crypto:** AES-256-GCM via `aes-gcm` crate. Random 12-byte nonce generated via `rand::random::<[u8; 12]>()`. Key derived by truncating password to 32 bytes.
- **Auto-save:** Editor saves to DB on each `TextEdit::changed()` event.

## Gotchas
- `rand::rngs::OsRng` does not expose `fill_bytes` in rand 0.9 on this platform — use `rand::random::<[u8; N]>()` instead for nonce generation.
- SQLite DB is named after the executable with `.db` extension (e.g., `uNote-rust.exe` → `uNote-rust.exe.db`).
- Note titles are stored encrypted in DB when `is_locked` is set; they are decrypted in-memory for display.
- egui panels use action-flags pattern (not captured closures) to avoid borrow conflicts — all mutations deferred until after panel closes.

## DB Schema
```sql
notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT, content TEXT,
  is_locked INTEGER DEFAULT 0,
  updated_at TEXT
)
```
