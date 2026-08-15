import { For, Show } from "solid-js";
import { openContextMenu, openTab, refreshNotes, setState, state } from "../store";
import { AVATAR_COLORS, PASSWORD } from "../constants";
import { svgLock, svgSearch } from "../svg";
import { invoke } from "../tauri";
import { Input } from "./Input";

export function NoteSearchPanel() {
  const onAdd = async () => {
    if (state.searchQuery) setState("searchQuery", "");
    const id = (await invoke("create_note", { password: PASSWORD })) as number;
    await refreshNotes();
    const note = state.notes.find((x) => x.id === id);
    if (note) openTab(note);
  };

  return (
    <div class={"search-panel" + (state.searchOpen ? " open" : "")} id="searchPanel">
      <div class="search-panel-head">
        <Input
          variant="default"
          icon={svgSearch()}
          placeholder="Notlarda ara..."
          aria-label="Notlarda ara"
          autocomplete="off"
          value={state.searchQuery}
          onInput={async (e) => {
            setState("searchQuery", (e.target as HTMLInputElement).value);
            await refreshNotes();
          }}
        />
        <button class="search-panel-add" type="button" title="Yeni not" onClick={onAdd}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" class="svg_icon">
            <path d="M352 128C352 110.3 337.7 96 320 96C302.3 96 288 110.3 288 128L288 288L128 288C110.3 288 96 302.3 96 320C96 337.7 110.3 352 128 352L288 352L288 512C288 529.7 302.3 544 320 544C337.7 544 352 529.7 352 512L352 352L512 352C529.7 352 544 337.7 544 320C544 302.3 529.7 288 512 288L352 288L352 128z" /></svg>
        </button>
      </div>
      <div class="search-panel-list">
        <Show when={state.notes.length === 0} fallback={
          <For each={state.notes}>{(n, i) => (
            <div
              class="search-note"
              role="button"
              tabIndex={0}
              onClick={() => { openTab(n); setState("searchOpen", false); }}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") { e.preventDefault(); openTab(n); setState("searchOpen", false); }
              }}
              onContextMenu={(e) => {
                e.preventDefault();
                openContextMenu(e.clientX, e.clientY, n.id);
              }}
            >
              <div class="search-note-avatar" style={{ background: AVATAR_COLORS[i() % AVATAR_COLORS.length] }} aria-hidden="true">
                {(n.title || "N").slice(0, 3).toUpperCase()}
              </div>
              <div class="search-note-body">
                <div class="search-note-title">
                  {n.title || "Başlıksız"}
                  {n.is_locked ? <span class="search-note-lock" aria-hidden="true" innerHTML={svgLock()} /> : null}
                </div>
                <div class="search-note-preview">{n.content ? n.content.slice(0, 50) : "Boş not"}</div>
              </div>
            </div>
          )}</For>
        }>
          <div class="search-panel-empty">Not bulunamadı</div>
        </Show>
      </div>
    </div>
  );
}
