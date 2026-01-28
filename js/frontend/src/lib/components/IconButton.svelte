<script lang="ts">
	import { type Component } from 'svelte';
	let { icon, class: className = '', style = '', onclick = undefined } = $props();
</script>

{#snippet Icon(icon: Component)}
	{@const IconComponent = icon}
	<IconComponent />
{/snippet}

<button
	{style}
	tabindex="-1"
	{onclick}
	class="btn btn-square h-6 w-6 p-1 btn-soft btn-xs hover:text-error-content {className}"
>
	{#if icon}
		{@render Icon(icon)}
	{/if}
</button>
