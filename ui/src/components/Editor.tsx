import { Show, createSignal } from "solid-js";
import { PASSWORD } from "../constants";
import { refreshNotes, setState, state } from "../store";
import { svgLock, svgX } from "../svg";
import { invoke } from "../tauri";
import { Input } from "./Input";

export function Editor(props: { noteId: number }) {
    const tab = () => state.tabs.find(t => t.note_id === props.noteId);
    let debounce: number | undefined;

    const [renaming, setRenaming] = createSignal(false);
    const [titleBuf, setTitleBuf] = createSignal("");

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

    const startRename = () => {
        const t = tab();
        if (!t || t.is_locked || t.note_id === -1) return;
        setTitleBuf(t.title);
        setRenaming(true);
    };

    const commitRename = () => {
        if (!renaming()) return;
        setRenaming(false);
        const t = tab();
        const buf = titleBuf().trim();
        if (!t || t.is_locked || t.note_id === -1 || !buf || buf === t.title) return;
        const idx = state.tabs.findIndex(x => x.note_id === t.note_id);
        if (idx !== -1) setState("tabs", idx, "title", buf);
        invoke("save_note_title", {
            args: { id: t.note_id, title: buf, is_locked: t.is_locked, password: PASSWORD },
        }).then(refreshNotes);
    };

    const cancelRename = () => setRenaming(false);

    return (
        <div class="editor-wrap">
            <div class="editor-meta">
                <Show when={!renaming()} fallback={
                    <div class="editor-title-edit">
                        <Input
                            variant="title"
                            ref={(el) => { el.focus(); el.select(); }}
                            value={titleBuf()}
                            onInput={(e) => setTitleBuf((e.target as HTMLInputElement).value)}
                            onClick={(e) => e.stopPropagation()}
                            onKeyDown={(e) => {
                                if (e.key === "Enter") commitRename();
                                if (e.key === "Escape") cancelRename();
                            }}
                            onBlur={commitRename}
                        />
                        <button
                            class="editor-title-cancel"
                            type="button"
                            aria-label="Başlık düzenlemeyi iptal et"
                            title="İptal"
                            onMouseDown={(e) => { e.preventDefault(); cancelRename(); }}
                            innerHTML={svgX()}
                        />
                    </div>
                }>
                    <div
                        class="editor-title"
                        title="Başlığı düzenlemek için çift tıklayın"
                        onDblClick={startRename}
                    >{tab()?.title || ""}</div>
                </Show>
                {tab()?.is_locked ? <span class="lock-badge"><span innerHTML={svgLock()} /> Kilitli</span> : null}
                <div class="saved-flag">Otomatik kaydedildi</div>
            </div>
            <Input
                variant="editor"
                multiline
                spellcheck={false}
                readonly={tab()?.is_locked}
                style={{ "font-size": state.font_size + "px", "white-space": state.word_wrap ? "pre-wrap" : "pre" }}
                value={tab()?.is_locked ? "* Bu not kilitli." : tab()?.content || ""}
                onInput={onInput}
            />
        </div>
    );
}
