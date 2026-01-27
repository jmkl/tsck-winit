<script lang="ts">
	import { invokePayload } from '$lib';
	import TitleBar from '$lib/components/TitleBar.svelte';
	import { onDestroy, onMount } from 'svelte';
</script>

<div
	class="flex h-screen w-screen flex-col overflow-hidden rounded-md border border-base-300 bg-base-100"
>
	<TitleBar />
	<div class="content">
		<button
			class="btn btn-xs"
			onclick={() => {
				invokePayload({ type: 'LaunchPlugin', value: 'hello' });
			}}>Second Panel</button
		>
	</div>
</div>
