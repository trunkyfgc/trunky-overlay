<script lang="ts">
	import Button from './Button.svelte';
	import './Button2.svelte';
	import Button3 from './Button3.svelte';

	let foo = $state(1);

	function onincrease() {
		foo++;
	}

	function ondecrease() {
		foo--;
	}
</script>

<div>
	foo = {foo}
</div>
<div class="flex flex-col">
	<Button on:increase={() => foo++} on:decrease={() => foo--} />
	<br />
	<button-web-component onincrease={() => foo++} ondecrease={() => foo--}></button-web-component>
	<br />
	<Button3 {onincrease} {ondecrease} value={foo} />
</div>
