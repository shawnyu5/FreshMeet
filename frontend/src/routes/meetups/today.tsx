import axios, { AxiosResponse } from "axios";
import {
  createResource,
  createSignal,
  ErrorBoundary,
  For,
  Show,
  Suspense,
} from "solid-js";
import { loadConfig } from "~/config";
import logger from "~/logger";
import { MeetupEvents } from "./types";
import { useAppState } from "~/state";
import { useSearchParams } from "@solidjs/router";

/**
 * Render the markdown in the description
 *
 * @returns a `td` element containing the description
 */

export default function () {
  // const [pageNumber, setPageNumber] = createSignal(1);
  // const [afterCursor, setAfterCursor] = createSignal("");
  // const [getHasNextPage, setHasNextPage] = createSignal(true);
  const [searchParams, setSearchParams] = useSearchParams();
  const [appState, setAppState] = useAppState();

  const [eventResource] = createResource(async () => {
    const dateRange = appState.dateRange;
    const startDate = `${dateRange.value.startDateObject?.year}-${dateRange.value.startDateObject?.month ?? 0 + 1}-${dateRange.value.startDateObject?.day}[US/Estern]`;
    const endDate = `${dateRange.value.endDateObject?.year}-${dateRange.value.endDateObject?.month ?? 0 + 1}-${dateRange.value.endDateObject?.day}[US/Estern]`;

    if (searchParams.query) {
      logger.info(`Search query found: '${searchParams.query}'. Using search query`);
      const events = await searchMeetups(
        searchParams.query || null,
        startDate,
        endDate,
        100,
      );
      setAppState("events", events);
    } else {
      logger.info("No search query found. Getting recommended meetups");
      const events = await getRecommendedMeetups("");
      setAppState("events", events);
    }

    return appState.events;
  });

  // const scrollNext = async () => {
  //   logger.info("Loading more events");
  //   await refetchEvents();

  // TODO: this does not work due to a framework issue
  // https://github.com/solidjs/solid/issues/1864
  // const new_events = await getMeetupsToday(afterCursor());
  // mutateEvents((prev) => {
  //   if (prev) {
  //     logger.info("prev is not null");
  //     console.log(prev.data.rankedEvents.edges);
  //     const updated_edge = prev.data.rankedEvents.edges.concat(
  //       new_events?.data.rankedEvents.edges as Array<Edge>
  //     );

  //     prev.data.rankedEvents.edges = updated_edge;
  //     console.log(prev.data.rankedEvents.edges);
  //     return prev;
  //   } else {
  //     return new_events as MeetupsToday;
  //   }
  // });
  // };

  return (
    <Suspense fallback={<p>loading....</p>}>
      <ErrorBoundary fallback={(err) => err}>
        <div id="meetup-today">
          <Show
            when={eventResource()?.data.result.edges.length != 0}
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
                <For each={eventResource()?.data.result.edges}>
                  {(node, _idx) => (
                    <tr>
                      <td>
                        <a target="_blank" href={node.node.eventUrl}>
                          {node.node.title}
                        </a>
                      </td>
                      <td>{node.node.dateTime}</td>
                      <td>{node.node.isAttending.toString()}</td>
                      <td innerHTML={node.node.description}></td>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
          </Show>
        </div>
      </ErrorBoundary>
    </Suspense>
  );
}

/**
 * Fetch list of meetups recommended meetups for a date range
 * @param after - after cursor
 */
async function getRecommendedMeetups(after: string): Promise<MeetupEvents> {
  try {
    let response: AxiosResponse<MeetupEvents> = await axios.get(
      `${loadConfig().apiUrl}/today`,
      {
        params: {
          after: after,
        },
      },
    );
    logger.debug(`Response data: ${response.data}`);
    response.data.data.result.edges.map((edge) => {
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
    logger.error("Failed to make API request");
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
  startDate: string,
  endDate: string,
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
    logger.debug(`Response data: ${response.data}`);
    response.data.data.result.edges.map((edge) => {
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
    logger.error("Failed to make API request");
    return Promise.reject(`Failed to fetch events: ${err}`);
  }
}
