import { onMount } from "solid-js";
import { PASSWORD } from "../../constants";
import { notifyError, setState, state } from "../../store";
import { svgX } from "../../svg";
import { invoke } from "../../tauri";
import "./Editor.scss";

export function Editor(props: { noteId: number }) {
    const tab = () => state.tabs.find(t => t.note_id === props.noteId);
    let debounce: number | undefined;

    onMount(() => {
        const el = document.getElementById("note-editor") as HTMLTextAreaElement | null;
        if (!el) return;
        const bold = state.font_style.includes("Bold");
        const italic = state.font_style.includes("Italic");
        el.style.fontFamily = `"${state.font_family}", sans-serif`;
        el.style.fontWeight = bold ? "700" : "400";
        el.style.fontStyle = italic ? "italic" : "normal";
    });

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
            }).catch((err) => notifyError(err, "Not kaydedilemedi"));
        }, 300);
    };

    return (
        <div class="editor-shell">
            <label for="note-editor" class="sr-only">Not metni</label>
            <textarea
                id="note-editor"
                class="note-editor"
                spellcheck={state.spell_check}
                aria-label="Not metni"
                readonly={tab()?.is_locked}
                style={{ "font-size": state.font_size + "px", "white-space": state.word_wrap ? "pre-wrap" : "pre" }}
                value={tab()?.is_locked ? "* Bu not kilitli." : tab()?.content || ""}
                onInput={onInput}
            ></textarea>
            <aside id="search-result" class="search-result" aria-live="polite">
                <span id="result-message"></span>
                <button class="result-close" id="result-close" type="button" aria-label="Arama sonucunu kapat" innerHTML={svgX()} />
            </aside>
        </div>
    );
}