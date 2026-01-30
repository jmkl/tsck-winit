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
	{@render children()}
</div>
