import { state } from "../../store";
import { svgSun, svgMoon, svgMonitor } from "../../svg";
import "./StatusBar.scss";

function ThemeIndicator() {
  const label = () => state.theme === "dark" ? "Koyu" : state.theme === "light" ? "Açık" : "Sistem";
  const icon = () => state.theme === "dark" ? svgMoon() : state.theme === "light" ? svgSun() : svgMonitor();
  return (
    <span class="theme-ind" id="sbTheme"><span innerHTML={icon()} /> {label()}</span>
  );
}

export function StatusBar() {
  return (
    <div class="statusbar">
      <div id="sbPos">St {state.cursor_line}, Süt {state.cursor_col}</div>
      <div class="ml-1">
        {state.char_count} karakter
      </div>
      <ThemeIndicator />
    </div>
  );
}