<script lang="ts">
	import { invokePayload } from '$lib';
	import { CHATS } from '$lib/temp';

	let toggleDrawer = $state(false);
	function launchWindow(which: string) {
		invokePayload({ type: 'LaunchPlugin', value: which });
	}
	let toggle = $state(true);
</script>

<div class="main size-full overflow-hidden bg-base-300">
	<div class="size-full overflow-hidden">
		<button onclick={() => launchWindow('llm')} class="btn btn-xs">Launch LLM</button>
		<button onclick={() => launchWindow('google')} class="btn btn-xs">Launch Google</button>
		<button
			onclick={() => {
				toggle = !toggle;
				invokePayload({ type: 'SetWindowSize', value: toggle ? [500, 300] : [500, 800] });
			}}
			class="btn btn-xs">Resize</button
		>
		<button
			onclick={() => {
				invokePayload({ type: 'SetWindowPosition', value: [300, 300] });
			}}
			class="btn btn-xs">Set Position</button
		>
		<button
			onclick={() => {
				toggle = !toggle;
				invokePayload({
					type: 'TransformWindow',
					value: {
						label: 'main',
						toSize: toggle ? [500, 300] : [500, 800],
						duration: 500,
						easing: 'EaseOutBack'
					}
				});
			}}
			class="btn btn-xs">Animate Window</button
		>
		<div class="flex size-full flex-1 flex-col gap-2 overflow-hidden overflow-y-scroll bg-base-200">
			{#each CHATS as chat}
				<!-- <span class="chat">{chat}</span> -->
			{/each}
		</div>
	</div>
</div>

<style>
</style>
