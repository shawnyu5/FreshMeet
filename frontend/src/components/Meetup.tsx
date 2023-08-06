import axios, { AxiosResponse } from "axios";
import { marked } from "marked";
import { load } from "~/environment";
import { For, createSignal, onMount } from "solid-js";
import Pagination from "./Pagination";

export default function (props: { query: Array<string>; per_page: number }) {
  const { query, per_page } = props;
  const [events, setEvents] = createSignal<MeetupEvent>();
  const [pageNumber, setPageNumber] = createSignal(1);

  onMount(async () => {
    console.log("mounted");
    // clear the after list in local storage on mount
    setCursors([]);

    // load initial events
    const events = await searchAllQueries(query, per_page, lastCursor());
    if (events.page_info.endCursor != "") {
      appendCursors(events.page_info.endCursor);
    }
    setEvents(events);
  });

  return (
    <div id="meetups">
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>description</th>
            <th>date</th>
          </tr>
        </thead>
        <tbody>
          <For each={events()?.nodes}>
            {(event, _) => (
              <tr>
                <td>
                  <a target="_blank" href={event.eventUrl}>
                    {event.title}
                  </a>
                </td>
                <Desciption description={event.description} />
                <td>{event.dateTime}</td>
              </tr>
            )}
          </For>
        </tbody>
      </table>

      <Pagination
        nextPageCallback={async () => {
          scrollToTop();
          setPageNumber((e) => e + 1);
          const events = await searchAllQueries(query, per_page, lastCursor());
          // if there are more results after this one, keep track of the end cursor
          if (events.page_info.endCursor != "") {
            appendCursors(events.page_info.endCursor);
          }
          setEvents(events);
        }}
        previousPageCallback={async () => {
          scrollToTop();
          setPageNumber((e) => e - 1);
          // remove the last 2 cursor from the cursor list
          let cursorArr = getCursors();
          cursorArr.pop();
          cursorArr.pop();

          setCursors(cursorArr);

          const events = await searchAllQueries(query, per_page, lastCursor());
          setEvents(events);
        }}
      />
    </div>
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

/** render the markdown in the description
 *
 * @returns a `td` element containing the description
 */
function Desciption(props: { description: string }) {
  let rendered = marked.parse(props.description);
  // __AUTO_GENERATED_PRINT_VAR_START__
  console.log("formatDescription rendered: %s", rendered); // __AUTO_GENERATED_PRINT_VAR_END__
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
    let searchResult = await searchEvents(query, per_page, after);
    // __AUTO_GENERATED_PRINT_VAR_START__
    console.log(
      "searchAllQueries#for_in searchResult: %s",
      JSON.stringify(searchResult)
    ); // __AUTO_GENERATED_PRINT_VAR_END__

    events.page_info = searchResult.page_info;
    events.nodes = events.nodes.concat(searchResult.nodes);
  }
  // __AUTO_GENERATED_PRINT_VAR_START__
  console.log("searchAllQueries events: %s", JSON.stringify(events)); // __AUTO_GENERATED_PRINT_VAR_END__
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
  console.log("fetching events");
  // __AUTO_GENERATED_PRINT_VAR_START__
  console.log("fetchEvents after: %s", after); // __AUTO_GENERATED_PRINT_VAR_END__
  try {
    let results: MeetupEvent | null = null;

    let response: AxiosResponse<MeetupEvent> = await axios.post(
      `${load().api_url}/meetup/search`,
      // "http://localhost:8000/meetup/search",
      {
        query,
        per_page: per_page.toString(),
        after: after,
      }
    );
    results = response.data;
    return Promise.resolve(results as MeetupEvent);
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
