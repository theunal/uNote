# uNote → Tauri Geçiş Prompt'u

**Rol:** Sen kıdemli bir Rust + Tauri geliştiricisisin. Mevcut egui/eframe tabanlı `uNote` uygulamasını Tauri v2'ye taşıyacaksın.

**Proje bağlamı:** Şu anki proje `C:\Users\UNAL\Desktop\uNote` klasöründe. Tüm backend mantığı Rust modüllerinde, tamamı taşınabilir ve **değişmeden yeniden kullanılacak**:
- `src/crypto.rs` — AES-256-GCM şifreleme (`encrypt`/`decrypt`)
- `src/db.rs` — SQLite CRUD (`get_db_path`, `init_db`, `load_notes`, `insert_note`, `update_note_content`, `update_note_title`, `delete_note`)
- `src/settings.rs` — JSON ayarlar (tema, font boyutu, kelime kaydırma, biçimlendirme, açık sekmeler)
- `src/models.rs` — `Note`, `NoteTab`, `AppTheme`, `note_avatar_color` veri tipleri

Bu dört dosyayı **Tauri command'ları olarak sarmala**, mantığı aynen koru. Sadece `src/app.rs` (egui UI) ve `src/main.rs` (eframe başlatma) atılacak ve yerine HTML/JS frontend + Tauri komutları gelecek.

**Hedef kurulum:**
- Tauri v2 + Rust backend
- Vanilla HTML/CSS/JS frontend (framework yok, basit tut) veya tercihen Vite + vanilla
- DB ve ayarlar mevcut `.db`/`.json` formatıyla aynı yerde kalsın (uyumluluk)
- Windows'ta `window-vibrancy` (acrylic) ve özel titlebar (decorations: false) korunacak

**Tasarım kaynağı:** `unote-figma-design.html` dosyası UI'ın **tam görsel spesifikasyonu**. Bu dosyadaki:
1. CSS değişkenlerini (`:root` light/dark tema renkleri, `--radius`, `--shadow`, fontlar) frontend'in `styles.css`'ine birebir taşı
2. Bileşen yapılarını (titlebar, tab bar, sidebar/note-list, editor, settings, statusbar, menu, about dialog, context menu, avatar renk paleti) aynen uygula
3. Tüm Türkçe metinleri kullan ("Notlar", "Notlarda ara...", "Ayarlar", "Hakkında", "Notu Sil", "Otomatik kaydedildi", "Güvenli Not Alma Kasası", vb.)
4. `data-theme="light"/"dark"` sistemi + Açık/Koyu/Sistem üçlü toggle'ı koru

**Uygulamanın özellikleri (taşınması gerekenler):**
- Not listesi sidebar + avatar renkleri, arama (title+content'ta filtreleme)
- Sekmeli editör (tab bar), sekme sürükle-bırak yeniden sıralama, sekme adı düzenleme, kapatma
- Otomatik kaydetme (her input değişiminde DB'ye yaz)
- Kilitli notlar (şifreyle decrypt; şifre boşsa düz metin)
- Ayarlar: tema, font boyutu slider (8-24), kelime kaydırma, zengin metin, başlangıçta sekmeleri geri yükle
- Uygulama menüsü (▾), bağlam menüsü (sağ tık → Notu Sil), Hakkında popup
- Status bar (sürüm, not sayısı, aktif not, tema göstergesi)

**Dosya yapısı (oluştur):**
- Tauri v2 projesi (`src-tauri/` + frontend klasörü)
- Rust tarafında `crypto.rs`, `db.rs`, `settings.rs`, `models.rs`'i `#[tauri::command]` fonksiyonlarına sarmala: `list_notes`, `create_note`, `get_note`, `save_note_content`, `save_note_title`, `delete_note`, `get_settings`, `save_settings`
- `tauri.conf.json` — `decorations: false`, pencere 1000x650, min 600x400

**Önemli kısıtlar:**
- Ses/video kaydı bu sürümde ekleme, sadece mevcut not işlevlerini taşı
- Şifreleme anahtarı türetme mantığını (32-byte truncate) aynen koru — veri uyumluluğu kritik
- Otomatik kaydetme için debounce (örn. 300ms) kullan, her tuş vuruşunda IPC spam yapma
- UI'ı `unote-figma-design.html`'deki gibi pixel-perfect yap

**Tamamlandığında şunları doğrula:**
1. `cargo build --release` ve `npm run build` / `tauri build` hatasız geçiyor
2. Uygulama açılınca eski `.db`'deki notlar görünüyor (geriye dönük uyumluluk)
3. Tüm etkileşimler (yeni not, silme, arama, sekmeler, tema, ayarlar) çalışıyor

Başla.