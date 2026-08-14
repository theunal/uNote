export interface Note {
  id: number;
  title: string;
  content: string;
  is_locked: boolean;
}

export interface NoteTab {
  note_id: number;
  title: string;
  content: string;
  is_locked: boolean;
}

export interface Settings {
  theme?: string;
  open_tab_ids?: number[];
  font_size?: number;
  word_wrap?: boolean;
  formatting_enabled?: boolean;
  restore_tabs?: boolean;
}

export interface CtxMenu {
  x: number;
  y: number;
  note_id: number;
}

export interface RenameState {
  index: number;
  buf: string;
}

export interface State {
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
