<script lang="ts" generics="T extends SmartObjectItem|TextureItem">
  import mia from "$lib/assets/yt-thumb.png";
  import Image from "$lib/components/Icon/Image.svelte";
  import { onMount } from "svelte";
  import gsap from "gsap";
  import Recycle from "../Icon/Recycle.svelte";
  import IconButton from "../IconButton.svelte";
  import Fav from "../Icon/Fav.svelte";
  import { GetAppsState } from "$lib/AppState.svelte";
  import { type SmartObjectItem } from "@tsck/lib";
  import { type TextureItem } from "$lib/texture.type";

  let hightlighter: HTMLDivElement | undefined = $state();
  interface Props<T> {
    files: T[];
    forceEnter: number;
    serverUrl: string;
    selectedIndex: number;
    appendSelected?: (file: T) => void;
    deleteSelected?: (file: T) => void;
    allowFavorite?: (file: T) => void;
    gridItems: HTMLElement[];
  }

  let {
    files,
    serverUrl,
    forceEnter,
    selectedIndex,
    appendSelected,
    deleteSelected,
    allowFavorite,
    gridItems = $bindable(),
  }: Props<T> = $props();
  function setGridItem(node: HTMLElement, index: number) {
    if (gridItems[index] !== node) {
      gridItems[index] = node;
    }
  }
  const ctx = GetAppsState();
  let COLUMNS = $derived(4);
  $inspect(forceEnter);
  function animateSelected() {
    console.log("animated selected");
    if (hightlighter) {
      gsap.to(hightlighter, {
        rotate: 360,
        duration: 0.1,
        transformOrigin: "center center",
        onComplete: (e) => {
          if (hightlighter) gsap.set(hightlighter, { rotate: 0 });
        },
      });
    }
  }
  function handleMouseEnter(index: number) {
    selectedIndex = index;
    animateSelected();
  }
  let gridHover: boolean[] = $state([]);

  function animateSelection() {
    const padding = 0;
    const node = gridItems[selectedIndex];
    if (!node || !hightlighter) return;
    const w = node.getBoundingClientRect();
    gsap.to(hightlighter, {
      duration: 0.1,
      x: node.offsetLeft - padding / 2,
      y: node.offsetTop - padding / 2,
      height: w.height + padding,
      width: w.width + padding,
      transformOrigin: "top left",
    });
  }

  $effect(() => {
    forceEnter;
    animateSelected();
  });
  $effect(() => {
    selectedIndex;
    animateSelection();
  });
</script>

<div
  tabindex="-1"
  class="grid relative scroll-container col-span-1 h-full w-full gap-2 overflow-x-hidden overflow-y- grid-cols-4 auto-rows-min"
>
  {#if selectedIndex >= 0}
    <div
      bind:this={hightlighter}
      class="z-9 absolute rounded-lg pointer-events-none bg-linear-to-t from-error/50 to-transparent border-4 border-error"
    ></div>
  {/if}
  {#each files as file, i}
    <div
      use:setGridItem={i}
      role="button"
      onmouseenter={() => handleMouseEnter(i)}
      tabindex="-1"
      use:setGridItem={i}
      class="group relative overflow-hidden rounded-md outline-0 transition-all duration-100 select-none focus:outline-0 {i ===
      selectedIndex
        ? ''
        : ''}"
    >
      <div
        role="button"
        tabindex="-1"
        onmouseenter={() => (gridHover[i] = true)}
        onmouseleave={() => (gridHover[i] = false)}
        class="btn-group absolute flex h-full w-full items-center justify-center"
      >
        {#if gridHover[i]}
          <div
            class="flex h-full w-full flex-row items-center justify-center gap-2 rounded-md backdrop-blur-sm"
          >
            {#if allowFavorite}
              <IconButton
                class="h-5! w-5! p-1! btn-ghost! btn-error"
                onclick={() => {
                  allowFavorite(file);
                }}
                icon={Fav}
              ></IconButton>
            {/if}
            <IconButton
              class="h-5! w-5! p-1! btn-ghost! btn-error"
              onclick={() => {
                animateSelected();

                if (appendSelected) appendSelected(file);
              }}
              icon={Image}
            ></IconButton>
            {#if deleteSelected}
              <IconButton
                class="h-5! w-5! p-1! btn-ghost! btn-error"
                onclick={() => {
                  deleteSelected(file);
                }}
                icon={Recycle}
              ></IconButton>
            {/if}
          </div>
        {/if}
      </div>
      <img
        onerror={(e) => {
          (e.currentTarget as HTMLImageElement).onerror = null;
          const t = e.target as HTMLImageElement;
          t.src = mia;
        }}
        tabindex="-1"
        class="pointer-events-none aspect-square h-full w-full object-cover select-none"
        src={serverUrl + "/" + file.thumb}
        alt={file.name}
      />
    </div>
  {/each}
</div>
