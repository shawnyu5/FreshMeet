<script lang="ts">
	import type { PageData } from "./$types";
	import DataTable, { Head, Body, Row, Cell } from "@smui/data-table";
	export let data: PageData;
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
		{#if data.events}
			{#each data.events.nodes as e}
				<Row>
					<Cell>{e.title}</Cell>
					<Cell>{e.description}</Cell>
					<Cell>{e.dateTime}</Cell>
					<Cell>{e.eventUrl}</Cell>
				</Row>
			{/each}
		{:else}
			<p>Error fetching events...</p>
		{/if}
	</Body>
</DataTable>
<!-- <p>{JSON.stringify(data.events)}</p> -->
