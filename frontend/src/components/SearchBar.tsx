import axios, { AxiosResponse } from "axios";
import { createSignal } from "solid-js";
import { loadConfig } from "~/config";
import { MeetupEvents } from "~/routes/meetups/types";

export function SearchForum() {
  const [searchQuery, setSearchQuery] = createSignal("");
  return (
    <form
      onSubmit={(e) => {
        onSubmit(e, searchQuery());
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

async function onSubmit(e: Event, searchQuery: string) {
  e.preventDefault();
  console.log("handling forum submission");
  let response: AxiosResponse<MeetupEvents> = await axios.post(
    `${loadConfig().apiUrl}/recommended`,
    {
      query: searchQuery,
      start_date: "2024-08-25T08:00:00-04:00[US/Eastern]",
      end_date: "2024-08-26T08:00:00-04:00[US/Eastern]",
    },
  );

  // __AUTO_GENERATED_PRINT_VAR_START__
  console.log("onSubmit response: %s", JSON.stringify(response)); // __AUTO_GENERATED_PRINT_VAR_END__

  return response.data;
}
