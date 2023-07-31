<!-- component to meetup display events  -->
<script lang="ts">
	import DataTable, { Head, Body, Row, Cell } from "@smui/data-table";
	import type { AxiosResponse } from "axios";
	import axios from "axios";
	import type { MeetupEvent } from ".";

	/**
	 * the query to search using meetup api
	 */
	export let queries: Array<string>;

	// /**
	//  * page number to display
	//  */
	// export let page_number: number;

	/**
	 * number of events per page
	 */
	export let per_page = 10;

	export async function fetchEvents(): Promise<MeetupEvent | undefined> {
		try {
			let results: MeetupEvent | null = null;
			console.log(`results: ${results}`);
			console.log(queries);

			for (const query of queries) {
				let response: AxiosResponse<MeetupEvent> = await axios.post(
					"http://localhost:8000/meetup/search",
					{
						query: query,
						per_page: per_page.toString()
					}
				);
				if (results == null) {
					results = response.data;
				} else {
					results.nodes = results.nodes.concat(response.data.nodes);
				}
			}
			return Promise.resolve(results as MeetupEvent);
		} catch (e: any) {
			console.log(e);
		}
	}
</script>

<DataTable table$aria-label="People list" style="max-width: 100%;">
	<Head>
		<Row>
			<Cell>title</Cell>
			<Cell>description</Cell>
			<Cell>date</Cell>
			<Cell>url</Cell>
		</Row>
	</Head>
	<Body>
		{#await fetchEvents()}
			<p>Loading events...</p>
		{:then events}
			{#each events.nodes as e}
				<Row>
					<Cell>{e.title}</Cell>
					<Cell>{e.description}</Cell>
					<Cell>{e.dateTime}</Cell>
					<Cell>{e.eventUrl}</Cell>
				</Row>
			{/each}
		{:catch}
			<p>Error fetching events...</p>
		{/await}
	</Body>
</DataTable>
