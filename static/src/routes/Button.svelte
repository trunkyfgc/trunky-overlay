<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
</script>

<button onclick={() => dispatch('increase')}>Inc</button>
<button onclick={() => dispatch('decrease')}>Dec</button>
