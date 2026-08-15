import { Show, splitProps } from "solid-js";
import type { JSX } from "solid-js";
import "./Input.scss";

type InputProps = {
  class?: string;
  icon?: string;
  variant?: "default" | "search" | "title" | "editor";
  multiline?: boolean;
  ref?: (el: HTMLInputElement | HTMLTextAreaElement) => void;
} & Omit<JSX.InputHTMLAttributes<HTMLInputElement>, "class">;

export function Input(props: InputProps) {
  const [local, rest] = splitProps(props, ["class", "icon", "variant", "multiline", "ref"]);

  return (
    <div
      class={
        `input input--${local.variant || "default"}` +
        (local.icon ? " has-icon" : "") +
        (local.class ? ` ${local.class}` : "")
      }
    >
      <Show when={local.icon}>
        <span class="input__icon" innerHTML={local.icon} />
      </Show>
      {local.multiline ? (
        <textarea ref={local.ref} {...(rest as JSX.TextareaHTMLAttributes<HTMLTextAreaElement>)} class="input__field" />
      ) : (
        <input ref={local.ref} {...rest} class="input__field" />
      )}
    </div>
  );
}

export default Input;
