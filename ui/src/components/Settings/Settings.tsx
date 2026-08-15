import { createSignal, onMount } from "solid-js";
import { state, setState, saveSettings, closeSettings } from "../../store";
import { invoke, appWindow } from "../../tauri";
import {
  svgBack, svgChevronUp, svgChevronDown, svgPalette, svgType,
  svgWrapText, svgWandSparkles, svgFolderOpen, svgFileClock, svgHistory,
  svgSpellCheck, svgSparkles,
} from "../../svg";
import { SelectBox } from "../SelectBox/SelectBox";
import { WindowControls } from "../WindowControls/WindowControls";
import "./Settings.scss";

const DEFAULT_FONTS = ["Space Mono", "DM Sans", "Georgia"];
const STYLES = ["Regular", "Italic", "Bold", "Bold Italic"];
const SIZES = [10, 11, 12, 14, 16];
const FILE_MODES = [
  { value: "new", label: "Yeni sekmede aç" },
  { value: "current", label: "Mevcut sekmede aç" },
];

const editorEl = () => document.getElementById("note-editor") as HTMLTextAreaElement | null;

function applyFontToEditor() {
  const el = editorEl();
  if (!el) return;
  const bold = state.font_style.includes("Bold");
  const italic = state.font_style.includes("Italic");
  el.style.fontFamily = `"${state.font_family}", sans-serif`;
  el.style.fontWeight = bold ? "700" : "400";
  el.style.fontStyle = italic ? "italic" : "normal";
}

function ToggleSwitch(props: { checked: boolean; label: string; on: string; onChange: (v: boolean) => void }) {
  return (
    <div class="settings-row">
      <label class="switch" aria-label={`${props.label} aç veya kapat`}>
        <input type="checkbox" checked={props.checked} onChange={(e) => props.onChange((e.target as HTMLInputElement).checked)} />
        <span class="switch-track"></span>
      </label>
      <span class="switch-text">{props.checked ? props.on : "Kapalı"}</span>
    </div>
  );
}

function CollapsibleCard(props: {
  icon: string;
  label: string;
  description?: string;
  open: boolean;
  onToggle: () => void;
  controls?: any;
  children?: any;
  class?: string;
}) {
  return (
    <div class="settings-card" classList={{ [props.class ?? ""]: !!props.class }}>
      <div class="settings-row clickable" onClick={props.onToggle}>
        <span class="row-icon" innerHTML={props.icon} />
        <div class="row-copy">
          <h3 class="row-title">{props.label}</h3>
          {props.description && <p class="row-description">{props.description}</p>}
        </div>
        {props.controls}
        <span class="expand-icon" innerHTML={props.open ? svgChevronUp() : svgChevronDown()} />
      </div>
      {props.open && props.children}
    </div>
  );
}

export function Settings() {
  const [openCards, setOpenCards] = createSignal<Record<string, boolean>>({
    theme: true, font: true, startup: true,
  });
  const [fonts, setFonts] = createSignal<string[]>(DEFAULT_FONTS);
  const [statusVisible, setStatusVisible] = createSignal(false);
  let statusTimer: number | undefined;

  onMount(async () => {
    try {
      const all = (await invoke("list_fonts")) as string[];
      if (all && all.length) {
        const merged = [...DEFAULT_FONTS];
        for (const f of all) if (!merged.includes(f)) merged.push(f);
        setFonts(merged);
      }
    } catch { /* tarayıcı modunda varsayılanlar yeterli */ }
  });

  const toggleCard = (key: string) => setOpenCards({ ...openCards(), [key]: !openCards()[key] });

  const showSaved = () => {
    setStatusVisible(true);
    clearTimeout(statusTimer);
    statusTimer = window.setTimeout(() => setStatusVisible(false), 1800);
  };

  const set = (key: any, value: any, applyEditor = false) => {
    setState(key, value);
    saveSettings();
    if (applyEditor) applyFontToEditor();
    showSaved();
  };

  const isInteractive = (t: Element) => t.closest("button") || t.closest(".wc-btn") || t.closest(".selectbox") || t.closest("input") || t.closest(".style-menu-wrap");
  const onDragDown = (e: MouseEvent) => {
    if (e.button !== 0 || isInteractive(e.target as Element)) return;
    if (e.detail === 2) {
      e.preventDefault();
      void (async () => {
        const m = await appWindow.isMaximized();
        if (m) await appWindow.unmaximize(); else await appWindow.maximize();
      })();
      return;
    }
    appWindow.startDragging();
  };

  const themeOpts: Array<[string, string]> = [["light", "Açık"], ["dark", "Koyu"], ["system", "Sistem ayarını kullan"]];

  return (
    <div class="settings-page">
      <header class="settings-header" onMouseDown={onDragDown}>
        <button class="back-btn" type="button" aria-label="Geri" onClick={() => closeSettings()}>
          <span innerHTML={svgBack()} />
          <span>Geri</span>
        </button>
        <span class="settings-title">Ayarlar</span>
        <WindowControls />
      </header>

      <div class="settings-scroll">
        <main class="settings-app" aria-labelledby="settings-title">
          <div class="settings-content">
            <header>
              <h1 id="settings-title" class="page-title">Ayarlar</h1>
            </header>

            <section class="settings-section" aria-labelledby="appearance-label">
              <h2 id="appearance-label" class="section-label">Görünüm</h2>
              <CollapsibleCard
                icon={svgPalette()} label="Uygulama teması" description="Görüntülenecek uygulama temasını seçin"
                open={openCards().theme} onToggle={() => toggleCard("theme")}
              >
                <div class="theme-options" role="radiogroup" aria-label="Uygulama teması">
                  {themeOpts.map(([val, label]) => (
                    <label class="choice">
                      <input type="radio" name="theme" value={val} checked={state.theme === val}
                        onChange={() => set("theme", val)} />
                      <span>{label}</span>
                    </label>
                  ))}
                </div>
              </CollapsibleCard>
            </section>

            <section class="settings-section" aria-labelledby="format-label">
              <h2 id="format-label" class="section-label">Metin Biçimlendirme</h2>
              <CollapsibleCard class="mb-1"
                icon={svgType()} label="Yazı Tipi"
                open={openCards().font} onToggle={() => toggleCard("font")}
              >
                <div class="font-panel">
                  <div class="field-row">
                    <label class="field-label" for="font-family">Aile</label>
                    <SelectBox
                      id="font-family"
                      value={state.font_family}
                      options={fonts()}
                      searchable
                      ariaLabel="Aile"
                      onChange={(v) => set("font_family", v, true)}
                    />
                  </div>
                  <div class="field-row">
                    <span class="field-label">Stil</span>
                    <SelectBox
                      value={state.font_style}
                      options={STYLES}
                      searchable={false}
                      ariaLabel="Stil"
                      onChange={(v) => set("font_style", v, true)}
                    />
                  </div>
                  <div class="field-row">
                    <label class="field-label" for="font-size">Boyut</label>
                    <SelectBox
                      id="font-size"
                      value={String(state.font_size)}
                      options={SIZES.map(String)}
                      searchable
                      ariaLabel="Boyut"
                      onChange={(v) => set("font_size", +v, true)}
                    />
                  </div>
                  <p id="font-preview" class="font-preview"
                    style={{
                      "font-family": `"${state.font_family}", sans-serif`,
                      "font-size": state.font_size + "px",
                      "font-weight": state.font_style.includes("Bold") ? "700" : "400",
                      "font-style": state.font_style.includes("Italic") ? "italic" : "normal",
                    }}>
                    Okyanus dalgalarının sesi ruhuma dinginlik veriyor.
                  </p>
                </div>
              </CollapsibleCard>

              <div class="settings-card mb-1">
                <div class="settings-row">
                  <span class="row-icon" innerHTML={svgWrapText()} />
                  <div class="row-copy">
                    <h3 class="row-title">Sözcük kaydırma</h3>
                    <p class="row-description">Sözcükleri pencere genişliğine sığdırın</p>
                  </div>
                  <ToggleSwitch checked={state.word_wrap} label="Sözcük kaydırma" on="Açık"
                    onChange={(v) => set("word_wrap", v)} />
                </div>
              </div>

              <div class="settings-card">
                <div class="settings-row">
                  <span class="row-icon" innerHTML={svgWandSparkles()} />
                  <div class="row-copy">
                    <h3 class="row-title">Biçimlendirme</h3>
                  </div>
                  <ToggleSwitch checked={state.formatting_enabled} label="Biçimlendirme" on="Açık" onChange={(v) => set("formatting_enabled", v)} />
                </div>
              </div>
            </section>

            <section class="settings-section" aria-labelledby="startup-label">
              <h2 id="startup-label" class="section-label">Not Defteri Açılıyor</h2>
              <div class="settings-card mb-1">
                <div class="settings-row">
                  <span class="row-icon" innerHTML={svgFolderOpen()} />
                  <div class="row-copy">
                    <h3 class="row-title">Dosyaları açma</h3>
                    <p class="row-description">Dosyalarınızın nerede açılacağını seçin</p>
                  </div>
                  <div>
                    <label for="open-files" class="sr-only">Dosya açma tercihi</label>
                    <SelectBox
                      id="open-files"
                      value={state.open_files_mode}
                      options={FILE_MODES}
                      searchable={false}
                      minWidth="200px"
                      ariaLabel="Dosya açma tercihi"
                      onChange={(v) => set("open_files_mode", v)}
                    />
                  </div>
                </div>
              </div>

              <CollapsibleCard class="mb-1"
                icon={svgFileClock()} label="Not Defteri başlatıldığında"
                open={openCards().startup} onToggle={() => toggleCard("startup")}
              >
                <div class="startup-options" role="radiogroup" aria-label="Başlangıç tercihi">
                  <label class="choice">
                    <input type="radio" name="startup" checked={state.restore_tabs}
                      onChange={() => set("restore_tabs", true)} />
                    <span>Önceki oturumdan devam et</span>
                  </label>
                  <label class="choice">
                    <input type="radio" name="startup" checked={!state.restore_tabs}
                      onChange={() => set("restore_tabs", false)} />
                    <span>Yeni oturum başlat ve kaydedilmemiş değişiklikleri at</span>
                  </label>
                </div>
              </CollapsibleCard>

              <div class="settings-card">
                <div class="settings-row">
                  <span class="row-icon" innerHTML={svgHistory()} />
                  <div class="row-copy">
                    <h3 class="row-title">Son Kullanılan Dosyalar</h3>
                  </div>
                  <ToggleSwitch checked={state.recent_files} label="Son kullanılan dosyalar" on="Açık" onChange={(v) => set("recent_files", v)} />
                </div>
              </div>
            </section>

            <section class="settings-section" aria-labelledby="spell-label">
              <h2 id="spell-label" class="section-label">Yazım Denetimi</h2>
              <div class="settings-card mb-1">
                <div class="settings-row">
                  <span class="row-icon" innerHTML={svgSpellCheck()} />
                  <div class="row-copy">
                    <h3 class="row-title">Yazım denetimi</h3>
                    <p class="row-description">Yazarken yazım denetimini kullanın</p>
                  </div>
                  <ToggleSwitch checked={state.spell_check} label="Yazım denetimi" on="Açık" onChange={(v) => set("spell_check", v)} />
                </div>
              </div>
              <div class="settings-card">
                <div class="settings-row">
                  <span class="row-icon" innerHTML={svgWandSparkles()} />
                  <div class="row-copy">
                    <h3 class="row-title">Otomatik düzeltme</h3>
                    <p class="row-description">Yazım denetimi etkinleştirildiğinde yazım hatalarını otomatik olarak düzeltir</p>
                  </div>
                  <ToggleSwitch checked={state.autocorrect} label="Otomatik düzeltme" on="Açık" onChange={(v) => set("autocorrect", v)} />
                </div>
              </div>
            </section>

            <section class="settings-section" aria-labelledby="advanced-label">
              <h2 id="advanced-label" class="section-label">Gelişmiş Özellikler</h2>
              <div class="settings-card">
                <div class="settings-row">
                  <span class="row-icon" innerHTML={svgSparkles()} />
                  <div class="row-copy">
                    <h3 class="row-title">Yazma araçları</h3>
                  </div>
                  <ToggleSwitch checked={state.writing_tools} label="Yazma araçları" on="Açık" onChange={(v) => set("writing_tools", v)} />
                </div>
              </div>
            </section>

            <p id="save-status" class={"status-message" + (statusVisible() ? " visible" : "")} aria-live="polite">Ayarlar kaydedildi.</p>
          </div>
        </main>
      </div>
    </div>
  );
}

export default Settings;