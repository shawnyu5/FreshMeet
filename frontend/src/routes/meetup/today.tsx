import axios, { AxiosResponse } from "axios";
import { createResource, createSignal, For, Suspense } from "solid-js";
import { Desciption, scrollToTop } from "~/components/Meetup";
import Pagination from "~/components/Pagination";
import { load } from "~/environment";
import logger from "~/logger";

export default function () {
  // onMount(async () => {
  //   logger.info("OnMount clear cursor in local storage");
  //   // clear the after list in local storage on mount
  //   setCursors([]);
  // });

  // onCleanup(() => {
  //   logger.info("OnCleanup clear cursor in local storage");
  //   // setCursors([]);
  // });

  const [pageNumber, setPageNumber] = createSignal(1);
  const [afterCursor, setAfterCursor] = createSignal("");
  const [getHasNextPage, setHasNextPage] = createSignal(true);

  const [eventResource, { mutate: mutateEvents, refetch: refetchEvents }] =
    createResource(pageNumber, async () => {
      const events = await getMeetupsToday(afterCursor());
      if (events.data.rankedEvents.pageInfo.hasNextPage) {
        setHasNextPage(true);
        setAfterCursor(events.data.rankedEvents.pageInfo.endCursor);
      } else {
        setHasNextPage(false);
      }

      return events;
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
            {/*
            <InfiniteScroll
              each={eventResource()?.data.rankedEvents.edges}
              hasMore={true}
              next={scrollNext}
              loadingMessage={<div>Loading...</div>}
            >
            {(edge, _index) => (
            */}
            <For each={eventResource()?.data.rankedEvents.edges}>
              {(node, _idx) => (
                <tr>
                  <td>
                    <a target="_blank" href={node.node.eventUrl}>
                      {node.node.title}
                    </a>
                  </td>
                  <td>{node.node.dateTime}</td>
                  <td>{node.node.going}</td>
                  <Desciption description={node.node.description} />
                </tr>
              )}
            </For>
            {/*  )}
                   </InfiniteScroll>*/}
          </tbody>
        </table>
        <Pagination
          nextPageCallback={async () => {
            scrollToTop();
            setPageNumber((e) => e + 1);
          }}
          previousPageCallback={async () => {}}
          // TODO: fix this
          disableNextBtn={!getHasNextPage()}
          disablePrevBtn={true}
        />
      </div>
    </Suspense>
  );
}

/**
 * Fetch list of meetups today
 * @param after - after cursor
 */
async function getMeetupsToday(after: string) {
  logger.info(`after cursor in request: ${after}`);
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
    `cursor in incoming request: ${response.data.data.rankedEvents.pageInfo.endCursor}`
  );
  return response.data;
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
