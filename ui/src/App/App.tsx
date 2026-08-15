import { createEffect, onMount, Show } from "solid-js";
import { state, setState, refreshNotes, openTab, hideMenus, closeSettings, openSettings } from "../store";
import { loadSettings } from "../store";
import { svgGear } from "../svg";
import "./App.scss";
import { TitleBar } from "../components/TitleBar/TitleBar";
import { MenuBar } from "../components/MenuBar/MenuBar";
import { Toolbar } from "../components/Toolbar/Toolbar";
import { Main } from "../components/Main/Main";
import { StatusBar } from "../components/StatusBar/StatusBar";
import { Overlays } from "../components/Overlays/Overlays";
import { Settings } from "../components/Settings/Settings";

function applyTheme() {
  const dark = state.theme === "dark"
    || (state.theme === "system" && window.matchMedia("(prefers-color-scheme: dark)").matches);
  document.documentElement.setAttribute("data-theme", dark ? "dark" : "light");
}

export function App() {
  createEffect(() => applyTheme());

  onMount(async () => {
    const s = await loadSettings();
    await refreshNotes();

    if (state.restore_tabs && s.open_tab_ids && s.open_tab_ids.length) {
      for (const id of s.open_tab_ids) {
        const n = state.notes.find((x) => x.id === id);
        if (n) openTab(n, false);
      }
    }
    if (state.tabs.length === 0 && state.notes.length) {
      openTab(state.notes[0], false);
    }
    if (state.tabs.length) {
      setState("activeTab", 0);
    }
  });

  // Global events via Solid JSX handlers on the root element
  const onWinClick = (e: MouseEvent) => {
    const t = e.target as Element;
    if (state.showAppMenu && !t.closest("#appMenu") && !t.closest("#btnMenu")) hideMenus();
    if (state.searchOpen && !t.closest("#searchPanel") && !t.closest(".searchbox-trigger")) hideMenus();
    if (state.ctx && !t.closest("#ctxMenu")) setState("ctx", null);
  };
  const onWinContextMenu = (e: MouseEvent) => {
    if (!(e.target as Element).closest(".search-note")) hideMenus();
  };
  const onWinKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Escape") { if (state.settingsOpen) closeSettings(); else hideMenus(); }
  };
  const onWinMouseUp = () => setState("dragIndex", null);

  return (
    <div class="win"
      onClick={onWinClick}
      onContextMenu={onWinContextMenu}
      onKeyDown={onWinKeyDown}
      onMouseUp={onWinMouseUp}
    >
      <Show when={state.settingsOpen} fallback={
        <>
          <TitleBar />
          <div class="editbar">
            <MenuBar />
            <Toolbar />
            <button class="editbar-settings" type="button"  aria-label="Ayarlar" onClick={() => openSettings()}>
              <span innerHTML={svgGear()} />
            </button>
          </div>
          <Main />
          <StatusBar />
          <Overlays />
        </>
      }>
        <Settings />
      </Show>
      <Show when={state.toast}>
        <div class="toast" role="alert">{state.toast}</div>
      </Show>
    </div>
  );
}