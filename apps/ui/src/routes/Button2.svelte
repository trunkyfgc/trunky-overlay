<svelte:options customElement="button-web-component" />

<script lang="ts">
	const dispatch = (eventName: string) => {
		$host().dispatchEvent(new CustomEvent(eventName));
	};
</script>

<button onclick={() => dispatch('increase')}>Inc</button>
<button onclick={() => dispatch('decrease')}>Dec</button>
