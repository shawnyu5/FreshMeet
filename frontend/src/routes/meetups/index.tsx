import axios, { AxiosResponse } from "axios";
import "./index.css"
import {
  createEffect,
  createResource,
  createSignal,
  ErrorBoundary,
  For,
  Show,
  Suspense,
} from "solid-js";
import { loadConfig } from "~/config";
import log from "~/logger";
import { MeetupEvents } from "./types";
import { useSearchParams } from "@solidjs/router";

/**
 * Render the markdown in the description
 *
 * @returns a `td` element containing the description
 */

export default function () {
  const [searchParams, _setSearchParams] = useSearchParams();
  // Create a signal that is derived from searchParams
  const [paramsSignal, setParamsSignal] = createSignal(searchParams);

  // Update the signal whenever searchParams changes
  createEffect(() => {
    setParamsSignal({ ...searchParams });
  });

  const [eventResource] = createResource(paramsSignal, async (paramsSignal) => {
    const startDate = paramsSignal.startDate as string;
    const endDate = paramsSignal.endDate as string;

    // If start or end date is null, it means date selection is in progress. Don't refetch events
    if (startDate == null || endDate == null) {
      return;
    }

    const query = paramsSignal.query as string;
    if (query) {
      log.info(`Searching for events using query: '${query}'`);
      const events = await searchMeetups(
        query,
        startDate as string,
        endDate as string,
        100,
      );
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
                      <td>{(node.node.isAttendingStr ?? node.node.isAttending).toString()}</td>
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
 * Fetch list of meetups recommended meetups for the current date
 * @param after - after cursor
 */
async function getRecommendedMeetups(
  startDate: string,
  endDate: string,
): Promise<MeetupEvents> {
  try {
    let response: AxiosResponse<MeetupEvents> = await axios.get(
      `${loadConfig().apiUrl}/recommended`,
      {
        params: {
          startDate,
          endDate,
        },
      },
    );
    log.debug(`Response data: ${response.data}`);
    // // TODO: move this processing into the backend
    // response.data.data.result.edges.map((edge) => {
    //   if (edge.node.isAttending == true) {
    //     // Doing some weird things here... ignore this
    //     // @ts-ignore
    //     edge.node.isAttending = "Attending! 😀";
    //   } else if (edge.node.isAttending == false) {
    //     // @ts-ignore
    //     edge.node.isAttending = "Not attending... 🫠";
    //   }
    // });

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
    log.debug(`Response data: ${response.data}`);
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
    log.error("Failed to make API request");
    return Promise.reject(`Failed to fetch events: ${err}`);
  }
}
