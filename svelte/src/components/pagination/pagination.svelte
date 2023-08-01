<script lang="ts">
	export let page_number: number;

	function scrollToTop() {
		window.scrollTo({
			top: 0,
			behavior: "smooth"
		});
	}

	function incrementPageNumber() {
		page_number = page_number + 1;
		console.log(page_number);
		scrollToTop();
	}

	function decrementPageNumber() {
		if (page_number >= 1) {
			page_number = page_number - 1;
			console.log(page_number);
			scrollToTop();
		}
	}
</script>

<div>
	<button on:click={decrementPageNumber}>Previous</button>
	<button on:click={incrementPageNumber}>Next</button>
</div>
