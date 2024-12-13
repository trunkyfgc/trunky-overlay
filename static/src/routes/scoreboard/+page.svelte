<script lang="ts">
	import { fade } from 'svelte/transition';

	import { scoreboardStore as scoreboard } from '$lib/scoreboard';
</script>

<div class="background">
	{#key $scoreboard.title}
		<div id="description" transition:fade>{$scoreboard.title}</div>
	{/key}
	{#key $scoreboard.player1?.name}
		<div id="p1name" transition:fade>{$scoreboard.player1?.name}</div>
	{/key}
	{#key $scoreboard.player2?.name}
		<div id="p2name" transition:fade>{$scoreboard.player2?.name}</div>
	{/key}
	{#key $scoreboard.player1?.score}
		<div id="p1score" transition:fade>{$scoreboard.player1?.score}</div>
	{/key}
	{#key $scoreboard.player2?.score}
		<div id="p2score" transition:fade>{$scoreboard.player2?.score}</div>
	{/key}
</div>

<style>
	@font-face {
		font-family: 'CityBrawlers';
		font-style: normal;
		font-weight: normal;
		src: url('fonts/CityBrawlersBoldCaps.otf') format('opentype');
	}

	* {
		font-family: 'CityBrawlers';
	}

	.background {
		background-image: url(./assets/scoreboard-dn.png);
		width: 1920px;
		height: 1080px;
		position: relative;
		display: flex;
		flex-direction: row;
	}

	.background > * {
		display: flex;
		align-items: center;
		justify-content: center;
		position: absolute;
		color: white;
	}

	#description {
		width: 448px;
		height: 27px;
		top: 0;
		left: 737px;
		font-size: 39px;
	}

	#p1score {
		font-size: 65px;
		color: white;
		left: 654px;
		width: 67px;
		height: 44px;
		margin-top: -5px;
	}

	#p2score {
		font-size: 65px;
		color: white;
		right: 654px;
		width: 67px;
		height: 44px;
		margin-top: -5px;
	}

	#p1name {
		left: 264px;
		height: 45px;
		width: 390px;
		font-size: 55px;
		margin-top: -5px;
	}

	#p2name {
		right: 264px;
		height: 45px;
		width: 390px;
		font-size: 55px;
		margin-top: -5px;
	}
</style>
