<script lang="ts">
	import type { PageProps } from '$lib/AppState.svelte';

	let { page, totalPages, imageCount }: PageProps = $props();
</script>

<div class="flex w-full flex-row justify-between py-1">
	<div class="flex flex-row gap-1 border font-unisans text-[9px]">
		<span class="opacity-50">page:</span>
		<span class="join-item">
			{page + 1} of {totalPages}
		</span>
	</div>
</div>

<style>
	.border {
		border-radius: 5px;
		border: solid 1px var(--color-base-100);
	}
	.border > * {
		padding: 1px 4px;
	}
	.border > *:nth-child(1) {
		background-color: var(--color-base-100);
		color: #ddd;
	}
</style>
