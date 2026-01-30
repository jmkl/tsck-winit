<script lang="ts">
	import Thumbnail from './Thumbnail/+page.svelte';
	import Assets from './Assets/+page.svelte';
	import RawFilter from './RawFilter/+page.svelte';
	import FaceRestore from './FaceRestore/+page.svelte';
	import Textures from './Textures/+page.svelte';
	import YoutubeThumbnail from './YoutubeThumbnail/+page.svelte';
	import { GetAppsState, HERO_PAGE, SetAppsState } from '$lib/AppState.svelte';
	import { onDestroy, onMount } from 'svelte';
	import { invokePayload, listen, type UnlistenFn } from '$lib';
	import type { EventPayload, UserEvent } from '$lib/tsck.types';
	import { bubblePop } from '$lib/animation';
	SetAppsState();

	const ctx = GetAppsState();
	let listenFn: UnlistenFn | undefined = $state();

	let snippet: string = $derived.by(() => {
		const texts = ctx.todoTemplateLines
			.map((t) => t.text)
			.join(' ')
			.split('$');
		if (texts.length > 0 && texts[1] != null) {
			return texts[1].replaceAll(',', '-');
		} else {
			return '';
		}
	});

	let splitSnippet = $derived(
		snippet
			.split(' ')
			.map((t) => t.replace(/[^a-z]/gi, ''))
			.filter((t) => t != '')
	);

	onMount(() => {
		invokePayload<UserEvent>({ type: 'SetWindowOnTop', value: true });
		listenFn = listen<EventPayload, UserEvent>('tsck::event|EVENTPAYLOAD::FRONTEND', (e) => {
			if (e == undefined) return;
			switch (e.type) {
				case 'CyclePages':
					const len = Object.keys(HERO_PAGE).length;
					ctx.globalActivePage = (ctx.globalActivePage + e.value + len) % len;
					break;
			}
		});
	});
	onDestroy(() => {
		if (listenFn) listenFn();
	});
</script>

<svelte:window onfocus={() => ctx.setWindowFocus(true)} onblur={() => ctx.setWindowFocus(false)} />
<div class="h-full w-full bg-base-300">
	{#if ctx.globalActivePage === HERO_PAGE.THUMBNAIL}
		<Thumbnail />
	{:else if ctx.globalActivePage === HERO_PAGE.SMARTOBJECT}
		<Assets />
	{:else if ctx.globalActivePage === HERO_PAGE.TEXTURES}
		<Textures />
	{:else if ctx.globalActivePage === HERO_PAGE.FACERESTORE}
		<FaceRestore />
	{:else if ctx.globalActivePage === HERO_PAGE.RAWFILTER}
		<RawFilter />
	{:else if ctx.globalActivePage === HERO_PAGE.YOUTUBETHUMBNAIL}
		<YoutubeThumbnail />
		<!-- {:else if ctx.globalActivePage === HERO_PAGE.LOG}
		<CommandLog splashScreen={false} onConnected={() => {}} />
	{:else if ctx.globalActivePage === HERO_PAGE.CLASSGEN}
		<ClassGen /> -->
	{/if}
	{#if ctx.showSnippet && [HERO_PAGE.THUMBNAIL, HERO_PAGE.SMARTOBJECT].includes(ctx.globalActivePage) && splitSnippet.length > 0}
		<div
			class="
			pointer-events-none absolute {ctx.globalActivePage == HERO_PAGE.THUMBNAIL
				? 'justify-left bottom-0 w-1/2 p-0 text-[12px]'
				: 'bottom-8 w-full justify-center bg-error/90 py-1 text-[12px]'}  left-0
			z-99999 footer flex
			flex-row flex-wrap gap-0.5 font-avantt font-bold select-none"
		>
			{#each splitSnippet as t, idx}
				<a
					tabindex="-1"
					href="/"
					onclick={(e) => {
						e.preventDefault();
						splitSnippet = splitSnippet.filter((_, i) => i != idx);
					}}
					class="rounded-xs bg-error p-0.5 leading-2 font-bold text-base-300"
				>
					{t}
				</a>
			{/each}
		</div>
	{/if}
	{#if ctx.IsWindowFocus}
		<div
			transition:bubblePop={{}}
			class="pointer-events-none absolute -top-8.75 left-1/2 h-20 w-1/2 -translate-x-1/2 -translate-y-1/2 rounded-2xl bg-error opacity-100"
		></div>
	{/if}
</div>
