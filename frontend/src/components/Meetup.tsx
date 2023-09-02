import axios, { AxiosResponse } from "axios";
import { marked } from "marked";
import { load } from "~/environment";
import {
  For,
  createSignal,
  Suspense,
  createResource,
  onMount,
  onCleanup,
} from "solid-js";
import Pagination from "./Pagination";
import logger from "~/logger";

export default function (props: { query: Array<string>; per_page: number }) {
  const [pageNumber, setPageNumber] = createSignal(1);
  const [afterCursor, setAfterCursor] = createSignal("");

  const [eventResource] = createResource(pageNumber, async () => {
    logger.info("Fetching events");
    const events = await searchAllQueries(
      props.query,
      props.per_page,
      afterCursor()
    );

    if (events.page_info.hasNextPage) {
      setAfterCursor(events.page_info.endCursor);
    }
    return events;
  });

  return (
    <Suspense fallback={<p>loading...</p>}>
      <div id="meetups">
        <table class="table is-striped">
          <thead>
            <tr>
              <th>Name</th>
              <th>date</th>
              <th>Attending</th>
              <th>description</th>
            </tr>
          </thead>
          <tbody>
            <For each={eventResource()?.nodes}>
              {(event, _) => (
                <tr>
                  <td>
                    <a target="_blank" href={event.eventUrl}>
                      {event.title}
                    </a>
                  </td>
                  <td>{event.dateTime}</td>
                  <td>{event.going}</td>
                  <Desciption description={event.description} />
                </tr>
              )}
            </For>
          </tbody>
        </table>

        <Pagination
          nextPageCallback={async () => {
            scrollToTop();
            setPageNumber((e) => e + 1);
          }}
          disableNextBtn={false}
        />
      </div>
    </Suspense>
  );
}

/** scroll the window to the top */
export function scrollToTop() {
  // Scroll to the top of the page smoothly
  window.scrollTo({
    top: 0,
    behavior: "smooth",
  });
}

/**
 * Render the markdown in the description
 *
 * @returns a `td` element containing the description
 */
export function Desciption(props: { description: string }) {
  marked.use({
    mangle: false,
    headerIds: false,
  });
  let rendered = marked.parse(props.description);
  return <td innerHTML={rendered}></td>;
}

/**
 * Perform a search for all queries, and accumulate into a single Meetup event
 * @param queries - list of queries to search for
 * @param per_page - number of results to return per page
 * @param after - after cursor
 * @returns a event object containing all of the search query results
 */
async function searchAllQueries(
  queries: Array<string>,
  per_page: number,
  after: string
): Promise<MeetupEvent> {
  let events: MeetupEvent = {
    nodes: [],
    page_info: { endCursor: "", hasNextPage: true },
  };
  for (const query of queries) {
    logger.info(`query: ${query}`);
    let searchResult = await searchEvents(query, per_page, after);
    logger.info(`search result: ${JSON.stringify(searchResult)}`);

    events.page_info = searchResult.page_info;
    events.nodes = events.nodes.concat(searchResult.nodes);
  }
  return events;
}

export async function searchEvents(
  query: string,
  per_page: number,
  after: string
): Promise<MeetupEvent> {
  logger.info("fetching events");
  try {
    let response: AxiosResponse<MeetupEvent> = await axios.post(
      `${load().api_url}/meetup/search`,
      // "http://localhost:8000/meetup/search",
      {
        query,
        per_page: per_page,
        after: after,
      }
    );
    return response.data;
  } catch (e: any) {
    console.log(e);
  }
}

// event response from backend api
export interface MeetupEvent {
  page_info: PageInfo;
  nodes: Array<Node>;
}

export interface PageInfo {
  hasNextPage: boolean;
  endCursor: string;
}

export interface Node {
  id: string;
  title: string;
  dateTime: string;
  endTime: string;
  description: string;
  duration: string;
  timezone: string;
  eventType: string;
  currency: string;
  eventUrl: string;
  going: number;
  isAttending: boolean;
  rsvpState: string;
}
