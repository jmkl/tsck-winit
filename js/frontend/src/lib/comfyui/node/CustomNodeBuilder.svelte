<script lang="ts">
  import { GetAppsState } from "$lib/AppState.svelte";
  import { onMount } from "svelte";
  import { sdUpscaleTemplate } from "../templates";
  import Play from "$lib/components/Icon/Play.svelte";
  import { invokePayload } from "$lib";
  import { type UserEvent } from "@tsck/lib";
  const ctx = GetAppsState();

  onMount(() => {});

  async function generateThatShit() {
    if (!ctx.FacerestoreImageFileName) return;
    const result = sdUpscaleTemplate(ctx.FacerestoreImageFileName);
    ctx.showLoadingPanel(true);
    const images = await ctx.comfyuiAPI?.deployComfyUIPrompt(result);
    const all_images = Object.values(images).flat() as string[];
    const payload: UserEvent = {
      type: "AppendComfyUIOutput",
      value: {
        images: all_images,
        bounds: ctx.facerestoreSelectionBound,
      },
    };
    invokePayload(payload);
    ctx.showLoadingPanel(false);
    invokePayload<UserEvent>({ type: "GenerateImage" });
  }
  $effect(() => {});
</script>

<div class="flex w-full flex-col p-4">
  <div class="flex w-full p-1">
    <button
      onclick={generateThatShit}
      disabled={ctx.FacerestoreImageFileName == undefined}
      class="btn btn-soft btn-error flex-1 [&>svg]:h-3 [&>svg]:w-3 [&>svg]:stroke-3"
    >
      <span class="font-unisans text-[10px] leading-0 font-medium">
        Generate
      </span>
      <Play />
    </button>
  </div>
</div>
