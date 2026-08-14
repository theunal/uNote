# uNote Rust — Agent Guide

## Project
Single-crate Rust desktop app. A secure, encrypted note-taking vault with egui GUI.

## Key Commands
- **Run:** `cargo run --release`
- **Build (debug):** `cargo build`
- **Build (release):** `cargo build --release`

No test, lint, CI, or formatter config exists. The only local config is `.gitignore` (just `/target`).

## Architecture
- **Entrypoint:** `src/main.rs` — just `main()`, starts eframe, applies DWM rounded corners + acrylic vibrancy
- **Modules:**
  - `src/app.rs` — `NoteApp` struct, state, `eframe::App::update()` with all UI panels
  - `src/db.rs` — SQLite CRUD (init, load, insert, update, delete)
  - `src/crypto.rs` — AES-256-GCM encrypt/decrypt
  - `src/models.rs` — `Note`, `NoteTab`, `AppTheme` data types
  - `src/settings.rs` — persistent user preferences (theme, font size, word wrap, open tabs)
  - `src/theme.rs` — Win11 Fluent Design color palette, style builder, helper functions
- **GUI:** egui/eframe 0.31. Custom title bar (no native decorations) + SidePanel (note list) + TopBottomPanel (tab bar) + CentralPanel (editor) + StatusBar
- **Theme:** Light/Dark/System toggle via settings. Default light. All colors centralized in `theme.rs`.
- **DB:** SQLite via rusqlite (bundled feature compiles SQLite from source). DB file is `<exe_name>.db` next to the binary.
- **Crypto:** AES-256-GCM via `aes-gcm` crate. Random 12-byte nonce generated via `rand::random::<[u8; 12]>()`. Key derived by truncating password to 32 bytes.
- **Auto-save:** Editor saves to DB on each `TextEdit::changed()` event.
- **Settings persistence:** JSON file at `<exe_name>.json` next to the binary. Stores theme, open tab IDs, font size, word wrap, formatting, restore tabs.
- **Platform:** Windows 11 acrylic vibrancy via `window-vibrancy` crate, DWM rounded corners via `windows` crate. HWND obtained from `raw-window-handle` 0.6 on `CreationContext`.

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
