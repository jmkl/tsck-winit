<script lang="ts">
	import '$lib/layout.css';

	let { children } = $props();
</script>

<svelte:head></svelte:head>
<div class="absolute inset-0 overflow-hidden">
	{@render children()}
</div>
