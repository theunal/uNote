import { createStore } from "solid-js/store";
import { invoke } from "./tauri";
import { PASSWORD } from "./constants";
import type { Note, NoteTab, Settings, CtxMenu } from "./types";

export interface AppState {
  notes: Note[];
  tabs: NoteTab[];
  activeTab: number | null;
  searchQuery: string;
  searchOpen: boolean;
  showAppMenu: boolean;
  showAbout: boolean;
  ctx: CtxMenu | null;
  theme: string;
  font_family: string;
  font_style: string;
  font_size: number;
  word_wrap: boolean;
  formatting_enabled: boolean;
  restore_tabs: boolean;
  open_files_mode: string;
  recent_files: boolean;
  spell_check: boolean;
  autocorrect: boolean;
  writing_tools: boolean;
  dragIndex: number | null;
  settingsOpen: boolean;
  toast: string | null;
  cursor_line: number;
  cursor_col: number;
  char_count: number;
}

export const [state, setState] = createStore<AppState>({
  notes: [],
  tabs: [],
  activeTab: null,
  searchQuery: "",
  searchOpen: false,
  showAppMenu: false,
  showAbout: false,
  ctx: null,
  theme: "light",
  font_family: "Space Mono",
  font_style: "Regular",
  font_size: 14,
  word_wrap: true,
  formatting_enabled: false,
  restore_tabs: true,
  open_files_mode: "new",
  recent_files: true,
  spell_check: true,
  autocorrect: true,
  writing_tools: true,
  dragIndex: null,
  settingsOpen: false,
  toast: null,
  cursor_line: 1,
  cursor_col: 1,
  char_count: 0,
});

// ---------- Settings ----------
export async function loadSettings(): Promise<Settings> {
  try {
    const s = (await invoke("get_settings")) as Settings;
    setState("theme", s.theme || "light");
    setState("font_family", s.font_family || "Space Mono");
    setState("font_style", s.font_style || "Regular");
    setState("font_size", s.font_size || 14);
    setState("word_wrap", s.word_wrap !== false);
    setState("formatting_enabled", !!s.formatting_enabled);
    setState("restore_tabs", s.restore_tabs !== false);
    setState("open_files_mode", s.open_files_mode || "new");
    setState("recent_files", s.recent_files !== false);
    setState("spell_check", s.spell_check !== false);
    setState("autocorrect", s.autocorrect !== false);
    setState("writing_tools", s.writing_tools !== false);
    return s;
  } catch {
    return {};
  }
}

export function saveSettings() {
  invoke("save_settings", {
    s: {
      theme: state.theme,
      open_tab_ids: state.tabs.map(t => t.note_id).filter(id => id !== -1),
      font_family: state.font_family,
      font_style: state.font_style,
      font_size: state.font_size,
      word_wrap: state.word_wrap,
      formatting_enabled: state.formatting_enabled,
      restore_tabs: state.restore_tabs,
      open_files_mode: state.open_files_mode,
      recent_files: state.recent_files,
      spell_check: state.spell_check,
      autocorrect: state.autocorrect,
      writing_tools: state.writing_tools,
    },
  });
}

// ---------- Errors ----------
let toastTimer: number | undefined;

export function notifyError(err: unknown, fallback = "Bir hata oluştu") {
  const raw = err instanceof Error ? err.message : String(err ?? "");
  const msg = raw && !["undefined", "[object Object]"].includes(raw) ? raw : fallback;
  console.error("[unote]", msg);
  setState("toast", msg);
  window.clearTimeout(toastTimer);
  toastTimer = window.setTimeout(() => setState("toast", null), 4000);
}

// ---------- Data ----------
export async function refreshNotes() {
  try {
    const notes = (await invoke("list_notes", {
      args: {
        query: state.searchQuery,
        password: PASSWORD
      }
    })) as Note[];
    setState("notes", notes);
  } catch (err) {
    notifyError(err, "Notlar yüklenemedi");
  }
}

// ---------- Tabs ----------
export function tabOf(noteId: number) { return state.tabs.findIndex(t => t.note_id === noteId); }

export function openTab(note: Note, select = true) {
  let idx = tabOf(note.id);
  if (idx === -1) {
    idx = state.tabs.length;
    setState("tabs", t => [...t, {
      note_id: note.id,
      title: note.title,
      content: note.content,
      is_locked: note.is_locked
    }]);
  }
  if (select) setState("activeTab", idx);
  saveSettings();
}

export function closeTab(idx: number) {
  const tab = state.tabs[idx];
  const empty = !tab.content || !tab.content.trim();
  const next = state.tabs.filter((_, i) => i !== idx);
  let newActive: number | null;
  if (state.activeTab === null) {
    newActive = null;
  } else if (idx < state.activeTab) {
    newActive = state.activeTab - 1;
  } else if (idx === state.activeTab) {
    newActive = next.length ? Math.min(idx, next.length - 1) : null;
  } else {
    newActive = state.activeTab;
  }
  setState("tabs", next);
  setState("activeTab", newActive);
  if (empty) {
    invoke("delete_note", { id: tab.note_id })
      .then(refreshNotes)
      .catch((err) => notifyError(err, "Not silinemedi"));
  }
  saveSettings();
}

export function closeOtherTabs(idx: number) {
  const closing = state.tabs.filter((_, i) => i !== idx);
  for (const t of closing) {
    if (!t.content || !t.content.trim()) {
      invoke("delete_note", { id: t.note_id })
        .catch((err) => notifyError(err, "Not silinemedi"));
    }
  }
  setState("tabs", [state.tabs[idx]]);
  setState("activeTab", 0);
  saveSettings();
}

export function closeTabsToRight(idx: number) {
  const closing = state.tabs.slice(idx + 1);
  const next = state.tabs.slice(0, idx + 1);
  for (const t of closing) {
    if (!t.content || !t.content.trim()) {
      invoke("delete_note", { id: t.note_id })
        .catch((err) => notifyError(err, "Not silinemedi"));
    }
  }
  setState("tabs", next);
  setState("activeTab", Math.min(state.activeTab ?? 0, idx));
  saveSettings();
}

export async function newTab() {
  if (state.searchQuery)
    setState("searchQuery", "");

  try {
    const note = (await invoke("create_note", { password: PASSWORD })) as Note;
    setState("notes", n => [note, ...n]);
    openTab(note, state.activeTab === null);
  } catch (err) {
    notifyError(err, "Not oluşturulamadı");
  }
}

export function openSettings() {
  setState("settingsOpen", true);
  hideMenus();
}

export function closeSettings() {
  setState("settingsOpen", false);
}

export async function deleteNoteFromList(noteId: number) {
  try {
    await invoke("delete_note", { id: noteId });
    const idx = tabOf(noteId);
    if (idx !== -1) closeTab(idx);
    await refreshNotes();
    hideMenus();
  } catch (err) {
    notifyError(err, "Not silinemedi");
    hideMenus();
  }
}

export function dragReorderTabs(i: number) {
  if (state.dragIndex === null || state.dragIndex === i) return;
  const from = state.dragIndex;
  const active = state.activeTab;
  const next = [...state.tabs];
  const [tab] = next.splice(from, 1);
  next.splice(i, 0, tab);
  setState("tabs", next);
  if (active === from) setState("activeTab", i);
  else if (active !== null && from < active && i >= active) setState("activeTab", active - 1);
  else if (active !== null && from > active && i <= active) setState("activeTab", active + 1);
  setState("dragIndex", i);
  saveSettings();
}

// ---------- Menus ----------
export function hideMenus() {
  setState("showAppMenu", false);
  setState("ctx", null);
  setState("searchOpen", false);
}

// export function toggleAppMenu() {
//   setState("showAppMenu", !state.showAppMenu);
//   setState("ctx", null);
// }

export function toggleSearch() {
  setState("searchOpen", !state.searchOpen);
}

export function openSearch() {
  setState("searchOpen", true);
}

export function openContextMenu(x: number, y: number, noteId: number) {
  setState("showAppMenu", false);
  setState("ctx", { x, y, note_id: noteId });
}

export function openTabContextMenu(x: number, y: number, tabIndex: number) {
  setState("showAppMenu", false);
  setState("ctx", { x, y, tab_index: tabIndex });
}