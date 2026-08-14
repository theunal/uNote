# uNote — UI Tasarım Rehberi (AI için)

Bu doküman, mevcut Rust + egui kodundan çıkarılmıştır. Amacı, bir yapay zekânın (veya tasarımcının) bu masaüstü uygulamasının kullanıcı arayüzünü anlayıp yeniden tasarlayabilmesidir.

## 1. Uygulama Nedir?

**uNote** — güvenli, şifreli bir not alma kasası. Windows 11 tarzı, modern "Fluent Design" görünümünde bir masaüstü not defteri. Araç çubuğu yok (custom title bar). Akrilik (acrylic) yarı saydam cam efekti ve yuvarlatılmış köşeler kullanır.

- **Dil:** Rust, **GUI:** egui/eframe 0.31
- **Pencere:** 1000×650 başlangıç, min 600×400. Çerçevesiz + şeffaf + yuvarlatılmış köşeler.
- **Tek ekran, çok panelli** düzen (tab-sekme not defteri mantığı).

## 2. Genel Yerleşim (Layout)

Ekran dikey olarak 3 ana bölgeye + 1 kenar paneline bölünür:

```
┌────────────────────────────────────────────────────────┐
│ ÜST PANEL — Sekme Çubuğu (Tab Bar)                     │
│ [sekme1] [sekme2] [+] [▾]        [arama...]      [X]   │
├──────────┬─────────────────────────────────────────────┤
│ SOL PANEL│  MERKEZ PANEL (Editör / Ayarlar)            │
│ (Not     │                                              │
│  Listesi)│                                              │
│          │                                              │
│          │                                              │
├──────────┴─────────────────────────────────────────────┤
│ ALT PANEL — Durum Çubuğu (Status Bar)                  │
└────────────────────────────────────────────────────────┘
```

- **Üst panel:** pencere sürükleme alanı (kendi title bar'ı), sekmeler, + butonu, menü (▾), arama, kapat (X).
- **Sol panel:** hamburger butonu + not listesi (geniş) veya sadece avatar küçük ikonları (daraltılmış).
- **Merkez panel:** seçili sekmenin içeriği — not editörü veya Ayarlar.
- **Alt panel:** durum bilgisi (sürüm, not sayısı, seçili sekme adı, tema göstergesi).

## 3. Sayfalar / Ekranlar / Paneller (Ayrıntılı)

### 3.1 Üst Panel — Sekme Çubuğu
- **Sekmeler:** Birden çok açık not sekmesi. Aktif sekme diğerlerinden farklı renkli (surface_hover) ve köşeleri üstte yuvarlatılmış. Sekme başlığı 15 karakterden uzunsa "…" ile kısaltılır.
  - Sekmeye tıkla → seç.
  - Aktif sekmeye tıkla → **başlığı yeniden adlandır** (inline metin kutusu; Enter kaydeder, Esc iptal).
  - Sekmedeki "X" → kapat. (İçerik boşsa not silinir.)
  - Sekmeyi **sürükle-bırak** → sırasını değiştir.
- **+ butonu:** Yeni not oluşturur ve açar.
- **▾ (chevron) butonu:** Aşağı açılır menüyü açar (Ayarlar, Hakkında).
- **Arama kutusu:** "🔍 Notlarda ara..." placeholder. Yazdıkça not listesini filtreler (başlık + içerik).
- **X (kapat) butonu:** Uygulamayı kapatır; üzerine gelince kırmızı arka plan.

### 3.2 Sol Panel — Not Listesi
- **Hamburger butonu:** Paneli daralt/ genişlet (animasyonlu). Daraltılınca sadece 3 karakterlik renkli **avatar** kareleri gösterilir; genişleyince tam başlıklar.
- **Geniş mod:** Her not satırı (yükseklik 26px):
  - Aktif seçili not: mavi seçim rengi (selection).
  - Hover: yumuşak gri (surface_hover).
  - Kilitli notların başında 🔒 ikonu.
- **Sağ tık (not üzerinde):** bağlam menüsü açılır (aşağıda).
- Not yoksa: "Henüz not yok" mesajı (geniş) / "-" (daraltılmış).

### 3.3 Merkez Panel — İki Durum

**A) Not Editörü (varsayılan):**
- Çok satırlı metin düzenleyici, monospace font (Consolas), font boyutu ayarlanabilir (8–24px, varsayılan 11).
- **Otomatik kayıt:** Her yazım değişikliğinde anında DB'ye kaydedilir.
- **Satır kaydırma (word wrap)** açık/kapalı olabilir.
- **Biçimlendirme (rich text)** kapalıysa `code_editor` moduna geçer (düz kod editörü).
- **Not seçili değilse:** boş merkezde büyük "uNote" başlığı + "Bir not seçin veya yeni bir not oluşturun" mesajı.

**B) Ayarlar Sayfası (özel sekme, not_id = -1):**
- **Tema:** Açık ☀ / Koyu 🌙 / Sistem 🖥 — üç seçmeli buton, seçili olanın kenarı mavi (accent).
- **Metin Biçimlendirme:**
  - Yazı Tipi: Consolas (Monospace) — şu an sabit, sadece görüntülenir.
  - Boyut: kaydırıcı (slider) 8–24 + px göstergesi.
  - Canlı önizleme kutusu: "AaBbCc 123!@# — The quick brown fox".
  - Onay kutusu: **Satır Kaydırma**.
  - Onay kutusu: **Biçimlendirme (Zengin Metin)**.
- **Gelişmiş:**
  - Onay kutusu: **Başlangıçta sekmeleri geri yükle**.

Ayarlar bölümleri kart şeklinde: yuvarlatılmış köşeli, gri arka planlı (surface) kutular.

### 3.4 Alt Panel — Durum Çubuğu
- Sol: `uNote v0.1.0  |  N not` + seçili sekme adı (separator ile ayrılır).
- Sağ: tema göstergesi (`☀ Light` / `🌙 Dark` / `🖥 System`).

### 3.5 Popup / Menüler (katmanlı)

**A) ▾ Menü (aşağı açılır):**
- ⚙ Ayarlar → Ayarlar sekmesini açar.
- ℹ Hakkında → Hakkında popup'ı.
- Menü, pencere dışına tıklanınca veya Esc ile kapanır.

**B) Hakkında Popup (merkezde):**
- "uNote", sürüm, "Güvenli not alma kasası", "Rust + egui".
- "Tamam" butonu.

**C) Not Bağlam Menüsü (sağ tık):**
- En üstte not başlığı (kısaltılmış) gösterilir.
- 🗑 **Notu Sil** — kırmızı renkli. Tıklanınca notu siler.

## 4. Tema ve Renkler (Win11 Fluent)

Tüm renkler `theme.rs`'te merkezlenir; açık/koyu için iki palet:

| Token | Açık (Light) | Koyu (Dark) |
|---|---|---|
| bg (ana zemin) | #F3F3F3 | #202020 |
| surface (kart/panel) | #FFFFFF | #2D2D2D |
| surface_hover | #F9F9F9 | #383838 |
| surface_pressed | #F0F0F0 | #444444 |
| accent (vurgu) | #0078D4 | #60CDFF |
| text | #1A1A1A | #FFFFFF |
| text_secondary | #616161 | #9E9E9E |
| border | #E0E0E0 | #404040 |
| separator | #E6E6E6 | #373737 |
| selection | accent + %40 saydam | accent + %60 saydam |
| danger (tehlike) | #C83232 | #FF6464 |

- Varsayılan tema **Açık (Light)**.
- Pencerede akrilik cam efekti + yuvarlatılmış köşeler (DWM).
- Kullanıcı avatar renkleri 8'li pastel paletten döngüsel alınır (açık/koyu ayrı paletler).

## 5. Etkileşim & Davranış (Önemli)

- **Pencere sürükleme:** Üst boş alanı sürükleyerek; çift tıklayınca maximize/minimize.
- **Otomatik kayıt:** Editörde her değişiklikte DB'ye yazar.
- **Arama:** Canlı filtreleme (başlık + içerik).
- **Sekme yönetimi:** aç/kapat/yeniden adlandır/sürükle-sırala/geri yükle.
- **Kilitli notlar:** 🔒 simgesiyle gösterilir; içerik/başlık şifrelidir.
- **Ayarlar kalıcılığı:** Tema, font boyutu, satır kaydırma, biçimlendirme, geri yükle, açık sekme ID'leri — JSON dosyasına kaydedilir, açılışta geri yüklenir.
- **Tüm paneller** tek `update()` içinde, "action-flags" deseniyle yönetilir (borrow çakışmasından kaçınmak için).

## 6. Dosya Yapısı (kod)

| Dosya | İçerik |
|---|---|
| `src/main.rs` | Pencere kurulumu, DWM köşeler, akrilik |
| `src/app.rs` | Tüm UI panelleri, etkileşim, durum (en büyük dosya) |
| `src/db.rs` | SQLite CRUD |
| `src/crypto.rs` | AES-256-GCM şifreleme |
| `src/models.rs` | Note, NoteTab, AppTheme, avatar renkleri |
| `src/settings.rs` | Kalıcı ayarlar (JSON) |
| `src/theme.rs` | Renk paleti + Win11 stil üretici |

## 7. Tasarım İpuçları (yeniden tasarlanırken)

- Mevcut yapı **Windows 11 Fluent / Mica** hissine sahip; tasarım bunun üzerine inşa edilmeli.
- Ekranın hepsi tek seferde görünür (tabs not defteri) — **çok sayfalı** değil, **çok panelli** bir yapı.
- vurgu rengi (accent) seçimler, butonlar ve seçili öğelerde.
- Not listesi avatar renkleri kişisel/renkli bir dokunuş katar.
- Dil: Türkçe (UI metinleri Türkçe).