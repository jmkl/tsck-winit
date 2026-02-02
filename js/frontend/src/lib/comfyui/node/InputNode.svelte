<script lang="ts">
  import TitleNode from "./TitleNode.svelte";

  let { value = $bindable(), title } = $props();
</script>

<div class="flex flex-row p-1 w-full items-center justify-between">
  <TitleNode {title} />
  <div class="node w-1/2">
    <input
      tabindex="-1"
      type="text"
      class="input input-error input-sm join-item"
      bind:value
    />
  </div>
</div>
