<script lang="ts">
  import { GetAppsState } from "$lib/AppState.svelte";
  import Add from "$lib/components/Icon/Add.svelte";
  import IconButton from "$lib/components/IconButton.svelte";
  import { CodeJar } from "codejar";
  import hljs from "highlight.js/lib/core";
  import json from "highlight.js/lib/languages/json";
  import "highlight.js/styles/atom-one-dark.css";
  import { onMount } from "svelte";

  hljs.registerLanguage("json", json);
  const ctx = GetAppsState();
  let editorEl: HTMLDivElement | undefined = $state();
  let jar: ReturnType<typeof CodeJar>;
  let rawfilter_value = $state();
  onMount(() => {
    if (editorEl)
      jar = CodeJar(editorEl, (el) => {
        const code = el.textContent ?? "";
        el.innerHTML = hljs.highlight(code, {
          language: "json",
        }).value;
      });
  });
  $effect(() => {
    if (ctx.AppConfig?.rawfilter_template)
      jar.updateCode(
        JSON.stringify(ctx.AppConfig?.rawfilter_template, null, 2),
      );
  });
</script>

{#snippet renderValue(v: any, k: string, depth: number = 0)}
  {#if k === "rawfilter_template"}
    <div bind:this={editorEl} class="editor" contenteditable="true"></div>
  {:else if typeof v === "string" || typeof v === "number" || typeof v === "boolean"}
    <input
      type="text"
      class="input flex-nowrap input-xs input-ghost w-full"
      value={v}
    />
  {:else if Array.isArray(v)}
    <div class="relative flex">
      {#if ["apps", "workspaces"].includes(k)}
        <IconButton
          icon={Add}
          class="z-10 btn-ghost btn-error absolute bottom-0 right-0"
        />
      {/if}
      <div
        class="flex w-full {depth == 1 && k == 'monitors'
          ? 'flex-row border border-error/20'
          : 'flex-col'}"
      >
        {#each v as item}
          {@render renderValue(item, k, depth + 1)}
        {/each}
      </div>
    </div>
  {:else if typeof v === "object" && v !== null}
    <div class="table relative">
      <IconButton
        icon={Add}
        class="z-10 btn-ghost btn-error absolute bottom-0 right-0"
      />
      <table class="w-full">
        <tbody>
          {#each Object.entries(v) as [key, val]}
            <tr>
              <td>
                <input
                  type="text"
                  class="input flex-nowrap input-xs input-ghost w-full"
                  value={key}
                /></td
              >
              <td>
                {@render renderValue(val, k, depth + 1)}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <span class="w-full">{String(v)}</span>
  {/if}
{/snippet}

<div class="w-full h-full overflow-hidden overflow-y-auto">
  {#if ctx.AppConfig}
    <table class="table table-xs table-zebra">
      <thead>
        <tr>
          <th></th>
        </tr>
      </thead>
      <tbody>
        {#each Object.entries(ctx.AppConfig) as [k, v]}
          <tr class="w-full p-0">
            <td class="w-full">
              <div class="flex flex-col">
                <div class="bold bg-error/20 uppercase px-1">
                  {k.replaceAll("_", " ")}
                </div>
                <div class="flex-1 w-full">
                  {@render renderValue(v, k)}
                </div>
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
