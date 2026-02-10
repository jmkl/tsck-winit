<script lang="ts">
  import { invokePayload, listen, type UnlistenFn } from "$lib";
  import "$lib/layout.css";
  import type { EventPayload, UserEvent } from "@tsck/lib";
  import { onDestroy, onMount } from "svelte";
  let activeWorkSpace = $state(0);
  let listenFn: UnlistenFn | undefined = $state();
  onMount(() => {
    listenFn = listen<EventPayload, UserEvent>(
      "tsck::event|EVENTPAYLOAD::FRONTEND",
      (e) => {
        if (e == undefined) return;
        switch (e.type) {
          case "ActivateWorkSpace":
            console.log(e.value);
            activeWorkSpace = Number(e.value);
            break;
        }
      },
    );
    setTimeout(() => {
      invokePayload<UserEvent>({
        type: "SetWindowSize",
        value: { width: 400, height: 45 },
      });
    }, 1000);
  });
  onDestroy(() => {
    if (listenFn) listenFn();
  });
</script>

<div
  class="m-0 flex h-screen w-full flex-row items-center justify-center gap-2 overflow-hidden p-0"
>
  {#each Array.from({ length: 3 }, (_, i) => i) as i, index}
    <div
      class="h-2 w-14 rounded-md {activeWorkSpace == index
        ? 'bg-error'
        : 'bg-neutral'}"
    ></div>
  {/each}
</div>
