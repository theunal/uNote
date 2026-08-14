import { For, } from "solid-js";
import {
  state, setState, refreshNotes, openTab,
  toggleAppMenu,
} from "../store";
import { invoke, appWindow } from "../tauri";
import { PASSWORD } from "../constants";
import { TabItem } from "./TabItem";
import { SearchBox } from "./SearchBox";

export function TitleBar() {
  const onAdd = async () => {
    if (state.searchQuery) setState("searchQuery", "");
    const id = (await invoke("create_note", { password: PASSWORD })) as number;
    await refreshNotes();
    const note = state.notes.find((x) => x.id === id);
    if (note) openTab(note);
  };

  const isInteractive = (t: Element) => t.closest(".tab") || t.closest(".icon-btn") ||
    t.closest(".wc-btn") || t.closest(".searchbox") || t.closest("input");

  const onDragDown = (e: MouseEvent) => {
    if (e.button !== 0 || isInteractive(e.target as Element)) return;
    appWindow.startDragging();
  };
  const onDragDblClick = async (e: MouseEvent) => {
    if (isInteractive(e.target as Element)) return;
    const m = await appWindow.isMaximized();
    if (m) await appWindow.unmaximize(); else await appWindow.maximize();
  };

  return (
    <div class="titlebar" onMouseDown={onDragDown} onDblClick={onDragDblClick}
    >
      <div class="drag">
        <div class="tabs" id="tabs">
          <For each={state.tabs}>{(tab, i) => <TabItem tab={tab} index={i()} />}</For>
        </div>
        <div class="icon-btn" id="btnAdd" title="Yeni not" onClick={onAdd}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" class="svg_icon">
            <path d="M352 128C352 110.3 337.7 96 320 96C302.3 96 288 110.3 288 128L288 288L128 288C110.3 288 96 302.3 96 320C96 337.7 110.3 352 128 352L288 352L288 512C288 529.7 302.3 544 320 544C337.7 544 352 529.7 352 512L352 352L512 352C529.7 352 544 337.7 544 320C544 302.3 529.7 288 512 288L352 288L352 128z" /></svg>
        </div>
        <div class="icon-btn" id="btnMenu" title="Menü" onClick={(e) => {
          e.stopPropagation();
          toggleAppMenu();
        }}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" class="svg_icon">
            <path d="M297.4 470.6C309.9 483.1 330.2 483.1 342.7 470.6L534.7 278.6C547.2 266.1 547.2 245.8 534.7 233.3C522.2 220.8 501.9 220.8 489.4 233.3L320 402.7L150.6 233.4C138.1 220.9 117.8 220.9 105.3 233.4C92.8 245.9 92.8 266.2 105.3 278.7L297.3 470.7z" /></svg>
        </div>
      </div>
      <div class="tbar-right">
        <SearchBox />
      </div>
      <div class="winctrl">
        <div class="wc-btn" id="btnMin" title="Küçült" onClick={() => appWindow.minimize()}>
          <svg viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.1">
            <line x1="0" y1="5" x2="10" y2="5" /></svg>
        </div>
        <div class="wc-btn" id="btnMax" title="Büyüt" onClick={async () => {
          const m = await appWindow.isMaximized();
          if (m) await appWindow.unmaximize(); else await appWindow.maximize();
        }}>
          <svg viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.1">
            <rect x="0.5" y="0.5" width="9" height="9" rx="1" /></svg>
        </div>
        <div class="wc-btn close" id="btnClose" title="Kapat" onClick={() => appWindow.close()}>
          <svg viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.1">
            <line x1="0" y1="0" x2="10" y2="10" /><line x1="10" y1="0" x2="0" y2="10" /></svg>
        </div>
      </div>
    </div>
  );
}