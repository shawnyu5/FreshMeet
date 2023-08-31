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
  const { query, per_page } = props;
  const [pageNumber, setPageNumber] = createSignal(1);
  const [eventResource] = createResource(pageNumber, async () => {
    logger.info("Fetching events");
    const events = await searchAllQueries(query, per_page, lastCursor());

    logger.info(`End cursor: ${events.page_info.endCursor}`);
    if (events.page_info.endCursor != "") {
      logger.info("Appending cursor to local storage");
      appendCursors(events.page_info.endCursor);
    }

    return events;
  });

  onMount(async () => {
    logger.info("OnMount clear cursor in local storage");
    // clear the after list in local storage on mount
    setCursors([]);
  });
  onCleanup(async () => {
    logger.info("OnCleanup clear cursor in local storage");
    setCursors([]);
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
          previousPageCallback={async () => {
            scrollToTop();
            // remove the last 2 cursor from the cursor list
            let cursorArr = getCursors();
            cursorArr.pop();
            cursorArr.pop();
            setCursors(cursorArr);

            setPageNumber((e) => e - 1);
          }}
        />
      </div>
    </Suspense>
  );
}

/** scroll the window to the top */
function scrollToTop() {
  // Scroll to the top of the page smoothly
  window.scrollTo({
    top: 0,
    behavior: "smooth",
  });
}

/**
 * appends the current cursor to the after list in local storage
 * @param cursor - the current cursor
 */
function appendCursors(cursor: string) {
  let currentCursors = localStorage.getItem("after") || "";
  if (currentCursors == "") {
    localStorage.setItem("after", cursor);
  } else {
    let currentCursorArray = stringToArray(currentCursors);
    currentCursorArray.push(cursor);
    setCursors(currentCursorArray);
  }
}

/**
 * @returns array of cursors in local storage
 */
function getCursors(): Array<string> {
  let cursors = localStorage.getItem("after");
  return stringToArray(cursors || "");
}

/**
 * sets the after list in local storage
 * @param cursors - array of cursors to set in local storage
 */
function setCursors(cursors: Array<string>) {
  // @ts-ignore
  localStorage.setItem("after", cursors);
}

/**
 * @returns the last cursor in the after list in local storage
 */
function lastCursor(): string {
  let afterString = localStorage.getItem("after");
  if (!afterString) {
    return "";
  }
  let afterArr = stringToArray(afterString);
  return afterArr[afterArr.length - 1];
}

/**
 * render the markdown in the description
 *
 * @returns a `td` element containing the description
 */
function Desciption(props: { description: string }) {
  marked.use({
    mangle: false,
    headerIds: false,
  });
  let rendered = marked.parse(props.description);
  return <td innerHTML={rendered}></td>;
}

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
/**
 * converts a comma separated string to an array of strings
 * @param str - comma separated string
 * @returns an array of strings
 */
function stringToArray(str: string): Array<string> {
  return str.split(",");
}
async function searchEvents(
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
