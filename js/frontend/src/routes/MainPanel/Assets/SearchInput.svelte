<script lang="ts">
	import { GetAppsState } from '$lib/AppState.svelte';
	import Image from '$lib/components/Icon/Image.svelte';
	interface Props {
		value: string;
		isSearching: boolean;
		onInput: (value: string) => void;
		onSave: () => void;
		onFocusChange?: (focus: boolean) => void;
		onMount: (element: HTMLInputElement) => void;
	}
	const ctx = GetAppsState();
	let {
		value = $bindable(),
		isSearching,
		onInput,
		onSave,
		onFocusChange,
		onMount
	}: Props = $props();
	let inputElement: HTMLInputElement | undefined = $state();

	$effect(() => {
		if (inputElement) {
			onMount(inputElement);
		}
	});
</script>

<div class="flex w-full flex-row items-center">
	<label
		class="input {ctx.GlobalCompactMode ? 'input-xs' : 'input-sm'} m-2 focus:border-0 {isSearching
			? 'border-error'
			: ''} join-item flex-1 bg-base-300 ring-0 outline-0 focus:ring-0 focus:outline-0 focus-visible:ring-0"
	>
		<svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
			<g
				stroke-linejoin="round"
				stroke-linecap="round"
				stroke-width="2.5"
				fill="none"
				stroke="currentColor"
			>
				<circle cx="11" cy="11" r="8"></circle>
				<path d="m21 21-4.3-4.3"></path>
			</g>
		</svg>
		<input
			onfocus={() => {
				if (onFocusChange) onFocusChange(true);
			}}
			onblur={() => {
				if (onFocusChange) onFocusChange(false);
			}}
			spellcheck={false}
			oninput={(e) => onInput(e.currentTarget.value)}
			type="text"
			bind:value
			class="image-search"
			bind:this={inputElement}
			required
			placeholder="Search"
		/>
		{#if isSearching}
			<div class="loading text-error"></div>
		{/if}
	</label>
	<button
		onclick={onSave}
		disabled={value === ''}
		class="btn m-0 btn-soft btn-xs btn-warning {ctx.GlobalCompactMode
			? 'h-6! w-6! p-1'
			: 'h-8! w-8! '} mr-2"
	>
		<Image />
	</button>
</div>
<!-- <input
    type="text"
    bind:value
    bind:this={inputElement}
    oninput={(e) => onInput(e.currentTarget.value)}
    placeholder="Search images..."
    class="input input-sm input-error"
  />
  {#if isSearching}
    <span class="searching-indicator">Searching...</span>
  {/if} -->
