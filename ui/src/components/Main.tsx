import { Show } from "solid-js";
import { state, setState } from "../store";
import { invoke } from "../tauri";
import { PASSWORD } from "../constants";
import { svgLock } from "../svg";
import SettingsView from "./Settings";

function Editor(props: { noteId: number }) {
  const tab = () => state.tabs.find(t => t.note_id === props.noteId);
  let debounce: number | undefined;

  const onInput = (e: Event) => {
    const t = tab();
    if (!t) return;
    const ta = e.currentTarget as HTMLTextAreaElement;
    clearTimeout(debounce);
    debounce = window.setTimeout(() => {
      const content = ta.value;
      const idx = state.tabs.findIndex(x => x.note_id === t.note_id);
      if (idx !== -1) setState("tabs", idx, "content", content);
      invoke("save_note_content", {
        args: { id: t.note_id, content, is_locked: t.is_locked, password: PASSWORD },
      });
    }, 300);
  };

  return (
    <div class="editor-wrap">
      <div class="editor-meta">
        <div class="editor-title">{tab()?.title || ""}</div>
        {tab()?.is_locked ? <span class="lock-badge"><span innerHTML={svgLock()} /> Kilitli</span> : null}
        <div class="saved-flag">Otomatik kaydedildi</div>
      </div>
      <textarea
        class="editor"
        spellcheck={false}
        readonly={tab()?.is_locked}
        style={{ "font-size": state.font_size + "px", "white-space": state.word_wrap ? "pre-wrap" : "pre" }}
        value={tab()?.is_locked ? "* Bu not kilitli." : tab()?.content || ""}
        onInput={onInput}
      />
    </div>
  );
}

export function MainView() {
  const activeNoteId = () => {
    if (state.activeTab === null || state.activeTab === -1) return null;
    const t = state.tabs[state.activeTab];
    return t ? t.note_id : null;
  };

  return (
    <main class="main" id="mainView">
      <Show when={state.activeTab === -1} fallback={
        <Show when={activeNoteId()} keyed fallback={<div class="hero-empty"><div class="logo">uNote</div><p>Bir not seçin veya yeni bir not oluşturun</p></div>}>
          {(id) => <Editor noteId={id} />}
        </Show>
      }>
        <SettingsView />
      </Show>
    </main>
  );
}