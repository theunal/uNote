import { state, setState, saveSettings } from "../store";
import { svgSun, svgMoon, svgMonitor } from "../svg";

export function SettingsView() {
  const themeBtns = [
    [svgSun(), "Açık", "light"],
    [svgMoon(), "Koyu", "dark"],
    [svgMonitor(), "Sistem", "system"],
  ] as const;

  return (
    <div class="settings">
      <h2>Ayarlar</h2>
      <div class="card">
        <h3>Tema</h3>
        <div class="seg">
          {themeBtns.map(([ico, label, val]) => (
            <button class={state.theme === val ? "active" : ""} data-theme={val} onClick={() => {
              setState("theme", val);
              saveSettings();
            }}>{ico} {label}</button>
          ))}
        </div>
      </div>
      <div class="card">
        <h3>Metin Biçimlendirme</h3>
        <div class="row"><div><div class="lbl">Yazı Tipi</div><div class="sub">Cascadia Code (Monospace)</div></div></div>
        <div class="row">
          <div class="lbl">Boyut</div>
          <input type="range" class="slider" id="fsSlider" min="8" max="24" value={state.font_size} onInput={(e) => {
            setState("font_size", +(e.target as HTMLInputElement).value);
            saveSettings();
          }} />
          <div class="size-val">{state.font_size}px</div>
        </div>
        <div class="preview" style={{ "font-size": state.font_size + "px" }}>AaBbCc 123!@# — The quick brown fox</div>
        <label class="check">
          <input type="checkbox" id="chkWrap" checked={state.word_wrap} onChange={(e) => { setState("word_wrap", (e.target as HTMLInputElement).checked); saveSettings(); }} />
          <span class="lbl">Satır Kaydırma</span>
        </label>
        <label class="check">
          <input type="checkbox" id="chkFmt" checked={state.formatting_enabled} onChange={(e) => { setState("formatting_enabled", (e.target as HTMLInputElement).checked); saveSettings(); }} />
          <span class="lbl">Biçimlendirme (Zengin Metin)</span>
        </label>
      </div>
      <div class="card">
        <h3>Gelişmiş</h3>
        <label class="check">
          <input type="checkbox" id="chkRestore" checked={state.restore_tabs} onChange={(e) => { setState("restore_tabs", (e.target as HTMLInputElement).checked); saveSettings(); }} />
          <span class="lbl">Başlangıçta sekmeleri geri yükle</span>
        </label>
      </div>
    </div>
  );
}

export default SettingsView;