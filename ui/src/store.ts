import { createStore } from "solid-js/store";
import { invoke } from "./tauri";
import { PASSWORD } from "./constants";
import type { Note, NoteTab, Settings, CtxMenu, RenameState } from "./types";

export interface AppState {
  notes: Note[];
  tabs: NoteTab[];
  activeTab: number | null;
  selectedNote: number | null;
  searchQuery: string;
  sidebarCollapsed: boolean;
  showAppMenu: boolean;
  showAbout: boolean;
  ctx: CtxMenu | null;
  theme: string;
  font_size: number;
  word_wrap: boolean;
  formatting_enabled: boolean;
  restore_tabs: boolean;
  rename: RenameState | null;
  dragIndex: number | null;
}

export const [state, setState] = createStore<AppState>({
  notes: [],
  tabs: [],
  activeTab: null,
  selectedNote: null,
  searchQuery: "",
  sidebarCollapsed: false,
  showAppMenu: false,
  showAbout: false,
  ctx: null,
  theme: "light",
  font_size: 14,
  word_wrap: true,
  formatting_enabled: false,
  restore_tabs: true,
  rename: null,
  dragIndex: null,
});

// ---------- Settings ----------
export async function loadSettings(): Promise<Settings> {
  try {
    const s = (await invoke("get_settings")) as Settings;
    setState("theme", s.theme || "light");
    setState("font_size", s.font_size || 14);
    setState("word_wrap", s.word_wrap !== false);
    setState("formatting_enabled", !!s.formatting_enabled);
    setState("restore_tabs", s.restore_tabs !== false);
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
      font_size: state.font_size,
      word_wrap: state.word_wrap,
      formatting_enabled: state.formatting_enabled,
      restore_tabs: state.restore_tabs,
    },
  });
}

// ---------- Data ----------
export async function refreshNotes() {
  const notes = (await invoke("list_notes", { args: { query: state.searchQuery, password: PASSWORD } })) as Note[];
  setState("notes", notes);
}

// ---------- Tabs ----------
export function tabOf(noteId: number) { return state.tabs.findIndex(t => t.note_id === noteId); }

export function openTab(note: Note, select = true) {
  let idx = tabOf(note.id);
  if (idx === -1) {
    setState("tabs", t => [...t, { note_id: note.id, title: note.title, content: note.content, is_locked: note.is_locked }]);
    idx = state.tabs.length;
  }
  if (select) setState("activeTab", idx);
  setState("selectedNote", note.id);
  saveSettings();
}

export function closeTab(idx: number) {
  const tab = state.tabs[idx];
  const empty = !tab.content || !tab.content.trim();
  const next = state.tabs.filter((_, i) => i !== idx);
  setState("tabs", next);
  setState("activeTab", next.length ? Math.min(idx, next.length - 1) : null);
  if (empty && tab.note_id !== -1) {
    invoke("delete_note", { id: tab.note_id }).then(refreshNotes);
  }
  saveSettings();
}

export function openSettings() {
  const idx = state.tabs.findIndex(t => t.note_id === -1);
  if (idx === -1) {
    setState("tabs", t => [...t, { note_id: -1, title: "⚙  Ayarlar", content: "", is_locked: false }]);
    setState("activeTab", state.tabs.length);
  } else {
    setState("activeTab", idx);
  }
  hideMenus();
}

export async function deleteNoteFromList(noteId: number) {
  await invoke("delete_note", { id: noteId });
  const idx = tabOf(noteId);
  if (idx !== -1) closeTab(idx);
  await refreshNotes();
  hideMenus();
}

export async function commitRename() {
  if (!state.rename) return;
  const idx = state.rename.index;
  const buf = state.rename.buf.trim();
  setState("rename", null);
  const tab = state.tabs[idx];
  if (buf && tab && tab.note_id !== -1) {
    setState("tabs", idx, "title", buf);
    invoke("save_note_title", {
      args: { id: tab.note_id, title: buf, is_locked: tab.is_locked, password: PASSWORD },
    }).then(refreshNotes);
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
}

export function toggleAppMenu() {
  setState("showAppMenu", !state.showAppMenu);
  setState("ctx", null);
}

export function openContextMenu(x: number, y: number, noteId: number) {
  setState("showAppMenu", false);
  setState("ctx", { x, y, note_id: noteId });
}

export function startRename(index: number, buf: string) {
  setState("rename", { index, buf });
}