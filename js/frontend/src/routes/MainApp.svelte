<script lang="ts">
  import TitleBar from "$lib/components/TitleBar.svelte";
  import MainPanel from "./MainPanel/+page.svelte";
  import { GetAppsState, HERO_PAGE, SetAppsState } from "$lib/AppState.svelte";
  import CurrentPage from "$lib/components/ImageGrid/CurrentPage.svelte";
  import IconButton from "$lib/components/IconButton.svelte";
  import Setting from "$lib/components/Icon/Setting.svelte";
  import Command from "$lib/components/Icon/Command.svelte";
  import EyeOpen from "$lib/components/Icon/EyeOpen.svelte";
  import EyeClose from "$lib/components/Icon/EyeClose.svelte";

  SetAppsState();
  const ctx = GetAppsState();
</script>

<div
  class="border-base-300 bg-base-300 flex h-screen w-screen flex-col overflow-hidden rounded-md border"
>
  <div class="flex h-auto w-full shrink-0 flex-row">
    <div class="flex flex-row items-center gap-0 pl-2">
      <IconButton
        class="btn-ghost! btn-error"
        onclick={() => {
          ctx.globalActivePage = HERO_PAGE.SETTINGUI;
        }}
        icon={Setting}
      />
      <IconButton
        class="btn-ghost! btn-error"
        onclick={() => {
          ctx.globalActivePage =
            ctx.globalActivePage == HERO_PAGE.COMMANDLOG
              ? HERO_PAGE.THUMBNAIL
              : HERO_PAGE.COMMANDLOG;
        }}
        icon={Command}
      />
      {#if [HERO_PAGE.SMARTOBJECT, HERO_PAGE.TEXTURES].includes(ctx.globalActivePage)}
        <CurrentPage
          page={ctx.Pages.page}
          totalPages={ctx.Pages.totalPages}
          imageCount={ctx.Pages.imageCount}
        ></CurrentPage>
      {/if}
    </div>
    <div data-tsck-drag-region class="panel flex-1"></div>
    <IconButton
      class="btn-ghost! btn-error"
      icon={ctx.WindowLevel === "Top" ? EyeOpen : EyeClose}
    />
    <TitleBar />
  </div>
  {#if !ctx.CompactMode}
    <div tabindex="-1" class="w-full flex-1 overflow-hidden p-0.5">
      <MainPanel />
    </div>
  {/if}
</div>
