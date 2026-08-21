import { Show } from "solid-js";
import {
  state, setState, deleteNoteFromList, closeOtherTabs, closeTabsToRight,
  openSettings,
  closeTab
} from "../../store";
import { trunc } from "../../util";
import { svgGear, svgInfo, svgTrash } from "../../svg";
import "./Overlays.scss";

export function Overlays() {
  const ctxNote = () => state.ctx ? state.notes.find(n => n.id === state.ctx!.note_id) : null;
  const ctxTabIndex = () => state.ctx?.tab_index;

  const closeOtherTabsHandler = () => {
    setState("ctx", null);
    closeOtherTabs(ctxTabIndex()!);
  }

  const closeTabsToRightHandler = () => {
    setState("ctx", null);
    closeTabsToRight(ctxTabIndex()!);
  }

  const closeTabHandler = () => {
    console.log("Closing tab from overlay", ctxTabIndex());
    closeTab(ctxTabIndex()!);
    setState("ctx", null);
  }

  return (
    <>
      <Show when={state.showAppMenu}>
        <div class="menu" id="appMenu" style={{ left: "60px", top: "44px" }}>
          <div class="menu-item" onClick={() => openSettings()}>
            <span class="mi-ico" innerHTML={svgGear()} />
            Ayarlar
          </div>
          <div class="menu-item" onClick={() => setState("showAbout", true)}>
            <span class="mi-ico" innerHTML={svgInfo()} />
            Hakkında
          </div>
        </div>
      </Show>

      <Show when={ctxTabIndex() !== undefined}>
        <div class="menu" id="ctxMenu" style={{
          left: state.ctx!.x + "px",
          top: state.ctx!.y + "px"
        }}>
          {/* <div class="menu-sep"></div> */}

          <div class="menu-item" onClick={closeTabHandler}>
            Kapat
          </div>
          <div class={"menu-item" + (state.tabs.length <= 1 ? " disabled" : "")}
            onClick={closeOtherTabsHandler}>
            Diğerlerini kapat
          </div>
          <div class={"menu-item" + (ctxTabIndex()! >= state.tabs.length - 1 ? " disabled" : "")}
            onClick={closeTabsToRightHandler}>
            Sağdakileri kapat
          </div>
        </div>
      </Show>

      <Show when={state.ctx && ctxTabIndex() === undefined}>
        <div class="menu" id="ctxMenu" style={{ left: state.ctx!.x + "px", top: state.ctx!.y + "px" }}>
          <div class="menu-head">{trunc(ctxNote()?.title || "", 20)}</div>
          <div class="menu-sep"></div>
          <div class="menu-item danger" onClick={() => deleteNoteFromList(state.ctx!.note_id!)}>
            <span class="mi-ico" innerHTML={svgTrash()} />
            Notu Sil
          </div>
        </div>
      </Show>

      <Show when={state.showAbout}>
        <div class="overlay">
          <div class="dialog" role="dialog" aria-modal="true" >
            <div class="d-logo">uNote</div>
            <h3>Güvenli Not Alma Kasası</h3>
            <p>Notlarınız AES-256-GCM ile şifrelenir ve yerel olarak saklanır.</p>
            <div class="version">Sürüm 0.1.0 · Rust + Tauri</div>
            <button class="btn primary dialog-ok" onClick={() => setState("showAbout", false)}>Tamam</button>
          </div>
        </div>
      </Show>
    </>
  );
}