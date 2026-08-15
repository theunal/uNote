import { createSignal, createEffect, For, Show, onCleanup, onMount } from "solid-js";
import { svgChevronDown, svgSearch } from "../../svg";
import "./SelectBox.scss";

export type SelectOption = string | { value: string; label: string };

type SelectBoxProps = {
  value: string;
  options: SelectOption[];
  onChange: (v: string) => void;
  searchable?: boolean;
  compact?: boolean;
  placeholder?: string;
  minWidth?: string;
  class?: string;
  id?: string;
};

function toOpt(o: SelectOption): { value: string; label: string } {
  return typeof o === "string" ? { value: o, label: o } : o;
}

const MENU_GAP = 5;
const MENU_EST_H = 260;

export function SelectBox(props: SelectBoxProps) {
  const [open, setOpen] = createSignal(false);
  const [query, setQuery] = createSignal("");
  const [highlight, setHighlight] = createSignal(-1);
  const [menuStyle, setMenuStyle] = createSignal<{ top: string; left: string; width: string }>();
  let rootRef: HTMLDivElement | undefined;
  let searchRef: HTMLInputElement | undefined;
  let menuRef: HTMLDivElement | undefined;

  const searchable = () => props.searchable !== false;
  const opts = () => props.options.map(toOpt);
  const filtered = () => {
    const q = query().trim().toLowerCase();
    if (!searchable() || !q) return opts();
    return opts().filter((o) => o.label.toLowerCase().includes(q));
  };
  const currentLabel = () => opts().find((o) => o.value === props.value)?.label ?? props.value;

  const placeMenu = () => {
    if (!rootRef) return;
    const r = rootRef.getBoundingClientRect();
    const h = menuRef ? menuRef.getBoundingClientRect().height : MENU_EST_H;
    const vh = window.innerHeight;
    let top = r.bottom + MENU_GAP;
    if (top + h > vh - 8) top = Math.max(8, r.top - h - MENU_GAP);
    setMenuStyle({ top: `${top}px`, left: `${r.left}px`, width: `${r.width}px` });
  };

  const openMenu = (withSearch: boolean) => {
    setQuery("");
    setHighlight(-1);
    setOpen(true);
    placeMenu();
    if (withSearch && searchable() && searchRef) searchRef.focus();
  };
  const close = () => {
    setOpen(false);
    setQuery("");
    setHighlight(-1);
  };

  const onRootClick = (e: MouseEvent) => {
    const t = e.target as Element;
    if (t.closest(".selectbox-chev")) {
      if (open()) close(); else openMenu(false);
      return;
    }
    if (!open()) openMenu(true);
  };

  const select = (o: { value: string; label: string }) => {
    props.onChange(o.value);
    close();
  };

  const onKeyDown = (e: KeyboardEvent) => {
    if (!open()) {
      if (e.key === "Enter" || e.key === " " || e.key === "ArrowDown") {
        e.preventDefault();
        openMenu(true);
      }
      return;
    }
    if (e.key === "Escape") {
      e.stopPropagation();
      close();
      return;
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setHighlight((h) => (h + 1 < filtered().length ? h + 1 : h));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setHighlight((h) => (h - 1 >= 0 ? h - 1 : h));
    } else if (e.key === "Enter") {
      e.preventDefault();
      const list = filtered();
      if (list.length && highlight() >= 0) {
        select(list[highlight()]);
      }
    }
  };

  createEffect(() => {
    if (open() && menuRef) placeMenu();
  });

  const onDocDown = (e: MouseEvent) => {
    if (rootRef && !rootRef.contains(e.target as Node)) close();
  };
  const onDocScroll = () => {
    if (open()) placeMenu();
  };

  onMount(() => {
    document.addEventListener("mousedown", onDocDown);
    window.addEventListener("scroll", onDocScroll, true);
    window.addEventListener("resize", onDocScroll);
  });
  onCleanup(() => {
    document.removeEventListener("mousedown", onDocDown);
    window.removeEventListener("scroll", onDocScroll, true);
    window.removeEventListener("resize", onDocScroll);
  });

  return (
    <div
      class={
        "selectbox" +
        (props.compact ? " compact" : "") +
        (open() ? " open" : "") +
        (props.class ? ` ${props.class}` : "")
      }
      ref={rootRef}
      id={props.id}
      style={props.minWidth ? `min-width:${props.minWidth}` : undefined}
      onClick={onRootClick}
      onKeyDown={onKeyDown}
      role="combobox"
      aria-haspopup="listbox"
      aria-expanded={open()}
      tabindex="0"
    >
      <div class="selectbox-trigger">
        <span class="selectbox-value" title={currentLabel()}>{currentLabel()}</span>
        <span class="selectbox-chev" aria-hidden="true" innerHTML={svgChevronDown()} />
      </div>

      <Show when={open()}>
        <div class="selectbox-menu" ref={menuRef} role="listbox"  style={menuStyle()}>
          <Show when={searchable()}>
            <div class="selectbox-search">
              <span class="selectbox-search-icon" aria-hidden="true" innerHTML={svgSearch()} />
              <input
                ref={searchRef}
                class="selectbox-search-input"
                type="text"
                placeholder={props.placeholder ?? "Ara..."}
                value={query()}
                onInput={(e) => { setQuery((e.target as HTMLInputElement).value); setHighlight(-1); }}
              />
            </div>
          </Show>
          <div class="selectbox-list">
            <For each={filtered()}>
              {(o, i) => (
                <div
                  class={
                    "selectbox-option" +
                    (o.value === props.value ? " selected" : "") +
                    (highlight() === i() ? " highlighted" : "")
                  }
                  role="option"
                  aria-selected={o.value === props.value}
                  onClick={(e) => { e.stopPropagation(); select(o); }}
                >
                  <span class="selectbox-mark" aria-hidden="true"></span>
                  <span class="selectbox-option-label">{o.label}</span>
                </div>
              )}
            </For>
            <Show when={filtered().length === 0}>
              <div class="selectbox-empty">Sonuç bulunamadı</div>
            </Show>
          </div>
        </div>
      </Show>
    </div>
  );
}

export default SelectBox;