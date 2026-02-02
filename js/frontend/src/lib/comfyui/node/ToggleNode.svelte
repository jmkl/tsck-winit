<script lang="ts">
	import TitleNode from './TitleNode.svelte';
	let { checked = $bindable(), title, onchange = null } = $props();
</script>

<div class="flex w-full flex-row items-center justify-end p-1">
	<TitleNode {title} />
	<div class="flex w-1/2 justify-end">
		<input type="checkbox" {onchange} bind:checked class="toggle toggle-error toggle-sm" />
	</div>
</div>
