export interface MeetupEvents {
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
  /**
   * A human friendly representation of if this event is being attended. Use self.isAttending as fall back
   */
  isAttendingStr: string | null;
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

