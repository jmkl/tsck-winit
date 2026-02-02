<script lang="ts">
	import Add from '$lib/components/Icon/Add.svelte';
	import TitleNode from './TitleNode.svelte';

	let { float, value = $bindable(), title, min, max, step, onchange = null } = $props();
	function onAdd() {
		value = value + step;
		if (value < min) value = min;
	}
	function onSubstract() {
		value = value - step;
		if (value > max) value = max;
	}
	let error = $state(false);
</script>

<div class="flex w-full flex-row items-center justify-between p-1">
	<TitleNode {title} />
	<div class="node">
		<div class="join">
			<button
				aria-label="min"
				tabindex="-1"
				onclick={onSubstract}
				class="btn join-item btn-soft btn-sm btn-error"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="4"
					class="ring-0 outline-0 hover:text-warning focus:outline-0"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d="M5 12h14" />
				</svg>
			</button>
			<input
				onchange={(e) => {
					const target = e.target as HTMLInputElement;
					error = !Number(target.value);
					if (error) {
						value = value;
						return;
					}
					if (target.value < min) value = min;
					if (target.value > max) value = max;

					if (onchange) onchange();
				}}
				tabindex="-1"
				type="text"
				class="
				input {error ? 'input-error' : 'input-ghost'} input-sm join-item max-w-18 text-center
					"
				bind:value
			/>
			<button
				aria-label="plus"
				tabindex="-1"
				onclick={onAdd}
				class="
				btn join-item btn-soft
				btn-sm btn-error
				"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="4"
					class="ring-0 outline-0 hover:text-warning focus:outline-0"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d="M5 12h14" />
					<path d="M12 5v14" />
				</svg>
			</button>
		</div>
	</div>
</div>
