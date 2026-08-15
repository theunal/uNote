import { createSignal } from "solid-js";
import { saveSettings, setState, state } from "../store";
import { svgAlignCenter, svgAlignLeft, svgAlignRight, svgList } from "../svg";

const editorEl = () => document.getElementById("note-editor") as HTMLTextAreaElement | null;

export function Toolbar() {
  const [font, setFont] = createSignal("Space Mono");
  const [size, setSize] = createSignal(state.font_size);
  const [active, setActive] = createSignal<Record<string, boolean>>({});

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
    const el = editorEl();
    if (!el) return;
    el.style.fontFamily = value === "DM Sans"
      ? '"DM Sans", sans-serif'
      : value === "Georgia"
        ? "Georgia, serif"
        : '"Space Mono", monospace';
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
      <select class="font-picker" id="font-picker" value={font()} onChange={(e) => onFontChange((e.target as HTMLSelectElement).value)}>
        <option>Space Mono</option>
        <option>DM Sans</option>
        <option>Georgia</option>
      </select>
      <label for="size-picker" class="sr-only">Yazı boyutu</label>
      <select class="size-picker" id="size-picker" value={String(size())} onChange={(e) => onSizeChange(+(e.target as HTMLSelectElement).value)}>
        <option value="12">12</option>
        <option value="13">13</option>
        <option value="14">14</option>
        <option value="16">16</option>
        <option value="18">18</option>
      </select>
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