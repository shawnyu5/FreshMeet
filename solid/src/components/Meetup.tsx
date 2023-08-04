import axios, { AxiosResponse } from "axios";
import { For, createEffect, createSignal, onMount } from "solid-js";
import Pagination from "./Pagination";

export default function (props: { query: string; per_page: number }) {
  const { query, per_page } = props;
  const [events, setEvents] = createSignal<MeetupEvent>();
  const [pageNumber, setPageNumber] = createSignal(1);

  // clear the after list in local storage on mount
  onMount(async () => {
    console.log("mounted");
    localStorage.setItem("after", "");

    const events = await fetchEvents(query, per_page, lastCursor());
    if (events.page_info.endCursor != "") {
      appendCursors(events.page_info.endCursor);
    }
    setEvents(events);
  });

  // createEffect(async () => {
  //   const events = await fetchEvents(query, per_page, lastCursor());
  //   if (events.page_info.endCursor != "") {
  //     appendCursors(events.page_info.endCursor);
  //   }

  //   setEvents(() => {
  //     return events;
  //   });
  // });

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
                <td>{event.title}</td>
                <td>{event.description}</td>
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
          const events = await fetchEvents(query, per_page, lastCursor());
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

          const events = await fetchEvents(query, per_page, lastCursor());
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

/**
 * converts a comma separated string to an array of strings
 * @param str - comma separated string
 * @returns an array of strings
 */
function stringToArray(str: string): Array<string> {
  return str.split(",");
}
async function fetchEvents(
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
      "http://localhost:8000/meetup/search",
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
