<script lang="ts">
	import { scoreboardStore } from '$lib/scoreboard';
	import { playerListStore } from '$lib/player-list';

	import Player from './Player.svelte';
	import PlayerList from './PlayerList.svelte';

	let dialog: HTMLDialogElement;

	function openConfirmDialog() {
		dialog.showModal();
	}

	function reset() {
		//scoreboardStore.reset();
		//playerListStore.reset();
		dialog.close();
	}
</script>

<div class="m-4">
	<div class="nes-container with-title mb-4">
		<h1 class="title">Scoreboard Control Center</h1>
		<div class="nes-field mb-4">
			<label for="title">Tournament title</label>
			<input type="text" id="title" class="nes-input" bind:value={$scoreboardStore.title} />
		</div>
		<div class="mb-4">
			<PlayerList />
		</div>
		<div class="grid grid-cols-1 md:grid-cols-2 mb-4 gap-6">
			<Player
				players={$playerListStore}
				name={$scoreboardStore.player1.name}
				score={$scoreboardStore.player1.score}
				title="Player 1"
				onscorechange={(value: number) => ($scoreboardStore.player1.score = value)}
				onnamechange={(value: string) => ($scoreboardStore.player1.name = value)}
			/>
			<Player
				players={$playerListStore}
				name={$scoreboardStore.player2.name}
				score={$scoreboardStore.player2.score}
				title="Player 2"
				onscorechange={(value: number) => ($scoreboardStore.player2.score = value)}
				onnamechange={(value: string) => ($scoreboardStore.player2.name = value)}
			/>
		</div>
		<div class="text-right">
			<button type="button" class="nes-btn" onclick={openConfirmDialog}> Reset </button>
		</div>
		<dialog class="nes-dialog !m-auto" bind:this={dialog}>
			<div class="flex flex-col">
				<h1 class="title mb-4">Confirmation</h1>
				<p class="mb-6">Do you want to reset everything ?</p>
				<menu class="dialog-menu text-right">
					<button class="nes-btn" onclick={() => dialog.close()}>Cancel</button>
					<button class="nes-btn is-primary" onclick={reset}>Confirm</button>
				</menu>
			</div>
		</dialog>
	</div>
</div>
