import { refreshNotes, setState, state } from "../store";
import { svgSearch } from "../svg";
import { Input } from "./Input";

export function SearchBox() {
    return (
        <Input
            class="searchbox"
            variant="search"
            icon={svgSearch()}
            id="search"
            placeholder="Notlarda ara..."
            aria-label="Notlarda ara"
            autocomplete="off"
            value={state.searchQuery}
            onInput={async (e) => {
                setState("searchQuery", (e.target as HTMLInputElement).value);
                await refreshNotes();
            }}
        />
    );
}
