<script lang="ts">
  import { onMount } from "svelte";
  import { CodeJar } from "codejar";
  import hljs from "highlight.js/lib/core";
  import json from "highlight.js/lib/languages/json";
  import "highlight.js/styles/atom-one-dark.css";
  import { invokePayloadWithCallback } from "$lib";
  import type { AppConfig, UserEvent } from "@tsck/lib";
  import { GetAppsState } from "$lib/AppState.svelte";
  import IconButton from "$lib/components/IconButton.svelte";
  import Save from "$lib/components/Icon/Save.svelte";
  import Recycle from "$lib/components/Icon/Recycle.svelte";
  import Play from "$lib/components/Icon/Play.svelte";
  import { bubblePop } from "$lib/animation";
  hljs.registerLanguage("json", json);
  const ctx = GetAppsState();
  let editorEl: HTMLDivElement;
  let jar: ReturnType<typeof CodeJar>;
  let config: AppConfig | undefined;

  function format(obj: AppConfig) {
    return JSON.stringify(obj, null, 2).replace(
      /\[\s*\n\s*([0-9.\-]+)\s*,\s*\n\s*([0-9.\-]+)\s*\n\s*\]/gm,
      "[ $1, $2 ]",
    );
  }
  onMount(() => {
    jar = CodeJar(editorEl, (el) => {
      const code = el.textContent ?? "";
      el.innerHTML = hljs.highlight(code, {
        language: "json",
      }).value;
    });
  });
  $effect(() => {
    if (jar && ctx.AppConfig) jar.updateCode(format(ctx.AppConfig));
  });
  let error = $state({
    show: false,
    is_error: true,
    message: ``,
  });
  function showToast(is_error: boolean, message: string) {
    error = { show: true, is_error, message };
    setTimeout(() => {
      error = { show: false, is_error, message };
    }, 2000);
  }
</script>

<div class="h-full w-full overflow-y-auto">
  <div class="save_panel absolute top-6 right-6 flex flex-col items-end gap-1">
    <IconButton
      onclick={() => {
        const result = jar.toString();
        try {
          let appconfig: AppConfig = JSON.parse(result);
          showToast(false, "save successfully");
        } catch (e) {
          showToast(true, e as string);
          console.log(e);
        }
      }}
      class="btn-error"
      icon={Save}
    />
    {#if error.show}
      <span
        transition:bubblePop={{}}
        class={`pointer-events-none rounded-md p-1 px-2 text-end font-mono text-[10px] italic ${
          error.is_error
            ? "bg-error text-error-content"
            : "bg-success text-success-content"
        }`}>{error.message}</span
      >
    {/if}
  </div>
  <div bind:this={editorEl} class="editor" contenteditable="true"></div>
</div>

<style>
  .editor {
    font-family: monospace;
    white-space: pre;
    outline: none;
    min-height: 100%;
  }
</style>
