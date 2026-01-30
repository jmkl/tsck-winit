<script lang="ts">
	import TitleBar from '$lib/components/TitleBar.svelte';
	import { onDestroy, onMount } from 'svelte';
	import MainPanel from './MainPanel/+page.svelte';
	import { listen, type UnlistenFn } from '$lib';
	import type { EventPayload, UserEvent } from '$lib/tsck.types';

	let listenFn: UnlistenFn | undefined = $state();
	onMount(() => {
		listenFn = listen<EventPayload, UserEvent>('tsck::event|EVENTPAYLOAD::FRONTEND', (e) => {
			if (e == undefined) return;
			console.log(e.type);
			switch (e.type) {
				case 'ActivateWorkSpace':
					console.log(e.value);
					break;
			}
		});
	});
	onDestroy(() => {
		if (listenFn) listenFn();
	});
</script>

<div
	class="flex h-screen w-screen flex-col overflow-hidden rounded-md border border-base-300 bg-base-300"
>
	<div class="h-auto w-full shrink-0">
		<TitleBar />
	</div>
	<div class="w-full flex-1 overflow-hidden p-0.5">
		<MainPanel />
	</div>
</div>
