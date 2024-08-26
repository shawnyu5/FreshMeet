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

/**
 * Render the markdown in the description
 *
 * @returns a `td` element containing the description
 */

export default function () {
  const [pageNumber, setPageNumber] = createSignal(1);
  const [afterCursor, setAfterCursor] = createSignal("");
  const [getHasNextPage, setHasNextPage] = createSignal(true);

  const [eventResource] = createResource(async () => {
    const events = await getMeetupsToday(afterCursor());
    setHasNextPage(events.data.result.pageInfo.hasNextPage);
    setAfterCursor(events.data.result.pageInfo.endCursor);
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
      <ErrorBoundary fallback={(err) => err}>
        <div id="meetup-today">
          <Show when={eventResource()?.data.result.edges.length != 0} fallback={<p>No Meetups for selected date... 🥲</p>}>
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
                {/*
            <InfiniteScroll
              each={eventResource()?.data.rankedEvents.edges}
              hasMore={true}
              next={scrollNext}
              loadingMessage={<div>Loading...</div>}
            >
            {(edge, _index) => (
            */}
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
                {/*  )}
                   </InfiniteScroll>*/}
              </tbody>
              {
                // <Pagination
                //   nextPageCallback={async () => {
                //     scrollToTop();
                //     setPageNumber((e) => e + 1);
                //   }}
                //   disableNextBtn={!getHasNextPage()}
                // />
              }
            </table>
          </Show>
        </div>
      </ErrorBoundary>
    </Suspense>
  );
}

/**
 * Fetch list of meetups today
 * @param after - after cursor
 */
async function getMeetupsToday(after: string): Promise<MeetupsToday> {
  try {
    let response: AxiosResponse<MeetupsToday> = await axios.get(
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

export interface MeetupsToday {
  data: Data;
}

export interface Data {
  result: Result;
}

export interface Result {
  pageInfo: PageInfo;
  totalCount: number;
  edges: Edge[];
  __typename: string;
}

export interface PageInfo {
  hasNextPage: boolean;
  endCursor: string;
  __typename: string;
}

export interface Edge {
  node: Node;
  metadata: Metadata;
  __typename: string;
}

export interface Node {
  dateTime: string;
  description: string;
  eventType: string;
  eventUrl: string;
  featuredEventPhoto?: FeaturedEventPhoto;
  feeSettings?: FeeSettings;
  id: string;
  isAttending: boolean;
  isOnline: boolean;
  isSaved: boolean;
  covidPrecautions: CovidPrecautions;
  group: Group;
  maxTickets: number;
  rsvps: Rsvps;
  title: string;
  venue?: Venue;
  socialLabels: any[];
  __typename: string;
  rsvpState: string;
  series?: Series;
}

export interface FeaturedEventPhoto {
  baseUrl: string;
  highResUrl: string;
  id: string;
  __typename: string;
}

export interface FeeSettings {
  accepts: string;
  currency: string;
  __typename: string;
}

export interface CovidPrecautions {
  venueType?: string;
  __typename: string;
}

export interface Group {
  id: string;
  isNewGroup: boolean;
  isPrivate: boolean;
  membershipMetadata: any;
  keyGroupPhoto?: KeyGroupPhoto;
  name: string;
  timezone: string;
  urlname: string;
  __typename: string;
}

export interface KeyGroupPhoto {
  baseUrl: string;
  highResUrl: string;
  id: string;
  __typename: string;
}

export interface Rsvps {
  totalCount: number;
  __typename: string;
}

export interface Venue {
  id: string;
  name: string;
  lat: number;
  lon: number;
  city: string;
  state: string;
  country: string;
  __typename: string;
}

export interface Series {
  events: Events;
  __typename: string;
}

export interface Events {
  edges: Edge2[];
  __typename: string;
}

export interface Edge2 {
  node: Node2;
  __typename: string;
}

export interface Node2 {
  id: string;
  dateTime: string;
  isAttending: boolean;
  group: Group2;
  __typename: string;
}

export interface Group2 {
  urlname: string;
  __typename: string;
}

export interface Metadata {
  recId: string;
  recSource: string;
  __typename: string;
}
