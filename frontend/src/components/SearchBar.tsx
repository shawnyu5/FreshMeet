import { useSearchParams } from "@solidjs/router";
import { createSignal } from "solid-js";

export function SearchBar() {
   const [searchParams, setSearchParams] = useSearchParams();
  const [searchQuery, setSearchQuery] = createSignal(searchParams.query);

  return (
    <form
      onSubmit={(e) => {
         e.preventDefault()
        setSearchParams({ query: searchQuery() });
      }}
    >
      <ul class="menu">
        <li>
          <input
            type="search"
            placeholder="Search"
            value={searchQuery()}
            onChange={(e) => setSearchQuery(e.target.value)}
          ></input>
        </li>
        <li>
          <button type="submit" class="button">
            Search
          </button>
        </li>
      </ul>
    </form>
  );
}

