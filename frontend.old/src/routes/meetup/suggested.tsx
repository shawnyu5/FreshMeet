import axios, { AxiosResponse } from "axios";
import { createResource, For, Suspense } from "solid-js";
import { load } from "~/environment";
import logger from "~/logger";

export default function () {
  const [eventResource] = createResource(getSuggestedEvents);
  return (
    <Suspense fallback={<p>loading...</p>}>
      <div id="suggested-events">
        <table class="table is-striped">
          <thead>
            <tr>
              <th>Name</th>
              <th>date</th>
              <th>Attending</th>
            </tr>
          </thead>
          <tbody>
            <For each={eventResource()?.data.rankedEvents.edges}>
              {(event, _) => (
                <tr>
                  <td>
                    <a target="_blank" href={event.node.eventUrl}>
                      {event.node.title}
                    </a>
                  </td>
                  <td>{event.node.dateTime}</td>
                  <td>{event.node.going}</td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </div>
    </Suspense>
  );
}

/**
 * Set the start and end cursor in local storage
 * @param startCursor start cursor
 * @param endCursor end cursor
 */
// function setStartEndCursor(startCursor: string, endCursor: string) {
//   // Oh wait... is there a point... to this?
//   // Theres no after key in the request query....
//   localStorage.setItem("start_cursor", startCursor);
//   localStorage.setItem("end_cursor", endCursor);
// }

/**
 * Get suggested events from Meetup api
 * @returns list of suggested events
 */
async function getSuggestedEvents(): Promise<SuggestedEvents> {
  let response: AxiosResponse<SuggestedEvents> = await axios.get(
    `${load().api_url}/meetup/suggested`
  );
  logger.info(response.data);
  return response.data;
}

export interface SuggestedEvents {
  data: Data;
}

export interface Data {
  rankedEvents: RankedEvents;
}

export interface RankedEvents {
  count: number;
  pageInfo: PageInfo;
  edges: Edge[];
}

export interface PageInfo {
  hasNextPage: boolean;
  hasPreviousPage: boolean;
  endCursor: string;
  startCursor: string;
}

export interface Edge {
  node: Node;
}

export interface Node {
  id: string;
  title: string;
  dateTime: string;
  endTime: string;
  duration: string;
  going: number;
  maxTickets: number;
  timezone: string;
  images: Image[];
  eventType: string;
  hosts: Host[];
  group: Group;
  isSaved: boolean;
  eventUrl: string;
  isAttending: boolean;
  rsvpState: string;
  venue: Venue;
}

export interface Image {
  id: string;
  baseUrl: string;
}

export interface Host {
  id: string;
  name: string;
  email: any;
}

export interface Group {
  id: string;
  name: string;
  isOrganizer: boolean;
  link: string;
  isPrivate: boolean;
  city: string;
  state: string;
  country: string;
  groupPhoto: GroupPhoto;
}

export interface GroupPhoto {
  id: string;
  baseUrl: string;
}

export interface Venue {
  city: string;
  state: string;
}
