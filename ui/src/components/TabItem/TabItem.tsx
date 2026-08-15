import { closeTab, saveSettings, setState, state } from "../../store";
import { svgLock } from "../../svg";
import { trunc } from "../../util";
import "./TabItem.scss";

export function TabItem(props: {
    tab: {
        note_id: number;
        title: string;
        content: string;
        is_locked: boolean
    }; index: number
}) {
    const tab = props.tab;
    const i = props.index;

    const reorder = () => {
        if (state.dragIndex === null || state.dragIndex === i) return;
        const from = state.dragIndex;
        const active = state.activeTab;
        const next = [...state.tabs];
        const [moved] = next.splice(from, 1);
        next.splice(i, 0, moved);
        setState("tabs", next);
        if (active === from) setState("activeTab", i);
        else if (active !== null && from < active && i >= active) setState("activeTab", active - 1);
        else if (active !== null && from > active && i <= active) setState("activeTab", active + 1);
        setState("dragIndex", i);
        saveSettings();
    };

    return (
        <div
            class={"tab" + (i === state.activeTab ? " active" : "")}
            role="tab"
            tabIndex={0}
            aria-selected={(i === state.activeTab).toString() as "true" | "false"}
            onClick={() => setState("activeTab", i)}
            onMouseDown={(e) => {
                if ((e.target as HTMLElement).classList.contains("tab-close")) return;
                setState("dragIndex", i);
            }}
            onMouseEnter={reorder}
        >
            {tab.is_locked ? <span class="lock" innerHTML={svgLock()} /> : null}
            <span class="tab-title">{trunc(tab.title, 15)}</span>
            <span class="tab-close" role="button" onClick={(e) => {
                e.stopPropagation();
                closeTab(i);
            }}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 640" class="svg_icon">
                    <path d="M183.1 137.4C170.6 124.9 150.3 124.9 137.8 137.4C125.3 149.9 125.3 170.2 137.8 182.7L275.2 320L137.9 457.4C125.4 469.9 125.4 490.2 137.9 502.7C150.4 515.2 170.7 515.2 183.2 502.7L320.5 365.3L457.9 502.6C470.4 515.1 490.7 515.1 503.2 502.6C515.7 490.1 515.7 469.8 503.2 457.3L365.8 320L503.1 182.6C515.6 170.1 515.6 149.8 503.1 137.3C490.6 124.8 470.3 124.8 457.8 137.3L320.5 274.7L183.1 137.4z" /></svg>
            </span>
        </div>
    );
}
