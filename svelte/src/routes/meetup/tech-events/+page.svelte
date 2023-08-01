<script lang="ts">
	import Pagination from "../../../components/pagination/pagination.svelte";
	import Meetups from "../../../components/meetup/Meetups.svelte";
	import { writable } from "svelte/store";

	let queries = ["tech", "coding", "computer science"];
	let per_page = 10;
	let page_number = 1;
	let after = [""];

	$: {
		console.log("reactive statement ran");
		console.log(`page number: ${page_number}`);
		console.log(`after: ${JSON.stringify(after)}`);
	}
</script>

<!-- create pagination buttons, update page_number on page change -->
<Pagination bind:page_number />
<!-- renders a table -->
<Meetups {queries} {per_page} {after} />
