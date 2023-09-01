import axios, { AxiosResponse } from "axios";
import {
  createResource,
  createSignal,
  For,
  onCleanup,
  onMount,
  Suspense,
} from "solid-js";
import {
  appendCursors,
  Desciption,
  getCursors,
  lastCursor,
  scrollToTop,
  setCursors,
} from "~/components/Meetup";
import Pagination from "~/components/Pagination";
import { load } from "~/environment";
import logger from "~/logger";

export default function () {
  // onMount(async () => {
  //   logger.info("OnMount clear cursor in local storage");
  //   // clear the after list in local storage on mount
  //   setCursors([]);
  // });

  onCleanup(() => {
    logger.info("OnCleanup clear cursor in local storage");
    setCursors([]);
  });

  const [pageNumber, setPageNumber] = createSignal(1);
  const [eventResource] = createResource(pageNumber, async () => {
    const events = await getMeetupsToday(lastCursor());
    if (events.data.rankedEvents.pageInfo.endCursor != "") {
      logger.info("Appending cursor to local storage");
      // __AUTO_GENERATED_PRINT_VAR_START__
      console.log(
        "(anon)#if events: %s",
        events.data.rankedEvents.pageInfo.endCursor
      ); // __AUTO_GENERATED_PRINT_VAR_END__

      appendCursors(events.data.rankedEvents.pageInfo.endCursor);
      setHasNextPage(events.data.rankedEvents.pageInfo.hasNextPage);
    }

    return events;
  });

  return (
    <Suspense fallback={<p>loading....</p>}>
      <div id="meetup-today">
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
            <For each={eventResource()?.data.rankedEvents.edges}>
              {(event, _idx) => (
                <tr>
                  <td>
                    <a target="_blank" href={event.node.eventUrl}>
                      {event.node.title}
                    </a>
                  </td>
                  <td>{event.node.dateTime}</td>
                  <td>{event.node.going}</td>
                  <Desciption description={event.node.description} />
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </div>
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
        disableNextBtn={!getHasNextPage()}
        disablePrevBtn={pageNumber() == 1}
      />
    </Suspense>
  );
}

/**
 * Fetch list of meetups today
 * @param after - after cursor
 */
async function getMeetupsToday(after: string | null) {
  logger.info(`after cursor: ${after}`);
  let response: AxiosResponse<MeetupsToday> = await axios.get(
    `${load().api_url}/meetup/today`,
    {
      params: {
        after: after,
      },
    }
  );
  logger.info(response.data);
  logger.info(
    `request cursor: ${response.data.data.rankedEvents.pageInfo.endCursor}`
  );
  return response.data;
}

/**
 * Sets `hasNextPage` in local storage
 * @param hasNextPage the value to store in local storage
 */
function setHasNextPage(hasNextPage: boolean) {
  localStorage.setItem("hasNextPage", String(hasNextPage));
}

/**
 * Get the `hasNextPage` from local storage
 */
function getHasNextPage(): boolean {
  return Boolean(localStorage.getItem("hasNextPage"));
}

export interface MeetupsToday {
  data: Data;
}

export interface Data {
  rankedEvents: RankedEvents;
}

export interface RankedEvents {
  pageInfo: PageInfo;
  count: number;
  edges: Edge[];
}

export interface PageInfo {
  hasNextPage: boolean;
  endCursor: string;
}

export interface Edge {
  node: Node;
  recommendationId: string;
  recommendationSource: string;
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
  images: Image[];
  venue: Venue;
  onlineVenue?: OnlineVenue;
  isSaved: boolean;
  eventUrl: string;
  group: Group;
  going: number;
  maxTickets: number;
  tickets: Tickets;
  isAttending: boolean;
  rsvpState: string;
  isNewGroup: boolean;
}

export interface Image {
  id: string;
  baseUrl: string;
  source: string;
}

export interface Venue {
  id: string;
  address: string;
  neighborhood: any;
  city: string;
  state: string;
  country: string;
  lat: number;
  lng: number;
  zoom: number;
  name: string;
  radius: number;
}

export interface OnlineVenue {
  type: string;
  url: any;
}

export interface Group {
  id: string;
  slug: string;
  isPrivate: boolean;
  isOrganizer: boolean;
  isNewGroup: boolean;
  name: string;
  urlname: string;
  timezone: string;
  link: string;
  city: string;
  state: string;
  country: string;
  groupPhoto: GroupPhoto;
}

export interface GroupPhoto {
  id: string;
  baseUrl: string;
  source?: string;
}

export interface Tickets {
  count: number;
  edges: Edge2[];
}

export interface Edge2 {
  node: Node2;
}

export interface Node2 {
  id: string;
}
