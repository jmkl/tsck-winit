<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import SearchInput from "./SearchInput.svelte";
  import { GetAppsState } from "$lib/AppState.svelte";
  import type {
    EventPayload,
    PaginationItems,
    SmartObjectItem,
    UserEvent,
  } from "@tsck/lib";
  import { createNavigation } from "$lib/components/ImageGrid/setupNavigation.svelte";
  import { invokePayload, invokePayloadWithCallback, listen } from "$lib";
  import ImageGrid from "$lib/components/ImageGrid/ImageGrid.svelte";

  const ctx = GetAppsState();
  const DEBOUNCE_MS = 100;
  let debounceTimeOut: NodeJS.Timeout | undefined = undefined;
  let searchQuery = $state("");
  let searchInput = $state<HTMLInputElement>();
  let searchInputFocus = $state<boolean>(false);
  let gridItems = $state.raw<HTMLElement[]>([]);
  let listener: (() => void) | undefined = $state();
  let dialogModal: HTMLDialogElement | undefined = $state();
  let deletedFile: SmartObjectItem | undefined = $state();
  let currentPage: number = $state(0);
  let totalPages: number = $state(0);
  const searchInputClass = "image-search";
  let pageFiles: SmartObjectItem[] = $state([]);
  let isSearching = $derived(searchQuery !== "");
  let rowIndex = $state(4);
  let imagePerPage = $derived(32);

  const navigation = createNavigation<SmartObjectItem>(
    () => ({
      items: pageFiles,
      rowIndex: rowIndex,
      gridItems: gridItems,
      currentPage: currentPage,
      totalPages: totalPages,
      searchInputClass: searchInputClass,
    }),
    {
      onPrevPage: (page) => {
        filterSmartObject(searchQuery, page);
      },
      onNextPage: (page) => {
        filterSmartObject(searchQuery, page);
      },
      onEnter: (item) => {
        appendSelection(item);
      },
      onCtrlEnter: () => {
        convertSelectedLayerToSmartObject();
      },
    },
  );

  function filterSmartObject(query: string = "", page = 0) {
    invokePayloadWithCallback<UserEvent, PaginationItems>(
      {
        type: "FilterSmartObjectChunk",
        value: { query, page, per_page: imagePerPage },
      },
      (error, result) => {
        if (error) return;
        if (!result) return;
        currentPage = result.page;
        totalPages = result.total_page;
        pageFiles = result.current_items;
      },
    );
  }

  $effect(() => {
    searchQuery;
    if (debounceTimeOut) clearTimeout(debounceTimeOut);
    debounceTimeOut = setTimeout(() => {
      filterSmartObject(searchQuery);
      clearTimeout(debounceTimeOut);
      isSearching = false;
    }, DEBOUNCE_MS);
  });

  function appendSelection(file: SmartObjectItem) {
    let payload: UserEvent = {
      type: "FunctionCall",
      value: {
        func: "appendLinkedObject",
        args: ["smartobject|" + file.name],
      },
    };
    invokePayload(payload);
  }

  function convertSelectedLayerToSmartObject() {
    ctx.showLoadingPanel(true);
    let payload: UserEvent = {
      type: "FunctionCall",
      value: {
        func: "layerToSmartObject",
        args: [searchQuery],
      },
    };
    invokePayload<UserEvent>(payload);
  }

  function deleteSmartObject(file: SmartObjectItem) {
    let payload: UserEvent = {
      type: "SmartObjectDelete",
      value: file,
    };
    invokePayloadWithCallback<UserEvent>(payload, (error, result) => {
      if (!error && result) {
        filterSmartObject(searchQuery);
      }
    });
  }

  function onCtrlEnterOnInput() {
    navigation.handleCtrlEnter();
  }

  onMount(() => {
    filterSmartObject();

    listener = listen<EventPayload, UserEvent>(
      "tsck::event|EVENTPAYLOAD::FRONTEND",
      (event: UserEvent) => {
        if (!event) return;

        switch (event.type) {
          case "SmartobjectThumbnailUpdate":
            filterSmartObject(searchQuery);
            break;
        }
      },
    );
  });

  onDestroy(() => {
    if (listener) listener();
    if (debounceTimeOut) clearTimeout(debounceTimeOut);
  });

  function handleSearchMount(element: HTMLInputElement) {
    searchInput = element;
  }

  function handleDeleteClick(file: SmartObjectItem) {
    deletedFile = file;
    dialogModal?.showModal();
  }

  function handleDeleteConfirm() {
    if (!deletedFile) return;
    deleteSmartObject(deletedFile);
    deletedFile = undefined;
  }

  function handleDeleteCancel() {
    deletedFile = undefined;
  }
  function updatePage() {
    console.log("update Page");
    ctx.Pages.page = currentPage;
    ctx.Pages.totalPages = totalPages;
    ctx.Pages.imageCount = pageFiles.length;
  }
  $effect(() => {
    updatePage();
  });
</script>

<dialog bind:this={dialogModal} class="modal">
  <div class="modal-box">
    <h3 class="text-lg font-bold">Deleting Thumbnail!</h3>
    <p class="py-4">
      Are you sure you want to delete
      <span class="font-bold text-error italic">
        {deletedFile?.name}
      </span>
      file?
    </p>
    <div class="modal-action">
      <form method="dialog">
        <button onclick={handleDeleteConfirm} class="btn btn-soft btn-error">
          Delete
        </button>
        <button onclick={handleDeleteCancel} class="btn btn-soft">
          Cancel
        </button>
      </form>
    </div>
  </div>
</dialog>

<div class="flex h-full w-full flex-col items-center overflow-hidden p-2">
  <div class={`flex w-full bg-base-300 transition-opacity`}>
    <SearchInput
      onFocusChange={(focus) => {
        searchInputFocus = focus;
      }}
      bind:value={searchQuery}
      {isSearching}
      onSave={onCtrlEnterOnInput}
      onInput={(val) => (searchQuery = val)}
      onMount={handleSearchMount}
    />
  </div>

  <ImageGrid
    serverUrl={ctx.httpServerStaticUrl + "/smartobject"}
    appendSelected={appendSelection}
    files={pageFiles}
    selectedIndex={navigation.selectedIndex}
    deleteSelected={handleDeleteClick}
    bind:gridItems
  />
</div>
