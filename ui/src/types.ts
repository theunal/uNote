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
