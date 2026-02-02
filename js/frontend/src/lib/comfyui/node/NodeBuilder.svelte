<script lang="ts">
  import Play from "$lib/components/Icon/Play.svelte";
  import { fade } from "svelte/transition";
  import { BuildFaceRestoreNode } from "../NodeBuilderHelper";
  import DropdownNode from "./DropdownNode.svelte";
  import InputNode from "./InputNode.svelte";
  import type { ComfyUINode } from "./NodeTypes";
  import NumberNode from "./NumberNode.svelte";
  import ToggleNode from "./ToggleNode.svelte";
  import { GetAppsState } from "$lib/AppState.svelte";
  import { onMount } from "svelte";
  import type { UserEvent } from "../../../../../@tsck/dist";
  import { invokePayload } from "$lib";
  const TEMPLATE = "LOCAL_FACERESTORE_TEMPLATE";
  let { onChange = null } = $props();
  const ctx = GetAppsState();

  function findNode(key: string) {
    return nodes.find((n) => n.title == key);
  }
  const DEFAULT_NODE = [
    {
      title: "Upscale Model",
      type: "DropdownNode",
      value: { models: ctx.comfyuiAPI?.upscaleModel, selectedIndex: 0 },
    },
    {
      title: "Facerestore Model",
      type: "DropdownNode",
      value: {
        models: ctx.comfyuiAPI?.faceRestoreModel,
        selectedIndex: 0,
      },
    },
    {
      title: "RMBG Model",
      type: "DropdownNode",
      value: { models: ctx.comfyuiAPI?.rmbgModel, selectedIndex: 0 },
    },
    {
      title: "Remove Background",
      type: "ToggleNode",
      value: { checked: false },
    },
    {
      title: "Processing Resolution",
      type: "NumberNode",
      value: {
        min: 256,
        max: 1024,
        step: 8,
        value: 512,
        isFloat: false,
      },
    },
  ];
  let nodes: ComfyUINode[] = $state(fetchLocalStorage());
  let selectedUpscaleModel = $derived.by(() => {
    let node = findNode("Upscale Model");
    if (node && node.type == "DropdownNode") {
      return node.value.models[node.value.selectedIndex];
    } else {
      return undefined;
    }
  });
  let selectedFaceRestoreModel = $derived.by(() => {
    let node = findNode("Facerestore Model");
    if (node && node.type == "DropdownNode") {
      return node.value.models[node.value.selectedIndex];
    } else {
      return undefined;
    }
  });
  let selectedRMBGModel = $derived.by(() => {
    let node = findNode("RMBG Model");
    if (node && node.type == "DropdownNode") {
      return node.value.models[node.value.selectedIndex];
    } else {
      return undefined;
    }
  });
  let removeBackground = $derived.by(() => {
    let node = findNode("Remove Background");
    if (node && node.type == "ToggleNode") {
      return node.value.checked;
    } else {
      return false;
    }
  });
  let info = $state("");
  const flattenNodes = () => {
    return nodes.map((node) => {
      let value = null;
      switch (node.type) {
        case "DropdownNode":
          value = node.value.selectedIndex;
          break;
        case "ToggleNode":
          value = node.value.checked;
          break;
        case "NumberNode":
          value = node.value.value;
          break;
      }
      return { title: node.title, value };
    });
  };
  function saveRecentToLocalStorage() {
    if (onChange) onChange(flattenNodes());
    localStorage.setItem(TEMPLATE, JSON.stringify(nodes));
  }
  function fetchLocalStorage() {
    let item = localStorage.getItem(TEMPLATE);
    const result = item ? JSON.parse(item) : DEFAULT_NODE;
    return result;
  }
  onMount(() => {});

  function generateThatShit() {
    saveRecentToLocalStorage();
    info = "";
    if (
      !selectedUpscaleModel ||
      !selectedFaceRestoreModel ||
      !selectedRMBGModel
    ) {
      info = "Model can't be null";
      return;
    }
    if (!ctx.FacerestoreImageFileName) {
      info = "Image not set yet.. crop something";
      return;
    }
    //save current setting into localstorage
    BuildFaceRestoreNode(
      ctx.FacerestoreImageFileName,
      selectedUpscaleModel,
      selectedFaceRestoreModel,
      removeBackground,
      selectedRMBGModel,
    )
      .then(async (result) => {
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
      })
      .catch((e) => {
        info = e;
      });
  }
  $effect(() => {});
</script>

<div class="flex w-full flex-col p-4">
  {#each nodes as node}
    {#if node.type == "InputNode"}
      <InputNode title={node.title} bind:value={node.value} />
    {:else if node.type == "DropdownNode"}
      <DropdownNode
        onSelectionChange={saveRecentToLocalStorage}
        title={node.title}
        items={node.value.models}
        bind:selectedIndex={node.value.selectedIndex}
      ></DropdownNode>
    {:else if node.type == "ToggleNode"}
      <ToggleNode
        bind:checked={node.value.checked}
        onchange={saveRecentToLocalStorage}
        title={node.title}
      ></ToggleNode>
    {:else if node.type == "NumberNode"}
      <NumberNode
        float={node.value.isFloat}
        step={node.value.step}
        min={node.value.min}
        max={node.value.max}
        bind:value={node.value.value}
        title={node.title}
        onchange={saveRecentToLocalStorage}
      ></NumberNode>
    {/if}
  {/each}
  <div class="flex w-full p-1">
    <button
      onclick={generateThatShit}
      disabled={ctx.FacerestoreImageFileName == undefined}
      class="btn flex-1 btn-soft btn-error [&>svg]:h-3 [&>svg]:w-3 [&>svg]:stroke-3"
    >
      <span class="font-unisans text-[10px] leading-0 font-medium">
        Generate
      </span>
      <Play />
    </button>
  </div>
  {#key info}
    <div transition:fade class="font-avantt text-xs text-error italic">
      {info}
    </div>
  {/key}
</div>
