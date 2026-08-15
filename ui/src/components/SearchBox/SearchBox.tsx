import { state, toggleSearch } from "../../store";
import { svgSearch } from "../../svg";
import "./SearchBox.scss";

export function SearchBox() {
    return (
        <button class={"searchbox-trigger" + (state.searchOpen ? " open" : "")} type="button" onClick={toggleSearch}
        >
            <span class="searchbox-trigger-icon" innerHTML={svgSearch()} />
            <span class="searchbox-trigger-label">Notlarda ara</span>
        </button>
    );
}
