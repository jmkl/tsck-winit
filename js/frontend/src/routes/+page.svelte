<script lang="ts">
	import TitleBar from '$lib/components/TitleBar.svelte';
	import MainPanel from './MainPanel/+page.svelte';
</script>

<div class="flex h-screen w-screen flex-col overflow-hidden rounded-md border border-base-300">
	<TitleBar />
	<MainPanel />
</div>
