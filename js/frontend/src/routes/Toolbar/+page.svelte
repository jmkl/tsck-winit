<script lang="ts">
  import { invokePayload } from "$lib";
  import RangeSlider from "svelte-range-slider-pips";
  import IconButton from "$lib/components/IconButton.svelte";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import { page } from "$app/state";
  import "$lib/layout.css";
  let zoom_value = $state(1.0);
  let currentPage = $derived(page.url.searchParams.get("page"));
  import { onDestroy, onMount } from "svelte";
  import ZoomIn from "$lib/components/Icon/ZoomIn.svelte";
  import ChevronLeft from "$lib/components/Icon/ChevronLeft.svelte";
  import ChevronRight from "$lib/components/Icon/ChevronRight.svelte";
  import Down from "$lib/components/Icon/Down.svelte";
  import Chatgpt from "$lib/components/Icon/Chatgpt.svelte";
  import { fade, fly, slide } from "svelte/transition";
  import Expand from "$lib/components/Icon/Expand.svelte";
  let iconStatus = $state(false);
  let timeout: NodeJS.Timeout | undefined = $state();
  type BrowserType = {
    title: string;
    url: string;
  };
  let isOnTop = $state(false);
  const BrowserPage: BrowserType[] = [
    { title: "GOOGLE", url: "https://google.com/search?udm=2&q=mia+malkova" },
    { title: "GPT", url: "https://chatgpt.com" },
    { title: "CLAUDE", url: "https://claude.ai" },
    { title: "QWEN", url: "https://chat.qwen.ai/" },
    { title: "AISTUDIO", url: "https://aistudio.google.com/" },
  ];
  onMount(() => {
    if (timeout) clearInterval(timeout);
    timeout = setInterval(() => {
      iconStatus = !iconStatus;
    }, 1000);
  });
  onDestroy(() => {
    if (timeout) clearInterval(timeout);
  });
  let url = $state("");

  function zoom() {
    zoom_value = 0.4 + ((zoom_value - 0.4 + 0.1) % (1.0 - 0.4));
  }
  function forward() {
    switch (currentPage) {
      case "google":
        invokePayload({ type: "EvalJs", value: "history.forward()" });
        break;
      case "llm":
        break;
    }
  }
  function backward() {
    switch (currentPage) {
      case "google":
        invokePayload({ type: "EvalJs", value: "history.back()" });
        break;
      case "llm":
        break;
    }
  }
  function updateToolbar() {
    const value = {
      max_width: null,
      height: 36,
      padding: 10,
      absolute: false,
      toolbar_position: "TopLeft",
    };
    invokePayload({ type: "UpdateToolbarPanel", value: value });
  }
  let activeLLM = $state(0);
  let hoverState = $state(false);
  let menuHoverState = $state(false);
  function onTopChange() {
    isOnTop = !isOnTop;
    invokePayload({ type: "SetWindowOnTop", value: isOnTop });
  }
</script>

{#snippet renderButton(llm: BrowserType, index: number)}
  <button
    onclick={() => {
      activeLLM = index;

      invokePayload({ type: "LoadUrl", value: llm.url });
    }}
    class="btn p-1 btn-soft btn-xs btn-error">{llm.title}</button
  >
{/snippet}

<div class="content flex size-full flex-col overflow-hidden">
  <div
    class="flex w-full flex-row items-center justify-between bg-base-300 py-1"
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      onmouseenter={() => (menuHoverState = true)}
      onmouseleave={() => (menuHoverState = false)}
      class="flex flex-row items-center gap-1 px-2"
    >
      {#if !menuHoverState}
        <IconButton class="btn-ghost!" icon={Expand} />
      {:else}
        <IconButton
          class="btn-ghost!"
          onclick={() => {
            zoom();
            invokePayload({ type: "ZoomWebview", value: zoom_value });
            if (zoom_value >= 1.0) zoom_value = 0.5;
          }}
          icon={ZoomIn}
        />
        <span class="badge badge-xs">{zoom_value.toFixed(1)}</span>
        <input
          type="checkbox"
          onchange={onTopChange}
          checked={isOnTop}
          class="toggle toggle-error toggle-xs"
        />
      {/if}
    </div>
    <IconButton onclick={backward} class="btn-ghost!" icon={ChevronLeft} />
    <IconButton onclick={forward} class="btn-ghost!" icon={ChevronRight} />
    <div
      data-tsck-drag-region
      class="flex h-full flex-1 cursor-grab items-center justify-center"
    ></div>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      onmouseenter={() => (hoverState = true)}
      onmouseleave={() => (hoverState = false)}
      class="flex flex-row items-center justify-center"
    >
      {#each BrowserPage as llm, index}
        {#if hoverState}
          {@render renderButton(llm, index)}
        {:else if activeLLM == index}
          {@render renderButton(llm, index)}
        {/if}
      {/each}
    </div>
    <TitleBar maximizeBtn={true} />
  </div>
  <div class="content flex-1 bg-base-100"></div>
</div>
