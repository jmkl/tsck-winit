<script lang="ts">
	import { invokePayload, invokePayloadWithCallback } from '$lib';
	import { CHATS } from '$lib/temp';
	import type { UserEvent, WindowInfoExt } from '$lib/tsck.types';
	import { onMount } from 'svelte';
	let activeWindows: WindowInfoExt[] | undefined = $state();
	let toggleDrawer = $state(false);
	function launchWindow(which: string) {
		invokePayload({ type: 'LaunchPlugin', value: which });
	}
	let toggle = $state(true);
	function getActiveWindow() {
		invokePayloadWithCallback<UserEvent>({ type: 'GetActiveWindows' }, (error, result) => {
			console.log(result);
			if (!error && result) {
				activeWindows = result.data;
			}
		});
	}
	onMount(() => {
		getActiveWindow();
	});
</script>

<div class="main size-full overflow-hidden bg-base-300">
	<div class="size-full overflow-hidden">
		<input
			type="checkbox"
			class="toggle toggle-xs"
			onchange={(e) => {
				let checked = (e.target as HTMLInputElement).checked;
				invokePayload({ type: 'SetWindowOnTop', value: checked });
			}}
		/>
		<button onclick={() => getActiveWindow()} class="btn btn-xs">GetActiveWindow</button>
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
		<div
			class="flex size-full flex-1 flex-col gap-2 overflow-hidden overflow-y-scroll bg-base-200 font-jetbrains text-xs font-extralight"
		>
			<table class="table table-xs">
				<thead>
					<tr>
						<th>Exe</th>
						<th>Class</th>
						<th>Title</th>
						<th>Size</th>
						<th>Pos</th>
					</tr>
				</thead>
				<tbody>
					{#each activeWindows as w}
						<tr>
							<th>{w.exe}</th>
							<th>{w.class}</th>
							<th>{w.title}</th>
							<th>{w.size.width + ':' + w.size.height}</th>
							<th>{w.position.x + ':' + w.position.y}</th>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>

<style>
</style>
