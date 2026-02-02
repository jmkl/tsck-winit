<script lang="ts">
  import { GetRootState, SetRootState } from "$lib/RootState.svelte";
  import { onDestroy, onMount } from "svelte";
  import CommandLog from "./CommandLog/+page.svelte";
  import MainApp from "./MainApp.svelte";
  import { invokePayloadWithCallback, listen, type UnlistenFn } from "$lib";
  import { type UserEvent, type AppConfig, type EventPayload } from "@tsck/lib";
  let interval: NodeJS.Timeout | undefined = $state();
  let comfyuiUrl = $state();
  SetRootState();
  const ctx = GetRootState();
  let listenFn: UnlistenFn | undefined = $state();
  async function checkServer() {
    if (!comfyuiUrl) return;
    const response = await fetch(`http://${comfyuiUrl}/prompt`);
    if (response.ok) {
      ctx.IsConnected = true;
    }
  }

  onMount(() => {
    invokePayloadWithCallback<UserEvent, AppConfig>(
      { type: "GetAppConfig" },
      (error, result) => {
        if (error || !result) return;
        comfyuiUrl = result.comfyui_url;
      },
    );

    if (interval) clearInterval(interval);
  });
  onDestroy(() => {
    if (listenFn) listenFn();
    if (interval) clearInterval(interval);
  });
</script>

{#if !ctx.IsConnected}
  <div
    class="border-base-300 bg-base-300 flex h-screen w-screen flex-col overflow-hidden rounded-md border"
  >
    <CommandLog
      onConnected={() => {
        interval = setInterval(() => {
          if (ctx.IsConnected) {
            clearInterval(interval);
          }
          checkServer();
        }, 1000);
      }}
      splashScreen={true}
    />
  </div>
{:else}
  <MainApp />
{/if}
