<script lang="ts">
  import { GetAppsState } from "$lib/AppState.svelte";
  import gsap from "gsap";
  import Add from "$lib/components/Icon/Add.svelte";
  import Expand from "$lib/components/Icon/Expand.svelte";
  import Play from "$lib/components/Icon/Play.svelte";
  import MinSlider from "$lib/components/MinSlider.svelte";
  import { circInOut } from "svelte/easing";
  import { slide } from "svelte/transition";
  import { RAWFILTER_VALUE_DEFAULT, RAWFILTER_WHITELIST } from "./data";
  import IconButton from "$lib/components/IconButton.svelte";
  import EyeOpen from "$lib/components/Icon/EyeOpen.svelte";
  import type { RawFilterDataType, UserEvent } from "@tsck/lib";
  import { invokePayload } from "$lib";
  import HorizontalDivider from "$lib/components/HorizontalDivider.svelte";
  import MultiSlider from "$lib/components/MultiSlider.svelte";
  import Circle from "$lib/components/Circle.svelte";
  const ctx = GetAppsState();
  let compactMode = $state(true);
  let inputNameVisible = $state(false);
  let newTemplateName = $state("");
  function applyRawfilter() {
    const rf_data: RawFilterDataType = ctx
      .rawfilterGetRawFilterData()
      .reduce((acc, rf) => {
        acc[rf.name as keyof RawFilterDataType] = rf.value;
        return acc;
      }, {} as RawFilterDataType);
    const payload: UserEvent = {
      type: "ApplyRawFilter",
      value: rf_data,
    };
    invokePayload(payload);
  }
  function applyTriColor() {
    const payload: UserEvent = {
      type: "ApplyTriColor",
      value: {
        tri_color: ctx.rawfilterTriColor,
        position: ctx.rawfilterColorRanges,
      },
    };
    invokePayload(payload);
  }
  let animInterval: NodeJS.Timeout | undefined;
  const rfIndex = () => {
    return ctx.getLocalStorageItem("selected_rawfilter_template") as number;
  };
</script>

<div class="relative z-1 flex h-full w-full flex-col overflow-hidden">
  <div class="relative container flex h-full w-full flex-col">
    <div
      class="left-0 z-10 flex w-full flex-row p-2 transition-all duration-75"
    >
      <div class="join flex-1">
        <select
          onchange={(e) => {
            const target = e.target as HTMLSelectElement;
            ctx.updateLocalStorageItem(
              "selected_rawfilter_template",
              target.selectedIndex,
            );
          }}
          class="select join-item flex-1 bg-base-300 select-xs ring-0 outline-0 select-error focus:ring-0 focus:outline-none"
        >
          {#each ctx.rawfilterTemplates as rfTemplate, index}
            <option selected={rfIndex() == index} value={rfTemplate.value}>
              {rfTemplate.name}
            </option>
          {/each}
        </select>
        <IconButton
          onclick={() => {
            if (rfIndex() == undefined) return;
            const data = ctx.rawfilterTemplates[rfIndex()].value;
            ctx.rawfilterUpdateRawfilterData(
              ctx.rawfilterGetRawFilterData().map((item) => ({
                ...item,
                value: data[item.name as keyof RawFilterDataType],
              })),
            );
            applyRawfilter();
          }}
          class="btn join-item border-error btn-soft btn-sm btn-error [&>svg]:h-4 [&>svg]:w-4"
          icon={Play}
        />
      </div>

      <div class="w-4"></div>
      <div class="join">
        <button
          onclick={() => (inputNameVisible = !inputNameVisible)}
          class="btn join-item btn-square btn-soft btn-sm btn-error [&>svg]:h-4 [&>svg]:w-4"
        >
          <Add />
        </button>
        <button
          onclick={() => (compactMode = !compactMode)}
          class="btn join-item btn-square btn-soft btn-sm btn-warning [&>svg]:h-4 [&>svg]:w-4"
        >
          <Expand />
        </button>
      </div>
    </div>

    {#if inputNameVisible}
      <div
        transition:slide={{ duration: 100, easing: circInOut }}
        class="join flex flex-row justify-center bg-black/20 p-2"
      >
        <input
          type="text"
          bind:value={newTemplateName}
          class=" input input-sm join-item input-error"
        />
        <button
          onclick={() => {
            if (newTemplateName == "") return;

            const values = ctx
              .rawfilterGetRawFilterData()
              .reduce((acc, item) => {
                acc[item.name as keyof RawFilterDataType] = item.value;
                return acc;
              }, {} as RawFilterDataType);
            if (!values) return;
            const saveThisNewTemplate = {
              name: newTemplateName,
              value: values,
            };
            ctx.rawfilterSaveRawFilterTemplate(saveThisNewTemplate);

            //reset the state and close the shit
            newTemplateName = "";
            inputNameVisible = false;
          }}
          class=" btn join-item btn-soft btn-sm btn-error"
        >
          Save
        </button>
      </div>
    {/if}

    <HorizontalDivider height={1} />
    <div class="rf-slider-container grid {'grid-cols-1 px-4'}">
      {#each ctx.rawfilterGetRawFilterData() as rf}
        {#if !compactMode || RAWFILTER_WHITELIST.includes(rf.name)}
          <MinSlider
            class={"text-[10px]"}
            resetValue={() => {
              rf.value =
                RAWFILTER_VALUE_DEFAULT.find((v) => v.name === rf.name)
                  ?.value ?? 0;
              applyRawfilter();
            }}
            onChange={(v: number) => {}}
            onAssign={(v: number) => {
              const item = ctx
                .rawfilterGetRawFilterData()
                ?.find((d) => d.name === rf.name);
              if (item) item.value = v;
              applyRawfilter();
            }}
            min={rf.min}
            max={rf.max}
            bind:value={rf.value}
            step={rf.step}
            title={rf.name}
          />
        {/if}
      {/each}
    </div>
    <HorizontalDivider height={5} />
    <div class="rf-slider-container flex flex-col px-5">
      <MultiSlider
        bind:pipRange={ctx.rawfilterPipsRanges}
        bind:values={ctx.rawfilterColorRanges}
        bind:tri_color={ctx.rawfilterTriColor}
        sendColorData={applyTriColor}
      />
    </div>
    <HorizontalDivider height={8} />
    <div class="multicolor flex flex-row justify-center odd:*:gap-2">
      {#each ctx.rawfilterColorList as col, index}
        <Circle
          indexKe={index}
          click={(e: MouseEvent) => {
            let index = 0;
            if (e.shiftKey) index = 1;
            else if (e.ctrlKey || e.metaKey) index = 2;
            ctx.rawfilterTriColor[index] = col;
            applyTriColor();
          }}
          class="col cursor-pointer"
          color={col}
        />
      {/each}
    </div>
  </div>
</div>
