import axios, { AxiosResponse } from "axios";
import { For, createEffect, createSignal, Signal, Setter } from "solid-js";

export default function (props: { query: string; per_page: number }) {
  const { query, per_page } = props;
  const [events, setEvents] = createSignal<MeetupEvent>();
  const afterSignal = createSignal("");
  const [pageNumber, setPageNumber] = createSignal(1);

  // Function to load more data when scrolling to the bottom
  function handleScroll() {
    const scrolledToBottom =
      window.innerHeight + window.scrollY >= document.body.offsetHeight;
    if (scrolledToBottom) {
      console.log("scrolled to botton");
      setPageNumber(pageNumber() + 1);
    }
  }

  createEffect(async () => {
    console.log(pageNumber());
    const events = await fetchEvents(query, per_page, afterSignal);
    setEvents((e) => {
      if (e) {
        e.nodes.concat(events.nodes);
        return e;
      } else {
        return events;
      }
    });
  });

  // Attach and remove event listeners for scrolling
  createEffect(() => {
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  });

  return (
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
  );
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
async function fetchEvents(
  query: string,
  per_page: number,
  afterSignal: Signal<string>
): Promise<MeetupEvent> {
  console.log("fetching events");
  try {
    let results: MeetupEvent | null = null;

    let response: AxiosResponse<MeetupEvent> = await axios.post(
      "http://localhost:8000/meetup/search",
      {
        query,
        per_page: per_page.toString(),
        // after: afterSignal,
      }
    );
    results = response.data;
    afterSignal[1](results.page_info.endCursor);
    return Promise.resolve(results as MeetupEvent);
  } catch (e: any) {
    console.log(e);
  }
}
