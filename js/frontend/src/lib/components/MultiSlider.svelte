<script lang="ts">
  import { GetAppsState } from "$lib/AppState.svelte";
  import gsap from "gsap";
  import { onMount } from "svelte";
  import RangeSlider from "svelte-range-slider-pips";
  const ctx = GetAppsState();
  let {
    values = $bindable(),
    tri_color = $bindable(),
    pipRange = $bindable(),
    sendColorData,
  } = $props();

  let min = $state(0);
  let max = $state(4096);
  let container: HTMLElement | undefined = $state();
  let containerWidth = $state(300);
  let dongle: HTMLElement | undefined = $state();

  let colorGap = $derived([
    mapRange(values[0]).toFixed(2) + "%",
    mapRange(values[1]).toFixed(2) + "%",
  ]);
  let sliderParent: HTMLDivElement | undefined = $state();
  let sliderWidth: number = $state(300);
  let tricolBg = $derived(triColorBackground());
  function getPipPos(range: number) {
    return sliderParent ? Math.round((range / 4096) * sliderWidth) : 0;
  }
  function handlePipClick(e: MouseEvent, r: number) {
    e.preventDefault();
    if (e.ctrlKey) {
      // Adjust upper range
      if (r > values[0]) {
        values[1] = r;
      }
    } else {
      // Adjust lower range
      if (r < values[1]) {
        values[0] = r;
      }
      if (r > values[1]) {
        values[1] = r;
      }
    }

    sendColorData();
  }
  onMount(() => {
    if (sliderParent) {
      sliderWidth = sliderParent.clientWidth;
      containerWidth = sliderWidth;
    }
  });

  function mapRange(value: number): number {
    return (value / 4096) * 100;
  }
  function mapRangeWindow(value: number): number {
    const w = containerWidth ?? 300;
    return (value / 4096) * w + 8;
  }

  function reAlign(e: any) {
    animateDongle(e);
    const handle = e.detail.activeHandle;
    const v = e.detail.value;
    const tolerance = 80;
    let closest = pipRange.reduce((prev: number | null, curr: number) => {
      if (Math.abs(curr - v) > tolerance) return prev;
      if (prev === null) return curr;
      return Math.abs(curr - v) < Math.abs(prev - v) ? curr : prev;
    }, null);

    values[handle] = closest ?? v;
    sendColorData();
  }

  function triColorBackground() {
    return `linear-gradient(90deg,${tri_color[0]} 0%, ${tri_color[0]} ${colorGap[0]}, ${tri_color[1]} ${colorGap[0]}, ${tri_color[1]} ${colorGap[1]}, ${tri_color[2]} ${colorGap[1]}, ${tri_color[2]} 100%);`;
  }
  function bar(index: number) {
    return mapRangeWindow(values[index]) + "px";
  }
  function animateDongle(e: any) {
    if (!dongle) return;
    const d = e.detail;
    const value = d.value;
    const pos = mapRangeWindow(value) - dongle.clientWidth / 2 + "px";
    gsap.to(dongle, {
      x: pos,
    });
  }
</script>

<svelte:window
  onresize={() => {
    sliderWidth = sliderParent?.clientWidth ?? sliderWidth;
    containerWidth = sliderParent?.clientWidth ?? 300;
  }}
/>

{#snippet renderPips(r: number)}
  {@const pos = getPipPos(r)}
  <a
    href="/"
    aria-label="link"
    class="range absolute z-5"
    style={`left:${pos}px`}
    onclick={(e) => handlePipClick(e, r)}
  ></a>
{/snippet}
{#snippet slipbar(idx: number)}
  <div
    style={`left:${bar(idx)};`}
    class="bar1 absolute -top-0.75 left-[3] z-1 h-5 w-px bg-black"
  ></div>
{/snippet}
<div bind:this={container} class="tri-color-slider relative mt-3 w-full">
  {@render slipbar(0)}
  {@render slipbar(1)}
  <div
    style={`background:${tricolBg}`}
    bind:this={sliderParent}
    class="bar-preview absolute left-[0.5em] h-2.5 w-[calc(100%-1em)] rounded-xs"
  >
    {@render renderPips(0)}
    {#each pipRange as r}
      {@render renderPips(r)}
    {/each}
    {@render renderPips(4096)}
  </div>

  <RangeSlider
    on:change={(e) => {}}
    on:stop={reAlign}
    all="label"
    pushy
    draggy
    {min}
    {max}
    step={1}
    id="multislider"
    range
    bind:values
    springValues={{ stiffness: 0.2, damping: 0.4 }}
  />
</div>

<style>
  .range {
    background: url("data:image/svg+xml,%3C%3Fxml%20version%3D%221.0%22%20encoding%3D%22UTF-8%22%3F%3E%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%2011.34%2014.17%22%3E%3Cpath%20fill%3D%22%232a2b2a%22%20d%3D%22M5.67%2C14.17c-.63%2C0-1.23-.31-1.6-.83-1.09-1.54-2.88-3.97-2.88-3.97-.04-.06-.08-.12-.12-.19C.12%2C7.83-.21%2C6.2.13%2C4.56.6%2C2.34%2C2.37.55%2C4.53.12c.38-.08.76-.12%2C1.14-.12%2C3.13%2C0%2C5.67%2C2.61%2C5.67%2C5.81%2C0%2C1.21-.37%2C2.37-1.07%2C3.37-.03.06-.07.13-.11.18%2C0%2C0-1.79%2C2.43-2.88%2C3.97-.38.53-.97.84-1.6.84Z%22%2F%3E%3Cpath%20fill%3D%22%23fff%22%20d%3D%22M9.98%2C5.81c0-2.74-2.43-4.89-5.19-4.33-1.65.33-2.97%2C1.69-3.33%2C3.38-.3%2C1.4.08%2C2.69.83%2C3.67h-.02s1.79%2C2.45%2C2.89%2C3.99c.25.35.76.35%2C1.01%2C0%2C1.1-1.54%2C2.89-3.98%2C2.89-3.98h-.02c.58-.76.93-1.69.93-2.72Z%22%2F%3E%3Cpath%20fill%3D%22%23dcdbdb%22%20d%3D%22M4.79%2C1.48c-1.65.33-2.97%2C1.69-3.33%2C3.38-.3%2C1.4.08%2C2.69.83%2C3.67h-.02s1.79%2C2.45%2C2.89%2C3.99c.13.18.32.27.51.27V1.4c-.29%2C0-.58.02-.88.08Z%22%2F%3E%3Cellipse%20fill%3D%22%232a2b2a%22%20cx%3D%225.67%22%20cy%3D%225.52%22%20rx%3D%222.1%22%20ry%3D%222.15%22%2F%3E%3C%2Fsvg%3E")
      no-repeat center;

    height: 14px;
    width: 14px;
    filter: hue-rotate(0.3);
    filter: drop-shadow(0, 2px, 4px, black);
    transform: translateX(-7px) translateY(-10px);
    transition: 0.2s ease;
  }
  .range:hover {
    transform: translateX(-7px) translateY(-8px);
  }

  :global(.rsPip) {
    display: none;
  }
</style>
