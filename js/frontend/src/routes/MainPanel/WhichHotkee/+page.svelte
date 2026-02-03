<script lang="ts">
  import { invokePayloadWithCallback } from "$lib";
  import { onMount } from "svelte";
  import type { ReadableHotkee } from "@tsck/lib";
  let hotkee: ReadableHotkee[] = $state([]);
  onMount(() => {
    invokePayloadWithCallback(
      { type: "GetReadableHotkee" },
      function (error, result) {
        hotkee = result;
      },
    );
  });
</script>

{#snippet modifier(key: string, which: boolean)}
  {#if which}
    <kbd class="kbd-xs kbd">{key}</kbd>
  {/if}
{/snippet}

<div class="content w-full h-full overflow-y-scroll">
  <table class="table table-xs table-zebra">
    <thead>
      <tr>
        <th></th>
        <th></th>
        <th></th>
        <th></th>
      </tr>
    </thead>

    <tbody>
      {#each hotkee as kee}
        <tr>
          <td>
            {@render modifier("Win", kee.meta)}
            {@render modifier("Ctrl", kee.ctrl)}
            {@render modifier("Shift", kee.shift)}
            {@render modifier("Alt", kee.alt)}
          </td>
          <td>{kee.key}</td>
          <td>{kee.func.split("::")[0].slice(0, 1)}</td>
          <td>{kee.func.split("::")[1]}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
