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
  font_family?: string;
  font_style?: string;
  font_size?: number;
  word_wrap?: boolean;
  formatting_enabled?: boolean;
  restore_tabs?: boolean;
  open_files_mode?: string;
  recent_files?: boolean;
  spell_check?: boolean;
  autocorrect?: boolean;
  writing_tools?: boolean;
}

export interface CtxMenu {
  x: number;
  y: number;
  note_id?: number;
  tab_index?: number;
}
