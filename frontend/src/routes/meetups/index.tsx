import axios, { AxiosResponse } from "axios";
import { appendLocalStorage } from "~/localStorage";
import "./index.css";
import {
  createEffect,
  createResource,
  createSignal,
  ErrorBoundary,
  For,
  Match,
  onMount,
  Show,
  Suspense,
  Switch,
} from "solid-js";
import { loadConfig } from "~/config";
import log from "~/logger";
import { MeetupEvents, MeetupEvents as RecommendedMeetups } from "./types";
import { useSearchParams } from "@solidjs/router";
import { PaginationButton } from "~/routes/meetups/PaginationButtons";

/*
The key in local storage used to store end cursors used for pagination
**/
const endCursorsLocalStorageKey = "endCursors";

export default function () {
  const [searchParams, _setSearchParams] = useSearchParams();
  // Create a signal that is derived from searchParams
  const [paramsSignal, setParamsSignal] = createSignal(searchParams);

  // On page load, we need to clear the local storage, otherwise it will keep growing
  onMount(() => {
    localStorage.setItem(endCursorsLocalStorageKey, "");
  });

  // Update the signal whenever searchParams changes
  createEffect(() => {
    setParamsSignal({ ...searchParams });
  });

  const [eventResource] = createResource(paramsSignal, async (paramsSignal) => {
    const startDate = new Date(paramsSignal.startDate as string);
    const endDate = new Date(paramsSignal.endDate as string);

    // If start or end date is null, it means date selection is in progress. Don't refetch events
    if (startDate == null || endDate == null) {
      return;
    }

    const query = paramsSignal.query as string;
    if (query) {
      log.info(`Searching for events using query: '${query}'`);
      log.info(`Start date: ${startDate}`);
      log.info(`End date: ${endDate}`);
      const events = await searchMeetups(query, startDate, endDate, 100);
      return events;
    } else {
      log.info(
        `Fetching recommended events for start date ${startDate} and end date ${endDate}`,
      );
      const events = await getRecommendedMeetups(startDate, endDate);
      return events;
    }
  });

  return (
    <Suspense fallback={<p>loading....</p>}>
      <ErrorBoundary fallback={(err) => err}>
        <div id="meetup-today">
          <Show
            when={eventResource()?.data?.result.edges.length != 0}
            fallback={<p>No Meetups for selected date... 🥲</p>}
          >
            <table class="hover">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Start date</th>
                  <th>Attending</th>
                  <th>description</th>
                </tr>
              </thead>
              <tbody>
                <For each={eventResource()?.data?.result.edges}>
                  {(node, _idx) => (
                    <tr>
                      <td>
                        <a target="_blank" href={node.node.eventUrl}>
                          {node.node.title}
                        </a>
                        <Show when={node.node.googleMapsUrl}>
                          <hr />
                          Location:{" "}
                          <a
                            target="_blank"
                            href={node.node.googleMapsUrl ?? ""}
                          >
                            {node.node.venue?.name}
                          </a>
                        </Show>
                      </td>
                      <td>{node.node.dateTime}</td>
                      <td>
                        {(
                          node.node.isAttendingStr ?? node.node.isAttending
                        ).toString()}
                      </td>
                      <td>
                        <Switch>
                          <Match
                            when={node.node.description.split("\n").length > 3}
                          >
                            <details>
                              <summary
                                innerHTML={node.node.description
                                  .split("\n")
                                  .slice(0, 2)
                                  .join("\n")}
                              />
                              <div
                                innerHTML={node.node.description
                                  .split("\n")
                                  .slice(3)
                                  .join("\n")}
                              />
                            </details>
                          </Match>
                          <Match
                            when={node.node.description.split("\n").length <= 3}
                          >
                            <div innerHTML={node.node.description} />
                          </Match>
                        </Switch>
                      </td>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
            <PaginationButton text="Next" onClick={() => {

            }}/>
            <PaginationButton text="Previous"/>
          </Show>
        </div>
      </ErrorBoundary>
    </Suspense>
  );
}

/**
 * Fetch list of meetups recommended meetups for the current date
 * @param startDate - the start date
 * @param endDate - the end date
 * @param after - the after cursor
 */
async function getRecommendedMeetups(
  startDate: Date,
  endDate: Date,
  after: string = "",
): Promise<RecommendedMeetups> {
  try {
    let response: AxiosResponse<RecommendedMeetups> = await axios.get(
      `${loadConfig().apiUrl}/recommended`,
      {
        params: {
          startDate: startDate.toISOString(),
          endDate: endDate.toISOString(),
          after: after,
        },
      },
    );
    log.debug(`Response data: ${response.data}`);
    if (response.data.data?.result.pageInfo.endCursor) {
      const localStorage = appendLocalStorage(
        endCursorsLocalStorageKey,
        response.data.data?.result.pageInfo.endCursor,
      );
      log.info(`Local storage end coursors: ${localStorage}`);
    }
    return response.data;
  } catch (err: unknown) {
    log.error("Failed to make API request");
    return Promise.reject(`Failed to fetch events: ${err}`);
  }
}

/**
 * Fetch meetup events
 *
 * @param query - Optional search query
 * @param startDate - Start date for event
 * @param endDate - end date for event
 * @param perPage - number of events to return
 */
async function searchMeetups(
  query: string | null,
  startDate: Date,
  endDate: Date,
  perPage: number,
): Promise<MeetupEvents> {
  try {
    let response: AxiosResponse<MeetupEvents> = await axios.post(
      `${loadConfig().apiUrl}/search`,
      {
        query: query,
        start_date: startDate,
        end_date: endDate,
        per_page: perPage,
      },
    );
    log.debug(`Response data: ${response.data}`);
    response.data.data?.result.edges.map((edge) => {
      if (edge.node.isAttending == true) {
        // Doing some weird things here... ignore this
        // @ts-ignore
        edge.node.isAttending = "Attending! 😀";
      } else if (edge.node.isAttending == false) {
        // @ts-ignore
        edge.node.isAttending = "Not attending... 🫠";
      }
    });

    return response.data;
  } catch (err: unknown) {
    log.error("Failed to make API request");
    return Promise.reject(`Failed to fetch events: ${err}`);
  }
}
