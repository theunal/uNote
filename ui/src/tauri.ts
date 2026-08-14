// Tauri bridge — safe fallbacks so the UI also renders in a plain browser (npm run dev).
const tauri = window.__TAURI__;

export const invoke = tauri
  ? tauri.core.invoke
  : async (_cmd: string, _args?: Record<string, unknown>) => {
      console.warn("[unote] invoke stub: running outside Tauri —", _cmd);
      if (_cmd === "list_notes") return [];
      if (_cmd === "get_settings") return {};
      if (_cmd === "create_note") return 1;
      return undefined;
    };

const noopWindow = {
  minimize: async () => {},
  maximize: async () => {},
  unmaximize: async () => {},
  isMaximized: async () => false,
  close: async () => {},
  startDragging: async () => {},
};

export const appWindow = tauri ? tauri.window.getCurrentWindow() : noopWindow;