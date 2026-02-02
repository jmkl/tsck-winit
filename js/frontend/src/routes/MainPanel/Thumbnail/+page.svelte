<script lang="ts">
  import { cicleRange, ScaleRange, type TemplateLine } from "./stringUtils";
  import LineEdit from "./LineEdit.svelte";
  import { GetAppsState } from "$lib/AppState.svelte";
  import {
    getShiftedNum,
    useKeyboard,
    type KeyConfig,
  } from "$lib/useKeyboard.svelte";
  import { onMount } from "svelte";
  import { invokePayload } from "$lib";
  import ThumbnailPreview from "./ThumbnailPreview.svelte";
  import TodoList from "./TodoList.svelte";
  const ctx = GetAppsState();
  let canvasMargin = $state(0);
  let typeface = $derived<"font-unisans" | "font-anton">(ctx.ThumbnailTypeface);
  let showEditor = $state(false);
  let forceFocus = $state(false);
  let focusTodo = $state(false);
  let editMode = $state(false);
  let templateFocusIndex = $state(-1);
  let templateButtonParent: HTMLDivElement | undefined = $state();
  let forceKey = $state(0);
  let FadeMinimap = $state(false);
  let templateNames = [
    { init: "RP", name: "REPOT", template: "RHC-DICECAR.psd" },
    { init: "DC", name: "DICECAR", template: "RHC-DICECAR.psd" },
    { init: "RH", name: "RHC", template: "RHC-RHC-2025-NEW.psd" },
    { init: "NF", name: "NAUFAL", template: "RHC-NAUFAL.psd" },
    { init: "KC", name: "KRNCDS", template: "RHC-KERENCADAS.psd" },
    { init: "RS", name: "REHABS", template: "RHC-RHC-2025-NEW.psd" },
    { init: "GG", name: "GRUPAG", template: "RHC-GRUPAG.psd" },
  ];
  function reset() {
    forceFocus = false;
    focusTodo = false;
    showEditor = false;
    templateFocusIndex = -1;
    (document.activeElement as HTMLElement)?.blur();
  }
  function sizeToHotkey(idx: number): KeyConfig[] {
    return [
      {
        key: idx.toString(),
        ctrl: false,
        handler: () => {
          if (idx > ctx.TodoTemplateLines.length) return;
          ctx.TodoTemplateLines[idx - 1].scale = cicleRange(
            ctx.TodoTemplateLines[idx - 1].scale + 1,
            0,
            ScaleRange.length - 1,
          );
        },
      },
      {
        key: getShiftedNum(idx.toString()),
        handler: () => {
          if (idx > ctx.TodoTemplateLines.length) return;

          ctx.TodoTemplateLines[idx - 1].italic =
            !ctx.TodoTemplateLines[idx - 1].italic;
        },
      },
    ];
  }
  const keyboardConfig = [
    {
      key: "Tab",
      ignoreInputs: true,
      handler: () => {
        focusTodo = true;
      },
    },
    {
      key: "t",
      handler: () => {
        typeface = typeface == "font-unisans" ? "font-anton" : "font-unisans";
      },
    },

    {
      key: "n",
      ctrl: true,
      handler: () => {
        const payload = {
          type: "FunctionCall",
          value: {
            func: "createNewDocument",
            args: [],
          },
        };
        invokePayload(payload);
      },
    },
    {
      key: "Tab",
      ignoreInputs: false,
      handler: () => {
        forceFocus = false;
      },
    },
    {
      key: "Escape",
      ignoreInputs: false,
      handler: reset,
    },
    {
      key: "E",
      ctrl: true,
      ignoreInputs: false,
      handler: () => {
        showEditor = !showEditor;
        forceFocus = false;
      },
    },
    {
      key: "f",
      ignoreInputs: true,
      handler: () => {
        FadeMinimap = !FadeMinimap;
      },
    },
    {
      key: "e",
      ignoreInputs: true,
      handler: () => {
        if (!showEditor) showEditor = !showEditor;
        forceFocus = showEditor ? true : false;
      },
    },
    {
      key: "ArrowRight",
      ctrl: true,
      handler: () => {
        templateFocusIndex = (templateFocusIndex + 1) % templateNames.length;
        (
          templateButtonParent?.childNodes[
            templateFocusIndex
          ] as HTMLAnchorElement
        )?.focus();
      },
    },
    {
      key: "ArrowLeft",
      ctrl: true,
      handler: () => {
        templateFocusIndex =
          (templateFocusIndex - 1 + templateNames.length) %
          templateNames.length;
        (
          templateButtonParent?.childNodes[
            templateFocusIndex
          ] as HTMLAnchorElement
        )?.focus();
      },
    },
    {
      key: "Enter",
      handler: () => {
        if (templateFocusIndex < 0) return;
        (
          templateButtonParent?.childNodes[
            templateFocusIndex
          ] as HTMLAnchorElement
        )?.click();
      },
    },
    {
      key: "Escape",
      handler: () => {
        showEditor = false;
      },
      when: () => showEditor,
    },

    ...sizeToHotkey(1),
    ...sizeToHotkey(2),
    ...sizeToHotkey(3),
    ...sizeToHotkey(4),
    ...sizeToHotkey(5),
    ...sizeToHotkey(6),
    ...sizeToHotkey(7),
    ...sizeToHotkey(8),
  ];

  onMount(() => {
    useKeyboard(keyboardConfig);
  });

  // =============================================
  // EFFECTS
  // =============================================
  $effect(() => {
    if (editMode) showEditor = false;
  });
</script>

<div class="relative flex h-full w-full flex-col justify-start overflow-hidden">
  <ThumbnailPreview
    class={FadeMinimap ? "opacity-10" : "opacity-100"}
    templateLines={ctx.TodoTemplateLines}
    {typeface}
    bind:canvasMargin
  />
  <div style="height:{canvasMargin}px;" class="spacing w-full shrink-0"></div>
  {#if showEditor}
    <LineEdit
      {showEditor}
      bind:forceFocus
      onChange={(texts) => {
        ctx.TodoTemplateLines = texts;
        forceKey += 1;
      }}
      texts={ctx.TodoTemplateLines}
    />
  {:else}
    <div
      bind:this={templateButtonParent}
      class="debug-border template-holder join bg-base-300 shrink-0 flex-wrap justify-center"
    >
      {#each templateNames as template, index}
        <a
          role="button"
          data-index={index}
          href="/"
          class=" join-item font-avantt [:hover,:focus,:active]:text-error text-[15px]
        leading-3 font-extrabold tracking-tighter text-white opacity-20 transition-all duration-150 ease-in-out hover:opacity-100 focus:opacity-100
        active:opacity-100 [:hover,:focus,:active]:font-black"
          onclick={(e) => {
            const payload = {
              type: "Template",

              value: {
                template: {
                  name: template.template,
                  content: ctx.TodoTemplateLines,
                  gap: 15,
                  padding: 30,
                },
              },
            };

            invokePayload(payload);
            ctx.resetShadowLayer();
            e.preventDefault();
          }}
        >
          {template.name}
        </a>
      {/each}
    </div>
  {/if}
  <div
    class="debug-border pointer-events-auto flex h-full flex-1 shrink-0 flex-col overflow-y-auto"
  >
    {#if !focusTodo}
      <div
        class="z-99 p-1 px-2 bg-black/20 backdrop-blur-sm rounded-md shadow-md opacity-100 pointer-events-none absolute top-1/2 -translate-y-1/2 font-black text-md text-center font-avantt -translate-x-1/2 left-1/2 content"
      >
        <div class="bg-blend-multiply">Press Tab</div>
      </div>
    {/if}
    <TodoList
      appendTodo={(response: TemplateLine[]) => {
        ctx.TodoTemplateLines = response;
        forceKey += 1;
      }}
      bind:focusMe={focusTodo}
    />
  </div>
</div>
