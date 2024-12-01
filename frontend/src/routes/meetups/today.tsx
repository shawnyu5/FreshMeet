import axios, { AxiosResponse } from "axios";
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

export default function() {
   // const [pageNumber, setPageNumber] = createSignal(1);
   // const [afterCursor, setAfterCursor] = createSignal("");
   // const [getHasNextPage, setHasNextPage] = createSignal(true);
   const [searchParams, _setSearchParams] = useSearchParams();
   // Create a signal that is derived from searchParams
   const [paramsSignal, setParamsSignal] = createSignal(searchParams);

   // Update the signal whenever searchParams changes
   createEffect(() => {
      // logger.info(
      //   `Updating search param signal: ${JSON.stringify(searchParams)}`,
      // );
      setParamsSignal({ ...searchParams });
   });

   const [eventResource] = createResource(paramsSignal, async (paramsSignal) => {
      log.info("Fetching events for today");
      const startDate = paramsSignal.startDate as string;
      const endDate = paramsSignal.endDate as string;

      // If start or end date is null, it means date selection is in progress. Don't refetch events
      if (startDate == null || endDate == null) {
         return;
      }

      const query = paramsSignal.query as string;
      if (query) {
         log.info(`Search query found: '${query}'. Using search query`);
         const events = await searchMeetups(
            query,
            startDate as string,
            endDate as string,
            100,
         );
         return events;
      } else {
         log.info("No search query found. Getting recommended meetups");
         const events = await getRecommendedMeetups(startDate, endDate);
         return events;
      }
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
                                 <td>{node.node.isAttending.toString()}</td>
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
