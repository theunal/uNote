import { createEffect, onMount } from "solid-js";
import { state, setState, refreshNotes, openTab, hideMenus } from "./store";
import { loadSettings } from "./store";
import { TitleBar } from "./components/TitleBar";
import { MenuBar } from "./components/MenuBar";
import { Toolbar } from "./components/Toolbar";
import { Main } from "./components/Main";
import { StatusBar, Overlays } from "./components/Overlays";

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
    if (e.key === "Escape") { hideMenus(); }
  };
  const onWinMouseUp = () => setState("dragIndex", null);

  return (
    <div class="win"
      onClick={onWinClick}
      onContextMenu={onWinContextMenu}
      onKeyDown={onWinKeyDown}
      onMouseUp={onWinMouseUp}
    >
      <TitleBar />
      <MenuBar />
      <Toolbar />
      <Main />
      <StatusBar />
      <Overlays />
    </div>
  );
}