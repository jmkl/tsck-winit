<script lang="ts">
	import '$lib/layout.css';

	let { children } = $props();
	let focus = $state(false);
	const padding = 2;
</script>

<svelte:window
	onblur={() => {
		focus = false;
	}}
	onfocus={() => {
		focus = true;
	}}
/>
<div class="absolute inset-0 overflow-hidden">
	<div
		style="width: calc(100% - {padding}px); height: calc(100% - {padding}px);top:{padding /
			2}px;left:{padding / 2}px;"
		class="pointer-events-none absolute z-9999 rounded-md border {focus
			? 'border-error'
			: 'border-transparent'}"
	></div>
	{@render children()}
</div>
