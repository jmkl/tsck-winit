<script lang="ts">
  import { onMount } from "svelte";
  import { GetAppsState, SetAppsState } from "./lib/AppState.svelte";
  import { logger } from "./lib/utils/addLog";
  SetAppsState();
  const ctx = GetAppsState();
  let size = $state({ w: 78, h: 100 });
  let mainParent: HTMLDivElement | undefined = $state();
  let value = $derived(ctx.cycleImage);
  let onoff = $derived(ctx.onOff);
  onMount(() => {
    onResize();
  });
  function onHover() {
    ctx.cycleImage = (ctx.cycleImage + 1) % 4;
  }
  function onResize() {
    if (!mainParent) return;
    const rect = mainParent.getBoundingClientRect();
    size = { h: rect.height, w: rect.height * 0.78 };
    console.log(size);
  }
</script>

<svelte:window onresize={onResize} />
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div onmouseenter={onHover} bind:this={mainParent} class="main">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    onclick={() => {
      window.location.reload();
    }}
    class="btn"
  >
    Reload
  </div>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    onmouseenter={onHover}
    onclick={() => {
      localStorage.clear();
    }}
    class="btn-sm"
  >
    ⚡
  </div>
  <div
    class="container"
    style={`transform:translateX(-50%);height:${size.h}px;width:${size.w}px;`}
  >
    <img
      style={`left:-${value * size.w}px;`}
      src={onoff ? "online.png" : "offline.png"}
      class="img"
      alt="img"
    />
  </div>
</div>

<style>
  .btn {
    width: 100%;
    height: 100%;
  }
  .btn-sm {
    height: 40px;
  }
  .btn-sm,
  .btn {
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    border: solid 0px transparent;

    text-align: center;
    background: transparent;
  }
  .btn-sm:hover,
  .btn:hover {
    background: #18181822;
  }
  :global(:root) {
    padding: 0px;
    margin: 0px;
  }
  .main {
    display: flex;
    width: 100vw;
    height: 100vh;
  }
  .container {
    z-index: 99;
    left: 50%;
    pointer-events: none;
    user-select: none;
    bottom: 0px;
    position: absolute;
    width: 78px;
    height: 100px;
    overflow: hidden;
  }

  .img {
    position: absolute;
    height: 100%;
    object-fit: contain;
  }
</style>
