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

<div class="flex h-screen w-screen flex-col overflow-hidden rounded-md border border-base-300">
	<TitleBar />
	<MainPanel />
</div>
