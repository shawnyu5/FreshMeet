import axios, { type AxiosResponse } from "axios";

// event response from backend api
export type Event = {
	page_info: PageInfo;
	nodes: Array<Node>;
};

export type PageInfo = {
	hasNextPage: boolean;
	endCursor: string;
};

export type Node = {
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
};

export async function load(): Promise<{ events: Event } | undefined> {
	try {
		let response: AxiosResponse<Event> = await axios.post("http://localhost:8000/meetup/search", {
			query: "tech events",
			per_page: "10"
		});
		return Promise.resolve({
			events: response.data
		});
	} catch (e: any) {
		console.log(e.data);
	}
}
