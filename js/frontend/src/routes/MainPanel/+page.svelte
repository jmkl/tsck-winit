<script lang="ts">
  import Thumbnail from "./Thumbnail/+page.svelte";
  import Assets from "./Assets/+page.svelte";
  import FaceRestore from "./FaceRestore/+page.svelte";
  import tits from "$lib/assets/flou.webm";

  import RawFilter from "./RawFilter/+page.svelte";
  import WhichHotkee from "./WhichHotkee/+page.svelte";
  import CommandLog from "../CommandLog/+page.svelte";
  import Textures from "./Textures/+page.svelte";
  import YoutubeThumbnail from "./YoutubeThumbnail/+page.svelte";
  import SettingUI from "./SettingUI/+page.svelte";
  import {
    GetAppsState,
    HERO_PAGE,
    SHARED_HERO_PAGE,
  } from "$lib/AppState.svelte";
  import { onDestroy, onMount } from "svelte";
  import { invokePayload, listen, type UnlistenFn } from "$lib";
  import type { UserEvent } from "@tsck/lib";
  import { bubblePop, slideMe } from "$lib/animation";
  const ctx = GetAppsState();
  let snippet: string = $derived.by(() => {
    const texts = ctx.TodoTemplateLines.map((t) => t.text)
      .join(" ")
      .split("$");
    if (texts.length > 0 && texts[1] != null) {
      return texts[1].replaceAll(",", "-");
    } else {
      return "";
    }
  });

  let splitSnippet = $derived(
    snippet
      .split(" ")
      .map((t) => t.replace(/[^a-z]/gi, ""))
      .filter((t) => t != ""),
  );

  onMount(() => {
    invokePayload<UserEvent>({ type: "SetWindowLevel", value: "Top" });
  });
  onDestroy(() => {});
</script>

{#if ctx.LoadingPanel}
  <div
    data-tsck-drag-region
    class="absolute loading_panel w-full h-full z-9999999 bg-base-300"
  >
    <video
      transition:slideMe={{ y: 300 }}
      class="pointer-events-none absolute bottom-0 left-2 h-[60%]"
      muted
      autoplay
      loop
    >
      <source src={tits} />
    </video>
  </div>
{/if}
<div tabindex="-1" class="bg-base-300 h-full w-full focus:outline-0">
  {#if ctx.globalActivePage === HERO_PAGE.THUMBNAIL}
    <Thumbnail />
  {:else if ctx.globalActivePage === HERO_PAGE.SMARTOBJECT}
    <Assets />
  {:else if ctx.globalActivePage === HERO_PAGE.TEXTURES}
    <Textures />
  {:else if ctx.globalActivePage === HERO_PAGE.FACERESTORE}
    <FaceRestore />
  {:else if ctx.globalActivePage === HERO_PAGE.RAWFILTER}
    <RawFilter />
  {:else if ctx.globalActivePage === HERO_PAGE.YOUTUBETHUMBNAIL}
    <YoutubeThumbnail />
  {:else if ctx.globalActivePage === HERO_PAGE.SETTINGUI}
    <SettingUI />
  {:else if ctx.globalActivePage === HERO_PAGE.COMMANDLOG}
    <CommandLog splashScreen={false} onConnected={() => {}} />
  {:else if ctx.globalActivePage === HERO_PAGE.HOTKEE}
    <WhichHotkee />
  {/if}
  {#if ctx.showSnippet && [HERO_PAGE.THUMBNAIL, HERO_PAGE.SMARTOBJECT].includes(ctx.globalActivePage) && splitSnippet.length > 0}
    <div
      tabindex="-1"
      class="
			pointer-events-none absolute {ctx.globalActivePage == HERO_PAGE.THUMBNAIL
        ? 'justify-left bottom-0 w-1/2 p-0 text-[12px]'
        : 'bg-error/90 bottom-8 w-full justify-center py-1 text-[12px]'}  footer
			font-avantt left-0 z-99999
			flex flex-row flex-wrap gap-0.5 font-bold select-none"
    >
      {#each splitSnippet as t, idx}
        <a
          tabindex="-1"
          href="/"
          onclick={(e) => {
            e.preventDefault();
            splitSnippet = splitSnippet.filter((_, i) => i != idx);
          }}
          class="bg-error text-base-300 rounded-xs p-0.5 leading-2 font-bold"
        >
          {t}
        </a>
      {/each}
    </div>
  {/if}
  {#if ctx.IsWindowFocus}
    <div
      transition:bubblePop={{}}
      class="bg-error pointer-events-none absolute -top-8.75 left-1/2 h-20 w-1/2 -translate-x-1/2 -translate-y-1/2 rounded-2xl opacity-100"
    ></div>
  {/if}
</div>
