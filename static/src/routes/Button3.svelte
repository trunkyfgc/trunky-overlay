<script lang="ts">
	const { onincrease, ondecrease, value } = $props();
</script>

<button onclick={onincrease}>Inc</button>
<button onclick={ondecrease}>Dec</button>
<span>hello {value}</span>
