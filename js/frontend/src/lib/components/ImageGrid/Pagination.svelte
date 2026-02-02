<script lang="ts">
	interface Props {
		currentPage: number;
		totalPages: number;
		onPrevPage: () => void;
		onNextPage: () => void;
		onGoToPage: (page: number) => void;
	}

	let { currentPage, totalPages, onPrevPage, onNextPage, onGoToPage }: Props = $props();

	function getVisiblePages() {
		const maxVisible = 10;
		const pages: number[] = [];

		for (let i = 0; i < Math.min(totalPages, maxVisible); i++) {
			const pageNum =
				currentPage < 5 ? i : Math.max(0, Math.min(currentPage - 5 + i, totalPages - 1));

			if (pageNum >= 0 && pageNum < totalPages) {
				if (i === 0 || pageNum !== pages[pages.length - 1]) {
					pages.push(pageNum);
				}
			}
		}

		return pages;
	}

	let visiblePages = $derived(getVisiblePages());
</script>

<div
	class="absolute bottom-0 m-0 flex w-full flex-row justify-center gap-1 bg-base-300/90 px-2 pt-1 pb-2 text-[10px]"
>
	<div class="absolute opacity-0 btn-soft btn-error"></div>
	{#each visiblePages as pageNum, index}
		<button
			tabindex="-1"
			class="btn m-0 px-1 btn-xs btn-error {currentPage == pageNum ? '' : 'btn-soft'}"
			onclick={(e) => {
				if (currentPage === pageNum) return;
				onGoToPage(pageNum);
			}}
		>
			{pageNum + 1}
		</button>
	{/each}
</div>

<style>
</style>
