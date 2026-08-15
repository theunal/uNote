import { state, toggleSearch } from "../store";
import { svgSearch } from "../svg";

export function SearchBox() {
    return (
        <button class={"searchbox-trigger" + (state.searchOpen ? " open" : "")} type="button" onClick={toggleSearch} title="Notlarda ara">
            <span class="searchbox-trigger-icon" innerHTML={svgSearch()} />
            <span class="searchbox-trigger-label">Notlarda ara</span>
        </button>
    );
}
