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
