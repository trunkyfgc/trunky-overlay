<script lang="ts">
	import { playerListStore } from '$lib/player-list';

	let playerText: string = $state('');
	playerListStore.subscribe((playerList: string[]) => (playerText = playerList.join('\n')));

	function onLoadPlayer() {
		playerListStore.set([
			...new Set(
				playerText
					.split('\n')
					.map((player) => player.trim())
					.filter((player) => !!player)
			)
		]);
	}
</script>

<label for="player-list">Player names (one per line)</label>
<textarea id="player-list" class="nes-textarea mb-4" rows="8" bind:value={playerText}> </textarea>
<button type="button" class="nes-btn is-primary" onclick={onLoadPlayer}> Load players </button>
