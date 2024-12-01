import { PickerValue } from "@rnwonder/solid-date-picker";
import { useSearchParams } from "@solidjs/router";
import axios, { AxiosResponse } from "axios";
import { createResource, createSignal } from "solid-js";
import { loadConfig } from "~/config";
import log from "~/logger";
import { MeetupEvents } from "~/routes/meetups/types";
import { State, useAppState } from "~/state";

export function SearchBar(props: { dateRange: PickerValue }) {
  const [searchQuery, setSearchQuery] = createSignal("");
  const [appState, setAppState] = useAppState();
  const [searchParams, setSearchParams] = useSearchParams();

  return (
    <form
      onSubmit={(e) => {
        onSubmit(e, searchQuery(), props.dateRange);
        setAppState("query", searchQuery());
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

async function onSubmit(e: Event, searchQuery: string, dateRange: PickerValue) {
  e.preventDefault();

  const [_, setAppState] = useAppState();
  setAppState("query", searchQuery);

  const startDate = `${dateRange.value.startDateObject?.year}-${dateRange.value.startDateObject?.month ?? 0 + 1}-${dateRange.value.startDateObject?.day}[US/Estern]`;
  const endDate = `${dateRange.value.endDateObject?.year}-${dateRange.value.endDateObject?.month ?? 0 + 1}-${dateRange.value.endDateObject?.day}[US/Estern]`;

  log.info("handling forum submission");
  let response: AxiosResponse<MeetupEvents> = await axios.post(
    `${loadConfig().apiUrl}/search`,
    {
      query: searchQuery,
      start_date: startDate,
      end_date: endDate,
      per_page: 100,
    },
  );

  setAppState("events", response.data);
}
