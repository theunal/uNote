import { createEffect, createSignal, onCleanup, onMount, Show } from "solid-js";
import {
  state, newTab,
} from "../../store";
import { appWindow } from "../../tauri";
import "./TitleBar.scss";
import { TabItem } from "../TabItem/TabItem";
import { SearchBox } from "../SearchBox/SearchBox";
import { NoteSearchPanel } from "../NoteSearchPanel/NoteSearchPanel";
import { WindowControls } from "../WindowControls/WindowControls";

export function TitleBar() {
  let tabsScroller: HTMLDivElement | undefined;
  const [hasOverflow, setHasOverflow] = createSignal(false);
  const [canScrollLeft, setCanScrollLeft] = createSignal(false);
  const [canScrollRight, setCanScrollRight] = createSignal(false);

  const layoutTabs = () => {
    const el = tabsScroller;
    if (!el) return;
    const n = el.querySelectorAll(".tab").length;
    if (n === 0) return;
    const available = el.clientWidth;
    if (available <= 0) return;
    const gap = 2;
    const visible = Math.min(10, n);
    let w = (available - gap * (visible - 1)) / visible;
    if (w > 200) w = 200;
    if (w < 1) w = 1;
    el.style.setProperty("--tab-w", w + "px");
  };

  const updateScrollArrows = () => {
    const el = tabsScroller;
    if (!el) return;
    layoutTabs();
    setHasOverflow(el.scrollWidth > el.clientWidth + 1);
    setCanScrollLeft(el.scrollLeft > 1);
    setCanScrollRight(el.scrollLeft + el.clientWidth < el.scrollWidth - 1);
    updateActiveTabPos();
  };

  const updateActiveTabPos = () => {
    const el = tabsScroller;
    if (!el) return;
    const titlebar = el.closest<HTMLElement>(".titlebar");
    if (!titlebar) return;
    const active = el.querySelector<HTMLElement>(".tab.active");
    if (!active) {
      titlebar.style.setProperty("--active-l", "0px");
      titlebar.style.setProperty("--active-w", "0px");
      return;
    }
    const tr = titlebar.getBoundingClientRect();
    const ar = active.getBoundingClientRect();
    titlebar.style.setProperty("--active-l", (ar.left - tr.left) + "px");
    titlebar.style.setProperty("--active-w", ar.width + "px");
  };

  const keepEndInView = () => {
    const el = tabsScroller;
    if (!el || el.scrollWidth <= el.clientWidth + 1) return;
    el.scrollLeft = el.scrollWidth - el.clientWidth;
  };

  createEffect(() => {
    state.tabs.length;
    state.activeTab;
    requestAnimationFrame(() => {
      updateScrollArrows();
      keepEndInView();
    });
  });

  onMount(() => {
    const el = tabsScroller;
    if (!el) return;
    const ro = new ResizeObserver(() => requestAnimationFrame(() => {
      updateScrollArrows();
      keepEndInView();
    }));
    ro.observe(el);
    onCleanup(() => ro.disconnect());
  });

  const scrollTabs = (dir: number) => {
    const el = tabsScroller;
    if (!el) return;
    const first = el.querySelector<HTMLElement>(".tab");
    const step = first ? first.offsetWidth + 2 : 240;
    el.scrollBy({ left: dir * step, behavior: "smooth" });
  };

  const isInteractive = (t: Element) => t.closest(".tab") || t.closest(".icon-btn") ||
    t.closest(".wc-btn") || t.closest(".searchbox") || t.closest(".tab-chevron") || t.closest("input");

  const toggleMaximize = async () => {
    const isMaximized = await appWindow.isMaximized();
    if (isMaximized)
      await appWindow.unmaximize();
    else
      await appWindow.maximize();
  };

  const onDragDown = (e: MouseEvent) => {
    if (e.button !== 0 || isInteractive(e.target as Element)) return;
    if (e.detail === 2) {
      e.preventDefault();
      void toggleMaximize();
      return;
    }
    appWindow.startDragging();
  };

  return (
    <div class="titlebar" onMouseDown={onDragDown}>
      <div class="drag">
        <div class="tabs-wrap">
          <Show when={hasOverflow()}>
            <button class="tab-chevron" type="button" disabled={!canScrollLeft()}
              aria-label="Sekmeleri sola kaydır" onClick={() => scrollTabs(-1)}
            >
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                <path
                  d="M199.7 299.8C189.4 312.4 190.2 330.9 201.9 342.6L329.9 470.6C339.1 479.8 352.8 482.5 364.8 477.5C376.8 472.5 384.6 460.9 384.6 447.9L384.6 191.9C384.6 179 376.8 167.3 364.8 162.3C352.8 157.3 339.1 160.1 329.9 169.2L201.9 297.2L199.7 299.6z" />
              </svg>
            </button>
          </Show>
          <div class="tabs" id="tabs" ref={tabsScroller} onScroll={updateScrollArrows}>
            {state.tabs.map((tab, i) =>
              <TabItem tab={tab} index={i} />
            )}

            {/* <TabItem tab={{
              content: "", is_locked: false, note_id: -1, title: "Yeni Sekme"
            }} index={5} /> */}
          </div>
          <Show when={hasOverflow()}>
            <button class="tab-chevron" type="button" disabled={!canScrollRight()}
              aria-label="Sekmeleri sağa kaydır" onClick={() => scrollTabs(1)}
            >
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640">
                <path
                  d="M441.3 299.8C451.5 312.4 450.8 330.9 439.1 342.6L311.1 470.6C301.9 479.8 288.2 482.5 276.2 477.5C264.2 472.5 256.5 460.9 256.5 448L256.5 192C256.5 179.1 264.3 167.4 276.3 162.4C288.3 157.4 302 160.2 311.2 169.3L439.2 297.3L441.4 299.7z" />
              </svg>
            </button>
          </Show>
          <div class="icon-btn add-tab" id="btnAdd" onClick={newTab}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" class="svg_icon">
              <path d="M352 128C352 110.3 337.7 96 320 96C302.3 96 288 110.3 288 128L288 288L128 288C110.3 288 96 302.3 96 320C96 337.7 110.3 352 128 352L288 352L288 512C288 529.7 302.3 544 320 544C337.7 544 352 529.7 352 512L352 352L512 352C529.7 352 544 337.7 544 320C544 302.3 529.7 288 512 288L352 288L352 128z" /></svg>
          </div>
        </div>
        {/* <div class="icon-btn" id="btnMenu" title="Menü" onClick={(e) => {
          e.stopPropagation();
          toggleAppMenu();
        }}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" class="svg_icon">
            <path d="M297.4 470.6C309.9 483.1 330.2 483.1 342.7 470.6L534.7 278.6C547.2 266.1 547.2 245.8 534.7 233.3C522.2 220.8 501.9 220.8 489.4 233.3L320 402.7L150.6 233.4C138.1 220.9 117.8 220.9 105.3 233.4C92.8 245.9 92.8 266.2 105.3 278.7L297.3 470.7z" /></svg>
        </div> */}
      </div>
      <div class="tbar-right">
        <SearchBox />
      </div>
      <NoteSearchPanel />
      <WindowControls />
    </div>
  );
}