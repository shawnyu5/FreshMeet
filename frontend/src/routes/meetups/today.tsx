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
async function getMeetupsToday(after: string): Promise<MeetupEvents> {
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

