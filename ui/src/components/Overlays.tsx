import { Show } from "solid-js";
import { state, setState, deleteNoteFromList, openSettings } from "../store";
import { trunc } from "../util";
import { svgGear, svgInfo, svgTrash, svgSun, svgMoon, svgMonitor } from "../svg";
import { VERSION } from "../constants";

function ThemeIndicator() {
  const label = () => state.theme === "dark" ? "Koyu" : state.theme === "light" ? "Açık" : "Sistem";
  const icon = () => state.theme === "dark" ? svgMoon() : state.theme === "light" ? svgSun() : svgMonitor();
  return (
    <span class="theme-ind" id="sbTheme"><span innerHTML={icon()} /> {label()}</span>
  );
}

export function StatusBar() {
  const activeTitle = () => {
    if (state.activeTab === -1) return "Ayarlar";
    if (state.activeTab !== null && state.tabs[state.activeTab]) return state.tabs[state.activeTab].title;
    return "Notlar";
  };

  return (
    <div class="statusbar">
      <span id="sbVersion">uNote v{VERSION}</span>
      <span class="sep">|</span>
      <span id="sbCount">{state.notes.length} not</span>
      <span class="sep">|</span>
      <span id="sbActive">{activeTitle()}</span>
      <ThemeIndicator />
    </div>
  );
}

export function Overlays() {
  const ctxNote = () => state.ctx ? state.notes.find(n => n.id === state.ctx!.note_id) : null;

  return (
    <>
      <Show when={state.showAppMenu}>
        <div class="menu" id="appMenu" style={{ left: "60px", top: "44px" }}>
          <div class="menu-item" onClick={() => openSettings()}><span class="mi-ico" innerHTML={svgGear()} /> Ayarlar</div>
          <div class="menu-item" onClick={() => setState("showAbout", true)}><span class="mi-ico" innerHTML={svgInfo()} /> Hakkında</div>
        </div>
      </Show>

      <Show when={state.ctx}>
        <div class="menu" id="ctxMenu" style={{ left: state.ctx!.x + "px", top: state.ctx!.y + "px" }}>
          <div class="menu-head">{trunc(ctxNote()?.title || "", 20)}</div>
          <div class="menu-sep"></div>
          <div class="menu-item danger" onClick={() => deleteNoteFromList(state.ctx!.note_id)}><span class="mi-ico" innerHTML={svgTrash()} /> Notu Sil</div>
        </div>
      </Show>

      <Show when={state.showAbout}>
        <div class="overlay">
          <div class="dialog" role="dialog" aria-modal="true" aria-label="Hakkında">
            <div class="d-logo">uNote</div>
            <h3>Güvenli Not Alma Kasası</h3>
            <p>Notlarınız AES-256-GCM ile şifrelenir ve yerel olarak saklanır.</p>
            <div class="version">Sürüm 0.1.0 · Rust + Tauri</div>
            <button class="btn primary dialog-ok" onClick={() => setState("showAbout", false)}>Tamam</button>
          </div>
        </div>
      </Show>
    </>
  );
}