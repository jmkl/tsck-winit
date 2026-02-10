<script lang="ts">
  import {
    invokePayload,
    invokePayloadWithCallback,
    listen,
    type UnlistenFn,
  } from "$lib";
  import Browser from "$lib/components/Icon/Browser.svelte";
  import File from "$lib/components/Icon/File.svelte";
  import Photoshop from "$lib/components/Icon/Photoshop.svelte";
  import Question from "$lib/components/Icon/Question.svelte";
  import Tsck from "$lib/components/Icon/Tsck.svelte";
  import Whatsapp from "$lib/components/Icon/Whatsapp.svelte";
  import Zed from "$lib/components/Icon/Zed.svelte";
  import IconButton from "$lib/components/IconButton.svelte";
  import "$lib/layout.css";
  import type { EventPayload, UserEvent, WorkspacePayload } from "@tsck/lib";
  import { onDestroy, onMount } from "svelte";
  let activeWorkSpace = $state(0);
  let activeApp = $state(3346320);
  let workspacePayload: WorkspacePayload | undefined = $state();
  let activeTitle = $derived.by(() => {
    if (workspacePayload) {
      let entry = workspacePayload.entries.find((p) => p.hwnd == activeApp);
      if (entry) {
        return entry.title;
      }
    } else {
      return "";
    }
  });
  let activeAppName = $derived.by(() => {
    if (workspacePayload) {
      let entry = workspacePayload.entries.find((p) => p.hwnd == activeApp);
      if (entry) {
        return entry.app;
      }
    } else {
      return "";
    }
  });
  let listenFn: UnlistenFn | undefined = $state();
  onMount(() => {
    listenFn = listen<EventPayload, UserEvent>(
      "tsck::event|EVENTPAYLOAD::FRONTEND",
      (e) => {
        if (e == undefined) return;
        switch (e.type) {
          case "WorkspaceAppFocusChange":
            activeApp = e.value;
            break;
          case "WorkspaceSendPayload":
            workspacePayload = e.value;
            console.log(workspacePayload);
            activeWorkSpace = Number(workspacePayload.active);
            break;
        }
      },
    );
    setTimeout(() => {
      invokePayloadWithCallback<UserEvent>(
        {
          type: "RequestWorkspacePayload",
        },
        (e, r) => {
          workspacePayload = r;
        },
      );
      invokePayload<UserEvent>({
        type: "SetWindowSize",
        value: { width: 2560, height: 45 },
      });
      invokePayload<UserEvent>({
        type: "SetWindowPosition",
        value: { x: 0, y: 0 },
      });
    }, 500);
  });
  onDestroy(() => {
    if (listenFn) listenFn();
  });
  function getIcon(name: String) {
    const lower = name.toLowerCase();
    if (/zen\.exe$/.test(lower)) return Browser;
    if (/chrome\.exe$/.test(lower)) return Browser;
    if (/tsck\.exe$/.test(lower)) return Tsck;
    if (/explorer\.exe$/.test(lower)) return File;
    if (lower.includes("whatsapp")) return Whatsapp;
    if (lower.includes("photoshop")) return Photoshop;
    if (/zed\.exe$/.test(lower)) return Zed;

    return Question;
  }
</script>

<div
  class="m-0 flex flex-row h-screen w-full items-center justify-center gap-2 overflow-hidden p-0"
>
  {#if workspacePayload}
    <div class="flex flex-row gap-2 items-center justify-center">
      {#each Array.from({ length: workspacePayload.count }) as i, index}
        <div
          class="min-h-2 min-w-14 rounded-md {activeWorkSpace == index
            ? 'bg-error'
            : 'bg-neutral'}"
        >
          <div class="flex flex-row gap-1 px-2">
            {#each workspacePayload.entries as entry}
              {#if index == entry.workspace}
                <IconButton
                  class="btn-ghost! *:stroke-3 {activeApp == entry.hwnd
                    ? ''
                    : 'opacity-50'}"
                  icon={getIcon(entry.app)}
                />
              {/if}
            {/each}
          </div>
        </div>
      {/each}

      {#if activeTitle}
        <div class="title px-4 flex justify-end">
          <div class="text-xs flex flex-row gap-2">
            <span class="font-avantt font-black"
              >{activeAppName?.replace(".exe", "").toUpperCase()}</span
            > <span class="font-sans italic">{activeTitle}</span>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>
