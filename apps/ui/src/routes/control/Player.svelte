<script lang="ts">
	let { players, name, score, title, onscorechange, onnamechange } = $props();

	$effect(() => onnamechange(name));
	$effect(() => onscorechange(score));
</script>

<div>
	<h2 class="text-xl font-semibold mb-4 text-gray-900">{title}</h2>
	<div class="mb-4">
		<label for="player-name">Name</label>
		<div class="nes-select">
			<select required bind:value={name}>
				<option disabled selected hidden>Select...</option>
				{#each players as player}
					<option value={player}>{player}</option>
				{/each}
			</select>
		</div>
	</div>
	<div class="nes-field mb-4">
		<label for="player-score">Score</label>
		<div class="flex items-center">
			<button type="button" class="nes-btn is-error" onclick={() => score--}>-</button>
			<input type="text" id="player1-score" class="nes-input" bind:value={score} />
			<button type="button" class="nes-btn is-success" onclick={() => score++}>+</button>
		</div>
	</div>
</div>
