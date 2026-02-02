<script lang="ts">
  import TitleBar from "$lib/components/TitleBar.svelte";
  import { onDestroy, onMount } from "svelte";
  import MainPanel from "./MainPanel/+page.svelte";
  import CommandLog from "./CommandLog/+page.svelte";
  import {
    invokePayload,
    invokePayloadWithCallback,
    listen,
    type UnlistenFn,
  } from "$lib";
  import { GetAppsState, HERO_PAGE, SetAppsState } from "$lib/AppState.svelte";
  import type { EventPayload, UserEvent, WindowSize } from "@tsck/lib";
  import CurrentPage from "$lib/components/ImageGrid/CurrentPage.svelte";

  const WINDOW_SIZE: WindowSize[] = [
    { width: 350, height: 25 },
    { width: 350, height: 560 },
  ];

  SetAppsState();
  const ctx = GetAppsState();
  let compactMode = $state(false);

  let listenFn: UnlistenFn | undefined = $state();
  onMount(() => {
    invokePayloadWithCallback<UserEvent>({ type: "RequestCommand" }, (e, f) => {
      console.log(e, f);
    });
    listenFn = listen<EventPayload, UserEvent>(
      "tsck::event|EVENTPAYLOAD::FRONTEND",
      (e) => {
        if (e == undefined) return;
        switch (e.type) {
          case "ActivateWorkSpace":
            console.log(e.value);
            break;
          case "WindowFocusChange":
            ctx.IsWindowFocus = e.value;
            if (!e.value) {
              //hide
              hideWindow();
              // invokePayload<UserEvent>({
              //   type: "SetWindowLevel",
              //   value: "Normal",
              // });
            } else {
              //show
              compactMode = false;
              animateWindow();
              invokePayload<UserEvent>({
                type: "SetWindowLevel",
                value: "Top",
              });
            }
            break;
        }
      },
    );
  });
  onDestroy(() => {
    if (listenFn) listenFn();
  });
  function animateWindow() {
    invokePayload<UserEvent>({
      type: "SetWindowSize",
      value: compactMode ? WINDOW_SIZE[0] : WINDOW_SIZE[1],
    });
    // invokePayload<UserEvent>({
    //   type: "TransformWindow",
    //   value: {
    //     label: "main",
    //     easing: "EaseInQuad",
    //     toSize: SIZE[compactMode ? 0 : 1],
    //     duration: 200,
    //   },
    // });
  }
  let hideTimeout: NodeJS.Timeout | undefined = $state();
  function hideWindow() {
    if (hideTimeout) clearTimeout(hideTimeout);
    hideTimeout = setTimeout(() => {
      compactMode = true;
      animateWindow();
    }, 150000);
  }
  onDestroy(() => {
    if (hideTimeout) clearTimeout(hideTimeout);
  });
</script>

<!-- <svelte:window
  onfocus={() => updateFocus(true)}
  onblur={() => updateFocus(false)}
/> -->
<div
  class="border-base-300 bg-base-300 flex h-screen w-screen flex-col overflow-hidden rounded-md border"
>
  <div class="flex h-auto w-full shrink-0 flex-row">
    <div class="flex flex-row items-center gap-2 pl-2">
      {#if [HERO_PAGE.SMARTOBJECT, HERO_PAGE.TEXTURES].includes(ctx.globalActivePage)}
        <CurrentPage
          page={ctx.Pages.page}
          totalPages={ctx.Pages.totalPages}
          imageCount={ctx.Pages.imageCount}
        ></CurrentPage>
      {/if}
    </div>
    <div data-tsck-drag-region class="panel flex-1"></div>
    <TitleBar />
  </div>
  {#if !compactMode}
    <div tabindex="-1" class="w-full flex-1 overflow-hidden p-0.5">
      <MainPanel />
    </div>
  {/if}
</div>
