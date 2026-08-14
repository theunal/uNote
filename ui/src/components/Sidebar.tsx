import { For, Show } from "solid-js";
import { state, setState, openTab, openContextMenu } from "../store";
import { AVATAR_COLORS } from "../constants";
import { trunc } from "../util";
import { svgLock } from "../svg";

export function Sidebar() {
  return (
    <aside class={"sidebar" + (state.sidebarCollapsed ? " collapsed" : "")} id="sidebar">
      <div class="sidebar-head">
        <div class="hamburger" id="btnHamburger" onClick={() => {
          setState("sidebarCollapsed", !state.sidebarCollapsed);
        }}>
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
            <line x1="3" y1="6" x2="21" y2="6" />
            <line x1="3" y1="12" x2="21" y2="12" />
            <line x1="3" y1="18" x2="21" y2="18" />
          </svg>
        </div>
        <Show when={!state.sidebarCollapsed}>
          <div class="sidebar-title">Notlar</div>
          <div class="sidebar-count" id="noteCount">{state.notes.length} not</div>
        </Show>
      </div>
      <div class="note-list" id="noteList">
        <Show when={state.notes.length === 0} fallback={
          <For each={state.notes}>{(n, i) => (
            <div
              class={"note-item" + (n.id === state.selectedNote ? " selected" : "")}
              role="button"
              tabIndex={0}
              onClick={() => openTab(n)}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") { e.preventDefault(); openTab(n); }
              }}
              onContextMenu={(e) => {
                e.preventDefault();
                openContextMenu(e.clientX, e.clientY, n.id);
              }}
            >
              <div class="avatar" style={{ background: AVATAR_COLORS[i() % 8] }} aria-hidden="true">{trunc(n.title, 3).toUpperCase()}</div>
              <Show when={!state.sidebarCollapsed}>
                <span class="note-title">{n.title}</span>
                {n.is_locked ? <span class="note-lock" aria-hidden="true" innerHTML={svgLock()} /> : null}
              </Show>
            </div>
          )}</For>
        }>
          <div class="empty-msg">{state.sidebarCollapsed ? "-" : "Henüz not yok"}</div>
        </Show>
      </div>
    </aside>
  );
}