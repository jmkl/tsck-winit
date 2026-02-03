<script lang="ts">
  import { GetAppsState } from "$lib/AppState.svelte";
  import ImageGrid from "$lib/components/ImageGrid/ImageGrid.svelte";
  import { onMount } from "svelte";
  import { createNavigation } from "$lib/components/ImageGrid/setupNavigation.svelte";
  import { useKeyboard } from "$lib/useKeyboard.svelte";
  import type { TextureItem, TexturePaginationItem } from "$lib/texture.type";
  import { invokePayload, invokePayloadWithCallback } from "$lib";
  import { type UserEvent } from "@tsck/lib";
  type Category = {
    category: string;
    key: string;
    index: number;
  };
  const ctx = GetAppsState();
  let textureDialog: HTMLDivElement | undefined = $state();
  let currentCategory = $state("Favorite");
  let gridItems = $state.raw<HTMLElement[]>([]);
  let currentPage: number = $state(0);
  let totalPages: number = $state(0);
  const searchInputClass = "image-search";
  let pageFiles: TextureItem[] = $state([]);
  let rowIndex = $state(4);
  let categories: Category[] = $state([]);

  const navigation = createNavigation<TextureItem>(
    () => ({
      items: pageFiles,
      rowIndex: rowIndex,
      gridItems: gridItems,
      currentPage: currentPage,
      totalPages: totalPages,
      searchInputClass: searchInputClass,
      textureDialog: textureDialog,
    }),
    {
      onPrevPage: (page) => {
        fetchTextures(currentCategory, page);
      },
      onNextPage: (page) => {
        fetchTextures(currentCategory, page);
      },
      onEnter: (item) => {
        appendSelection(item);
      },

      onSetFavorite: (item) => {
        if (item.id) {
          invokePayload<UserEvent>({
            type: "UpdateTextureFavorite",
            value: [item.id, true],
          });
        }
      },
    },
  );
  function appendSelection(item: TextureItem) {
    let payload: UserEvent = {
      type: "FunctionCall",
      value: {
        func: "appendLinkedObject",
        args: [["texture", item.category, item.name].join("|")],
      },
    };
    invokePayload(payload);
  }
  function fetchTextures(query: string = "", page = 0) {
    if (!textureDialog?.classList.contains("hidden"))
      textureDialog?.classList.add("hidden");
    invokePayloadWithCallback<UserEvent, TexturePaginationItem>(
      {
        type: "FetchTextures",
        value: [query, page, 32],
      },
      (error, result) => {
        if (error || !result) return;
        console.log(result);
        currentPage = result.page;
        totalPages = result.total_pages;
        pageFiles = result.items.map((f) => {
          return {
            id: f.id,
            name: f.filename,
            thumb: f.thumbnail,
            category: f.category,
          };
        });
        updatePage();
      },
    );
  }
  let buttonsHandler: HTMLDivElement | undefined = $state();
  function registerListener() {
    const handler = (key: string, index: number) => {
      return {
        key: key,
        ctrl: false,
        shift: false,
        alt: false,
        priority: 15,
        handler: () => {
          if (buttonsHandler) {
            let button = buttonsHandler.children[index] as HTMLButtonElement;
            if (button) {
              button.click();
            }
          }
        },
      };
    };
    const keymap = categories.map((k) => handler(k.key, k.index));
    useKeyboard(keymap);
  }
  $effect(() => {
    registerListener();
  });
  onMount(() => {
    const chars = "abcdefghijklmnopqrstuvwxyz0123456789";
    invokePayloadWithCallback<UserEvent, Array<string>>(
      { type: "FetchTextureCategories" },
      (error, result) => {
        if (error || !result) return;
        const r = ["Favorite", ...result];
        categories = r.map((c, i) => ({
          category: c,
          key: chars[i],
          index: i,
        }));

        updatePage();
      },
    );
    fetchTextures(currentCategory);
  });
  function updatePage() {
    ctx.Pages.page = currentPage;
    ctx.Pages.totalPages = totalPages;
    ctx.Pages.imageCount = pageFiles.length;
  }
</script>

<div
  bind:this={textureDialog}
  class="texture-category-dialog absolute z-99999 flex justify-center items-center h-full w-full bg-base-300"
>
  <div tabindex="-1" class="flex flex-col justify-center p-2 -translate-y-5">
    <div
      tabindex="-1"
      bind:this={buttonsHandler}
      class="flex flex-row flex-wrap h-full justify-center items-center gap-1 font-avantt"
    >
      {#each categories as cat, index (cat)}
        <button
          onclick={() => {
            currentCategory = cat.category;
            fetchTextures(cat.category);
          }}
          class="rounded-sm bg-error/20 p-1"
        >
          <div class="flex w-full flex-row gap-2 text-[12px] px-1">
            <span
              class="rounded-xs bg-error/50 p-0 px-1 font-black text-error-content"
            >
              {cat.key.toUpperCase()}
            </span>
            <span class="p-0"
              >{cat.category.replace("phtos-", "P:").toUpperCase()}</span
            >
          </div>
        </button>
      {/each}
    </div>
  </div>
</div>

<div
  tabindex="-1"
  class="flex h-full w-full flex-col items-center overflow-hidden p-2"
>
  <ImageGrid
    serverUrl={ctx.httpServerStaticUrl + "/texture"}
    appendSelected={appendSelection}
    files={pageFiles}
    selectedIndex={navigation.selectedIndex}
    bind:gridItems
    allowFavorite={(file) => {
      if (file.id)
        invokePayload<UserEvent>({
          type: "UpdateTextureFavorite",
          value: [file.id, true],
        });
    }}
  />
</div>
