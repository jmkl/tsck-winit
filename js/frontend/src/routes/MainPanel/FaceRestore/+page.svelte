<script lang="ts">
  import { GetAppsState } from "$lib/AppState.svelte";
  import gsap from "gsap";
  import mia from "$lib/assets/yt-thumb.png";
  import Crop from "$lib/components/Icon/Crop.svelte";
  import Layers from "$lib/components/Icon/Layers.svelte";
  import { slideMe } from "$lib/animation";
  import Logger from "$lib/Logger";
  import NodeBuilder from "$lib/comfyui/node/NodeBuilder.svelte";
  import CustomNodeBuilder from "$lib/comfyui/node/CustomNodeBuilder.svelte";
  import { invokePayload } from "$lib";
  import { type UserEvent } from "@tsck/lib";
  const DURATION = 0.3;
  const DELAY = 0.3;

  type TemplateModeType = "FaceRestore" | "SDUpscale";

  const ctx = GetAppsState();
  let isProcessing = $state(false);
  let imageContainer: HTMLImageElement | undefined = $state();
  let tempImageSrc: string | undefined = $state(getPreviewUrl());

  function getPreviewUrl() {
    if (!ctx.FacerestoreImageFileName) return undefined;
    return `http://${ctx.AppConfig?.comfyui_url}/api/view?type=input&filename=${ctx.FacerestoreImageFileName}`;
  }

  let currentTimeline: gsap.core.Timeline | null = null;

  function startProcessing() {
    if (!imageContainer) return;

    // Kill any existing animations
    if (currentTimeline) {
      currentTimeline.kill();
    }
    gsap.killTweensOf(imageContainer);

    // Reset to initial state
    gsap.set(imageContainer, {
      scale: 1,
      rotateY: 0,
      rotateX: 0,
      z: 0,
      opacity: 1,
    });

    // Create infinite loading animation
    currentTimeline = gsap.timeline({ repeat: -1 });

    currentTimeline

      .to(imageContainer, {
        rotateY: 360,
        duration: 1.2,
        ease: "none",
      })
      .to(imageContainer, {
        rotateY: 0, // Reset rotation for next loop
        duration: 0,
      });
  }

  function completeWithTransition() {
    if (!imageContainer || !currentTimeline) return;

    // Get current rotation
    const currentRotation = gsap.getProperty(
      imageContainer,
      "rotateY",
    ) as number;

    // Calculate nearest 360° completion
    const targetRotation = Math.ceil(currentRotation / 360) * 360;
    const remainingRotation = targetRotation - currentRotation;
    const completionDuration = (remainingRotation / 360) * 1.2;

    // Kill the infinite loop
    currentTimeline.kill();
    currentTimeline = null;

    // Create completion timeline
    const completionTimeline = gsap.timeline();

    completionTimeline
      .to(imageContainer, {
        rotateY: targetRotation,
        duration: completionDuration,
        ease: "none",
      })
      .to(imageContainer, {
        rotateY: targetRotation + 90,
        opacity: 0,
        duration: 0.1,
        ease: "power2.inOut",
        onComplete: () => {
          tempImageSrc = getPreviewUrl();
        },
      })

      .to(imageContainer, {
        rotateY: targetRotation + 180,
        opacity: 1,
        duration: 0.3,
        ease: "power2.inOut",
      })
      // Elastic scale back
      .to(imageContainer, {
        rotateX: 0,
        rotateY: targetRotation + 0,
        z: 0,
        scale: 1,
        duration: 0.3,
        ease: "power2.out",
        onComplete: () => {
          isProcessing = false;
        },
      });
  }

  // Start animation when processing begins
  $effect(() => {
    if (isProcessing) {
      startProcessing();
    }
  });

  // Complete animation when image source changes
  $effect(() => {
    if (ctx.FacerestoreImageFileName) {
      setTimeout(() => {
        if (isProcessing && currentTimeline) {
          completeWithTransition();
        }
      }, 1000);
    }
  });
  function nodeOnChange(content: any) {
    Logger.error(content.map((c: any) => c.title + "-" + c.value));
  }

  let TemplateMode: TemplateModeType = $state("FaceRestore");
</script>

<div class="relative z-2 h-full w-full overflow-hidden">
  <div
    in:slideMe={{ x: -300, duration: DURATION }}
    out:slideMe={{ x: -300, duration: DURATION }}
    class="relative flex w-full flex-col"
  >
    {#if TemplateMode.includes("FaceRestore")}
      <NodeBuilder onChange={nodeOnChange} />
    {:else if TemplateMode.includes("SDUpscale")}
      <CustomNodeBuilder />
    {/if}
    <div class="relative z-1 w-full p-8">
      <div class="buttons pointer-events-auto absolute top-2 right-2">
        <button
          onclick={() => {
            isProcessing = true;
            invokePayload<UserEvent>({
              type: "PerformSelectionToImage",
            });
          }}
          disabled={!ctx.facerestoreSelectionMode}
          class="btn btn-square btn-soft btn-error [&>svg]:h-4 [&>svg]:w-4"
        >
          <Crop />
        </button>
        <button
          onclick={() => {
            isProcessing = true;
            invokePayload<UserEvent>({ type: "PerformLayerToImage" });
          }}
          class="btn btn-square btn-soft btn-error [&>svg]:h-4 [&>svg]:w-4"
        >
          <Layers />
        </button>
      </div>
      <div class="flex h-auto w-full items-center overflow-hidden">
        <img
          bind:this={imageContainer}
          src={tempImageSrc ?? mia}
          onerror={(e) => {
            (e.target as HTMLImageElement).onerror = null;
            const t = e.target as HTMLImageElement;
            t.src = mia;
          }}
          alt=""
          class="pointer-events-none w-full rounded-md object-contain select-none"
        />
      </div>
    </div>
  </div>
</div>
