import { createSignal, onMount } from "solid-js";
import { saveSettings, setState, state } from "../../store";
import { svgAlignCenter, svgAlignLeft, svgAlignRight, svgList } from "../../svg";
import { invoke } from "../../tauri";
import { SelectBox } from "../SelectBox/SelectBox";
import "./Toolbar.scss";

const editorEl = () => document.getElementById("note-editor") as HTMLTextAreaElement | null;

const DEFAULT_FONTS = ["Space Mono", "DM Sans", "Georgia"];
const SIZES = ["12", "13", "14", "16", "18"];

export function Toolbar() {
  const [font, setFont] = createSignal(state.font_family);
  const [size, setSize] = createSignal(state.font_size);
  const [active, setActive] = createSignal<Record<string, boolean>>({});
  const [fonts, setFonts] = createSignal<string[]>(DEFAULT_FONTS);

  onMount(async () => {
    try {
      const all = (await invoke("list_fonts")) as string[];
      if (all && all.length) {
        const merged = [...DEFAULT_FONTS];
        for (const f of all) {
          if (!merged.includes(f)) merged.push(f);
        }
        setFonts(merged);
      }
    } catch (err) {
      console.warn("[unote] font listesi yüklenemedi", err);
    }
    const el = editorEl();
    if (el && state.font_family) {
      el.style.fontFamily = `"${state.font_family}", sans-serif`;
    }
  });

  const toggleCmd = (cmd: string) => {
    const el = editorEl();
    if (!el) return;
    const on = !!active()[cmd];
    setActive({ ...active(), [cmd]: !on });
    if (cmd === "bold") {
      el.style.fontWeight = on ? "400" : "700";
    } else if (cmd === "italic") {
      el.style.fontStyle = on ? "normal" : "italic";
    } else if (cmd === "underline") {
      el.style.textDecoration = on ? "none" : "underline";
    } else if (cmd === "justifyLeft") {
      el.style.textAlign = "left";
    } else if (cmd === "justifyCenter") {
      el.style.textAlign = "center";
    } else if (cmd === "justifyRight") {
      el.style.textAlign = "right";
    } else if (cmd === "insertUnorderedList") {
      const start = el.selectionStart;
      el.setRangeText("• ", start, start, "end");
    }
    el.focus();
  };

  const onFontChange = (value: string) => {
    setFont(value);
    setState("font_family", value);
    saveSettings();
    const el = editorEl();
    if (!el) return;
    el.style.fontFamily = `"${value}", sans-serif`;
    el.focus();
  };

  const onSizeChange = (value: number) => {
    setSize(value);
    setState("font_size", value);
    saveSettings();
  };

  return (
    <section class="toolbar" aria-label="Biçim araçları">
      <label for="font-picker" class="sr-only">Yazı tipi</label>
      <SelectBox id="font-picker" value={font()} options={fonts()} compact searchable minWidth="108px"
        ariaLabel="Yazı tipi" onChange={onFontChange} />
      <label for="size-picker" class="sr-only">Yazı boyutu</label>
      <SelectBox id="size-picker" value={String(size())} options={SIZES} compact searchable minWidth="47px"
        ariaLabel="Yazı boyutu" onChange={(v) => onSizeChange(+v)} />
      <span class="toolbar-divider" aria-hidden="true"></span>
      <button class={"tool-button text-tool bold" + (active().bold ? " active" : "")} type="button" data-command="bold" aria-label="Kalın" onClick={() => toggleCmd("bold")}><strong>B</strong></button>
      <button class={"tool-button text-tool italic" + (active().italic ? " active" : "")} type="button" data-command="italic" aria-label="İtalik" onClick={() => toggleCmd("italic")}>I</button>
      <button class={"tool-button text-tool underline" + (active().underline ? " active" : "")} type="button" data-command="underline" aria-label="Altı çizili" onClick={() => toggleCmd("underline")}>U</button>
      <span class="toolbar-divider" aria-hidden="true"></span>
      <button class={"tool-button" + (active().justifyLeft ? " active" : "")} type="button" data-command="justifyLeft" aria-label="Sola hizala" onClick={() => toggleCmd("justifyLeft")} innerHTML={svgAlignLeft()} />
      <button class={"tool-button" + (active().justifyCenter ? " active" : "")} type="button" data-command="justifyCenter" aria-label="Ortala" onClick={() => toggleCmd("justifyCenter")} innerHTML={svgAlignCenter()} />
      <button class={"tool-button" + (active().justifyRight ? " active" : "")} type="button" data-command="justifyRight" aria-label="Sağa hizala" onClick={() => toggleCmd("justifyRight")} innerHTML={svgAlignRight()} />
      <button class="tool-button" type="button" data-command="insertUnorderedList" aria-label="Madde işaretleri" onClick={() => toggleCmd("insertUnorderedList")} innerHTML={svgList()} />
    </section>
  );
}

export default Toolbar;