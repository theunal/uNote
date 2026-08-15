import { Show } from "solid-js";
import { state, } from "../../store";
import "./Main.scss";
import { Editor } from "../Editor/Editor";

export function Main() {
  const activeNoteId = () => {
    if (state.activeTab === null || state.activeTab === -1) return null;
    const t = state.tabs[state.activeTab];
    return t ? t.note_id : null;
  };

  return (
    <main class="main" id="mainView">
      <Show when={activeNoteId()} keyed fallback={<div class="hero-empty">
        <div class="logo">
          uNote
        </div>
        <p>
          Bir not seçin veya yeni bir not oluşturun
        </p>
      </div>
      }>
        {(id) => <Editor noteId={id} />}
      </Show>
    </main>
  );
}