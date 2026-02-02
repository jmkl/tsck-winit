<script lang="ts">
  import placeholder from "$lib/assets/yt-thumb.png";
  import gsap from "gsap";
  import { GetAppsState, HERO_PAGE } from "$lib/AppState.svelte";
  import LazyImage from "./LazyImage.svelte";
  import { onMount } from "svelte";
  import type { UserEvent } from "@tsck/lib";
  import { invokePayload, invokePayloadWithCallback } from "$lib";
  const ctx = GetAppsState();
  function getYouTubeID(url: string) {
    try {
      const u = new URL(url);
      if (u.searchParams.has("v")) {
        return u.searchParams.get("v") ?? "";
      }

      // Case 2: Share/live/shorts links -> pathname
      const paths = u.pathname.split("/").filter(Boolean);
      for (const p of paths) {
        if (/^[a-zA-Z0-9_-]{11}$/.test(p)) {
          return p;
        }
      }

      return ""; // not found
    } catch {
      return "";
    }
  }

  let title = $state(
    "JKW MAIN ANCAM?! AKUI DENDAM KE 3 NAMA! MASIH RAHASIA! SIAPA?! - JKW JAS, JKE BOTAK, JKW IJAZAH DAN IJAZAH LINGKAR MERAH",
  );
  let apiThumb: string | undefined = $state();
  let id: string | null = $state(null);

  type YTResult = {
    title: string;
    thumbnail_url: string;
  };
  onMount(() => {
    fetchYoutubeInfo();
  });

  function fetchYoutubeInfo() {
    if (ctx.youtubeThumbnailUrl) id = getYouTubeID(ctx.youtubeThumbnailUrl);
    if (id)
      invokePayloadWithCallback<UserEvent, YTResult>(
        { type: "YoutubeTitle", value: id },
        (error, result) => {
          if (!error && result) {
            title = result.title;
            apiThumb = result.thumbnail_url;
          }
        },
      );
  }
  let timeline = gsap.timeline({ defaults: { duration: 0.1 } });
  function imageClick(e: MouseEvent) {
    e.preventDefault();
    let a = e.target as HTMLAnchorElement;
    timeline.clear();
    timeline
      .to(a, {
        scale: 0.8,
        rotate: 4,
      })
      .to(a, {
        scale: 1,
        rotate: 0,
      });
    if (apiThumb)
      invokePayload<UserEvent>({
        type: "GoogleDownloadImage",
        value: apiThumb,
      });
  }
  function titleClick(e: MouseEvent) {
    e.preventDefault();
    if (title == "") return;

    ctx.todoHelper?.addTodoFromClipboard(title.toUpperCase()).then(() => {
      ctx.todoUpdate();
      ctx.globalActivePage = HERO_PAGE.THUMBNAIL;
    });
  }
</script>

<svelte:document
  onkeydown={async (e) => {
    if (e.key == "v" && e.ctrlKey) {
      const text = await navigator.clipboard.readText();
      ctx.youtubeThumbnailUrl = text;
      fetchYoutubeInfo();
    }
  }}
/>
<div class="flex w-full flex-col items-center gap-2 p-2">
  <input
    type="text"
    bind:value={ctx.youtubeThumbnailUrl}
    oninput={(e) => {
      const el = e.target as HTMLInputElement;
      ctx.youtubeThumbnailUrl = el.value;
      if (ctx.youtubeThumbnailUrl) id = getYouTubeID(ctx.youtubeThumbnailUrl);
    }}
    placeholder="Type url here"
    class="input input-sm join-item w-full input-ghost input-error"
  />
  <div class="flex h-full w-full flex-col items-center gap-2">
    <div class="flex flex-row gap-2">
      <LazyImage index={0} class="w-auto" {id} />
      <LazyImage index={1} class="w-auto" {id} />
      <LazyImage index={2} class="w-auto" {id} />
    </div>
    <a class="self-center" href="/" onclick={imageClick}>
      <img
        alt=""
        class={`w-full cursor-pointer rounded-xl object-contain`}
        src={apiThumb ? apiThumb : placeholder}
      />
    </a>
  </div>
  <a
    href="/"
    onclick={titleClick}
    class="p-2 text-center font-avantt text-xs text-error uppercase"
  >
    {title}
  </a>
</div>

<style>
  .todo-text {
    font-family: "JetBrains Mono";
    font-size: 9px;
    font-weight: 200;
    line-height: 140%;
    text-transform: uppercase;
  }
</style>
