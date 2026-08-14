/// <reference types="vite/client" />

interface TauriCore {
  invoke(cmd: string, args?: Record<string, unknown>): Promise<any>;
}

interface TauriWindow {
  getCurrentWindow(): {
    minimize(): Promise<void>;
    maximize(): Promise<void>;
    unmaximize(): Promise<void>;
    isMaximized(): Promise<boolean>;
    close(): Promise<void>;
    startDragging(): Promise<void>;
  };
}

interface Window {
  __TAURI__?: {
    core: TauriCore;
    window: TauriWindow;
  };
}