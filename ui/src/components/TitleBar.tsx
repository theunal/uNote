import { For, Show } from "solid-js";
import { state, setState, refreshNotes, openTab, closeTab, commitRename, startRename, toggleAppMenu, saveSettings } from "../store";
import { invoke, appWindow } from "../tauri";
import { PASSWORD } from "../constants";
import { trunc } from "../util";
import { svgLock } from "../svg";

function TabItem(props: { tab: { note_id: number; title: string; content: string; is_locked: boolean }; index: number }) {
  const tab = props.tab;
  const i = props.index;

  const isRenaming = () => state.rename?.index === i;

  const reorder = () => {
    if (state.dragIndex === null || state.dragIndex === i) return;
    const from = state.dragIndex;
    const active = state.activeTab;
    const next = [...state.tabs];
    const [moved] = next.splice(from, 1);
    next.splice(i, 0, moved);
    setState("tabs", next);
    if (active === from) setState("activeTab", i);
    else if (active !== null && from < active && i >= active) setState("activeTab", active - 1);
    else if (active !== null && from > active && i <= active) setState("activeTab", active + 1);
    setState("dragIndex", i);
    saveSettings();
  };

  return (
    <div
      class={"tab" + (i === state.activeTab ? " active" : "")}
      role="tab"
      tabIndex={0}
      aria-selected={(i === state.activeTab).toString() as "true" | "false"}
      onClick={() => {
        if (state.rename) return;
        if (i === state.activeTab && tab.note_id !== -1) { startRename(i, tab.title); return; }
        setState("activeTab", i);
      }}
      onDblClick={(e) => {
        if ((e.target as HTMLElement).classList.contains("tab-close")) return;
        if (tab.note_id !== -1) startRename(i, tab.title);
      }}
      onMouseDown={(e) => {
        if ((e.target as HTMLElement).classList.contains("tab-close")) return;
        setState("dragIndex", i);
      }}
      onMouseEnter={reorder}
    >
      <Show when={isRenaming()} fallback={
        <>
          {tab.is_locked ? <span class="lock" innerHTML={svgLock()} /> : null}
          <span class="tab-title">{trunc(tab.title, 15)}</span>
          <span class="tab-close" role="button" aria-label="Sekmeyi kapat" onClick={(e) => { e.stopPropagation(); closeTab(i); }}>×</span>
        </>
      }>
        <input
          ref={(el) => { el.focus(); el.select(); }}
          value={state.rename?.buf || ""}
          placeholder="Başlık"
          onClick={(e) => e.stopPropagation()}
          onKeyDown={(e) => {
            if (e.key === "Enter") commitRename();
            if (e.key === "Escape") setState("rename", null);
          }}
          onInput={(e) => { if (state.rename) setState("rename", "buf", (e.target as HTMLInputElement).value); }}
        />
      </Show>
    </div>
  );
}

function SearchBox() {
  return (
    <div class="searchbox">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
        <circle cx="11" cy="11" r="7" />
        <line x1="21" y1="21" x2="16.5" y2="16.5" />
      </svg>
      <input
        id="search"
        placeholder="Notlarda ara..."
        aria-label="Notlarda ara"
        autocomplete="off"
        value={state.searchQuery}
        onInput={async (e) => {
          setState("searchQuery", (e.target as HTMLInputElement).value);
          await refreshNotes();
        }}
      />
    </div>
  );
}

export function TitleBar() {
  const onAdd = async () => {
    if (state.searchQuery) setState("searchQuery", "");
    const id = (await invoke("create_note", { password: PASSWORD })) as number;
    await refreshNotes();
    const n = state.notes.find((x) => x.id === id);
    if (n) openTab(n);
  };

  const isInteractive = (t: Element) => t.closest(".tab") || t.closest(".icon-btn") || t.closest(".wc-btn") || t.closest(".searchbox") || t.closest("input");

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
    <div class="titlebar" onMouseDown={onDragDown} onDblClick={onDragDblClick}>
      <div class="drag">
        <div class="tabs" id="tabs">
          <For each={state.tabs}>{(tab, i) => <TabItem tab={tab} index={i()} />}</For>
        </div>
        <div class="icon-btn" id="btnAdd" title="Yeni not" onClick={onAdd}>+</div>
        <div class="icon-btn" id="btnMenu" title="Menü" onClick={(e) => { e.stopPropagation(); toggleAppMenu(); }}>▾</div>
      </div>
      <div class="tbar-right">
        <SearchBox />
      </div>
      <div class="winctrl">
        <div class="wc-btn" id="btnMin" title="Küçült" onClick={() => appWindow.minimize()}>
          <svg viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.1"><line x1="0" y1="5" x2="10" y2="5" /></svg>
        </div>
        <div class="wc-btn" id="btnMax" title="Büyüt" onClick={async () => {
          const m = await appWindow.isMaximized();
          if (m) await appWindow.unmaximize(); else await appWindow.maximize();
        }}>
          <svg viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.1"><rect x="0.5" y="0.5" width="9" height="9" rx="1" /></svg>
        </div>
        <div class="wc-btn close" id="btnClose" title="Kapat" onClick={() => appWindow.close()}>
          <svg viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1.1"><line x1="0" y1="0" x2="10" y2="10" /><line x1="10" y1="0" x2="0" y2="10" /></svg>
        </div>
      </div>
    </div>
  );
}