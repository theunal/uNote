import { state } from "../../store";
import { svgSun, svgMoon, svgMonitor } from "../../svg";
import { VERSION } from "../../constants";
import "./StatusBar.scss";

function ThemeIndicator() {
  const label = () => state.theme === "dark" ? "Koyu" : state.theme === "light" ? "Açık" : "Sistem";
  const icon = () => state.theme === "dark" ? svgMoon() : state.theme === "light" ? svgSun() : svgMonitor();
  return (
    <span class="theme-ind" id="sbTheme"><span innerHTML={icon()} /> {label()}</span>
  );
}

export function StatusBar() {
  const activeTitle = () => {
    if (state.activeTab !== null && state.tabs[state.activeTab]) return state.tabs[state.activeTab].title;
    return "Notlar";
  };

  return (
    <div class="statusbar">
      <span id="sbVersion">uNote v{VERSION}</span>
      <span class="sep">|</span>
      <span id="sbCount">{state.notes.length} not</span>
      <span class="sep">|</span>
      <span id="sbActive">{activeTitle()}</span>
      <ThemeIndicator />
    </div>
  );
}