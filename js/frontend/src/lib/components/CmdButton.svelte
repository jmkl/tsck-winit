<script lang="ts">
  import Play from "./Icon/Play.svelte";
  import Stop from "./Icon/Stop.svelte";

  let { class: className = "", cli = $bindable(), onPlay, onClick } = $props();
</script>

<a
  tabindex="-1"
  href="/"
  onclick={(e) => {
    e.preventDefault();
    if (onClick) onClick(e);
  }}
  class="rounded-md flex-1 {className} text-[10px] cursor-pointer select-none group flex {cli.pid ==
  0
    ? 'hover:bg-error border-error border'
    : 'hover:bg-success border-success border'} font-avantt pl-2 bg-base-200"
>
  <div class="flex items-center w-full flex-row gap-2">
    <span
      class="{cli.pid == 0
        ? 'text-error'
        : 'text-success'} font-black group-hover:text-base-300 uppercase"
    >
      {cli.name}
    </span>
    <span
      class="group-hover:text-base-300 {cli.pid == 0
        ? 'text-error'
        : 'text-white'}"
    >
      {cli.pid}
    </span>
    <div class="flex-1"></div>
    <span
      tabindex="-1"
      role="button"
      onkeydown={() => {}}
      onclick={(e) => {
        e.preventDefault();
        e.stopImmediatePropagation();
        if (onPlay) onPlay();
      }}
      class="*:block active:bg-warning hover:bg-error hover:text-error-content p-1.5 group-hover:bg-base-300 bg-base-300 rounded-md {cli.pid ==
      0
        ? 'text-error'
        : 'text-success'}  *:h-2.5 *:stroke-3"
    >
      {#if cli.pid}
        <Stop />
      {:else}
        <Play />
      {/if}
    </span>
  </div>
</a>
