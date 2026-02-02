<script lang="ts">
  import TitleNode from "./TitleNode.svelte";

  let {
    title,
    items,
    selectedIndex = $bindable(),
    onSelectionChange = null,
  } = $props();
</script>

<div class="flex w-full flex-row items-center justify-end p-1">
  <TitleNode {title} />
  <div class="flex w-1/2 justify-end">
    <select
      onchange={(e) => {
        const target = e.target as HTMLSelectElement;
        selectedIndex = target.selectedIndex;
        if (onSelectionChange) onSelectionChange(selectedIndex);
      }}
      class="select join-item bg-base-300 select-xs select-error ring-0 outline-0 focus:ring-0 focus:outline-none"
    >
      {#each items as item, index}
        <option selected={selectedIndex == index} value={item}>
          {item}
        </option>
      {/each}
    </select>
  </div>
</div>
