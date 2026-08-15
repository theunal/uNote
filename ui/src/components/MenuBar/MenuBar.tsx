import { Show, createSignal, onCleanup, onMount } from "solid-js";
import { PASSWORD } from "../../constants";
import { refreshNotes, saveSettings, setState, state, openSearch, openTab } from "../../store";
import { invoke } from "../../tauri";
import "./MenuBar.scss";

export function MenuBar() {
  const [openMenu, setOpenMenu] = createSignal<string | null>(null);

  const toggleMenu = (name: string) => setOpenMenu(openMenu() === name ? null : name);
  const closeMenus = () => setOpenMenu(null);

  onMount(() => {
    document.addEventListener("click", closeMenus);
    onCleanup(() => document.removeEventListener("click", closeMenus));
  });

  const onNew = async () => {
    if (state.searchQuery) setState("searchQuery", "");
    const id = (await invoke("create_note", { password: PASSWORD })) as number;
    await refreshNotes();
    const note = state.notes.find((x) => x.id === id);
    if (note) openTab(note);
    closeMenus();
  };

  const onClear = () => {
    const idx = state.activeTab;
    if (idx === null || idx === -1) return;
    const t = state.tabs[idx];
    if (!t) return;
    setState("tabs", idx, "content", "");
    invoke("save_note_content", {
      args: { id: t.note_id, content: "", is_locked: t.is_locked, password: PASSWORD },
    });
    closeMenus();
  };

  const onSelectAll = () => {
    const el = document.getElementById("note-editor") as HTMLTextAreaElement | null;
    if (el) { el.focus(); el.select(); }
    closeMenus();
  };

  const onFind = () => {
    openSearch();
    closeMenus();
  };

  const onToggleWrap = () => {
    setState("word_wrap", !state.word_wrap);
    saveSettings();
    closeMenus();
  };

  const stop = (e: MouseEvent) => e.stopPropagation();

  return (
    <nav class="menu-row" aria-label="Not Defteri menüsü">
      <button
        class={"menu-button" + (openMenu() === "file-options" ? " active" : "")}
        type="button"
        data-menu="file-options"
        onClick={(e) => { e.stopPropagation(); toggleMenu("file-options"); }}
      >Dosya</button>
      <button
        class={"menu-button" + (openMenu() === "edit-options" ? " active" : "")}
        type="button"
        data-menu="edit-options"
        onClick={(e) => { e.stopPropagation(); toggleMenu("edit-options"); }}
      >Düzenle</button>
      <button
        class={"menu-button" + (openMenu() === "view-options" ? " active" : "")}
        type="button"
        data-menu="view-options"
        onClick={(e) => { e.stopPropagation(); toggleMenu("view-options"); }}
      >Görünüm</button>

      <Show when={openMenu() === "file-options"}>
        <div id="file-options" class="menu-popover" role="menu" aria-label="Dosya menüsü" onClick={stop}>
          <button type="button" role="menuitem" onClick={onNew}><span>Yeni</span><span class="menu-key">Ctrl + N</span></button>
          <button type="button" role="menuitem" onClick={onClear}><span>Temizle</span><span class="menu-key">Ctrl + L</span></button>
        </div>
      </Show>

      <Show when={openMenu() === "edit-options"}>
        <div id="edit-options" class="menu-popover edit" role="menu" aria-label="Düzenle menüsü" onClick={stop}>
          <button type="button" role="menuitem" onClick={onSelectAll}><span>Tümünü seç</span><span class="menu-key">Ctrl + A</span></button>
          <button type="button" role="menuitem" onClick={onFind}><span>Bul</span><span class="menu-key">Ctrl + F</span></button>
        </div>
      </Show>

      <Show when={openMenu() === "view-options"}>
        <div id="view-options" class="menu-popover view" role="menu" aria-label="Görünüm menüsü" onClick={stop}>
          <button type="button" role="menuitem" onClick={onToggleWrap}><span>Kelime kaydırma</span><span id="wrap-check">{state.word_wrap ? "✓" : ""}</span></button>
        </div>
      </Show>
    </nav>
  );
}

export default MenuBar;