<!-- component to meetup display events  -->
<script lang="ts">
	import DataTable, { Head, Body, Row, Cell } from "@smui/data-table";
	import type { AxiosResponse } from "axios";
	import axios from "axios";
	import type { MeetupEvent } from ".";
	import { writable, type Writable } from "svelte/store";
	import { beforeUpdate, onMount } from "svelte";

	/**
	 * the query to search using meetup api
	 */
	export let queries: Array<string>;

	/**
	 * number of events per page
	 */
	export let per_page = 5;

	/**
	 * after cursor
	 */
	export let after: Array<string> = [""];

	let events: MeetupEvent | undefined;
	// onMount(async () => {
	// 	events = await fetchEvents();
	// });

	// beforeUpdate(async () => {
	// 	events = await fetchEvents();
	// });

	async function fetchEvents(): Promise<MeetupEvent | undefined> {
		console.log("fetching events");
		try {
			let results: MeetupEvent | null = null;

			for (const [i, query] of queries.entries()) {
				let response: AxiosResponse<MeetupEvent> = await axios.post(
					"http://localhost:8000/meetup/search",
					{
						query,
						per_page: per_page.toString(),
						after: after[i]
					}
				);
				if (results == null) {
					results = response.data;
				} else {
					results.nodes = results.nodes.concat(response.data.nodes);
				}
				after[i] = results.page_info.endCursor;
			}
			return Promise.resolve(results as MeetupEvent);
		} catch (e: any) {
			console.log(e);
		}
	}
</script>

{#if events}
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
			{#each events.nodes as e}
				<Row>
					<Cell>{e.title}</Cell>
					<Cell>{e.description}</Cell>
					<Cell>{e.dateTime}</Cell>
					<Cell>{e.eventUrl}</Cell>
				</Row>
			{/each}
		</Body>
	</DataTable>
{/if}
