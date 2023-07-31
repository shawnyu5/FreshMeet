<script lang="ts">
	import Meetups from "../../../components/meetup/Meetups.svelte";
	let queries = ["tech events", "coding"];
	let per_page = 10;
</script>

<Meetups {queries} {per_page} />
