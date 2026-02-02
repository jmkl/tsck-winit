import gsap from "gsap";
import { useKeyboard } from "$lib/useKeyboard.svelte";
import { GetAppsState } from "$lib/AppState.svelte";

export interface NavigationConfig<T> {
  items: T[];
  rowIndex: number;
  gridItems: HTMLElement[];
  currentPage: number;
  totalPages: number;
  searchInputClass?: string;
  textureDialog?: HTMLDivElement | null;
}

export interface NavigationCallbacks<T> {
  onSelectedIndexChange?: (index: number) => void;
  onPrevPage?: (page: number) => void;
  onNextPage?: (page: number) => void;
  onEnter?: (item: T) => void;
  onSetFavorite?: (item: T) => void;
  onCtrlEnter?: () => void;
}

export function createNavigation<T>(
  getConfig: () => NavigationConfig<T>,
  callbacks: NavigationCallbacks<T> = {},
) {
  let selectedIndex = $state(-1);
  let lastNavigationTime = 0;
  const throttleMs = 50;
  const ctx = GetAppsState();
  let timeline = gsap.timeline({ defaults: { duration: 0.3 } });

  // Watch for selection changes
  $effect(() => {
    callbacks.onSelectedIndexChange?.(selectedIndex);
  });

  // Computed values
  function getMaxIndex() {
    return getConfig().items.length - 1;
  }

  // ============================================
  // UTILITY FUNCTIONS
  // ============================================
  function isModalOpen() {
    if (!getConfig().textureDialog) return false;
    const is_open = !getConfig().textureDialog?.classList.contains("hidden");
    return is_open;
  }

  function isSearchFocused() {
    const searchClass = getConfig().searchInputClass;
    if (!searchClass) return false;
    const active = document.activeElement as HTMLElement;
    return active?.classList.contains(searchClass);
  }

  function cycleModalButton(step: number) {
    const modal = getConfig().textureDialog;
    if (!modal) return;

    const focusableSelectors =
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])';
    const elements = Array.from(
      modal.querySelectorAll<HTMLElement>(focusableSelectors),
    ).filter((el) => !el.hasAttribute("disabled") && el.offsetParent !== null);

    if (elements.length === 0) return;

    const active = document.activeElement as HTMLElement | null;
    const index = elements.indexOf(active!);
    const next = elements[(index + step + elements.length) % elements.length];
    next?.focus();
    next?.scrollIntoView({ block: "nearest", behavior: "smooth" });
  }

  // ============================================
  // NAVIGATION FUNCTIONS
  // ============================================
  function scrollToSelected() {
    const config = getConfig();
    const selectedElement = config.gridItems[selectedIndex];
    if (!selectedElement) return;

    const gridContainer = selectedElement.closest(".scroll-container");
    if (gridContainer) {
      const gridRect = gridContainer.getBoundingClientRect();
      const itemRect = selectedElement.getBoundingClientRect();

      if (itemRect.top < gridRect.top || itemRect.bottom > gridRect.bottom) {
        const targetScrollTop =
          gridContainer.scrollTop +
          itemRect.top -
          gridRect.top -
          gridRect.height / config.rowIndex;

        gsap.to(gridContainer, {
          scrollTop: targetScrollTop,
          duration: 0.3,
          ease: "power1.out",
        });
      }
    }
  }

  function animateStamp(rotate = false) {
    const config = getConfig();
    const selectedElement = config.gridItems[selectedIndex];
    if (!selectedElement || !ctx.stamp) return;
    const itemRect = selectedElement.getBoundingClientRect();
    const stamp = ctx.stamp?.getBoundingClientRect();
    if (!itemRect || !stamp) return;
    if (timeline) timeline.clear();
    timeline
      .to(selectedElement, {
        opacity: 0,
        scale: 0,
        transformOrigin: "center center",
        rotate: rotate ? 90 : 0,
        duration: 0.1,
      })
      .to(selectedElement, {
        opacity: 1,
        scale: 1,
        rotate: 0,
        duration: 0.1,
      });
  }

  function throttleNavigation(action: () => void) {
    const now = Date.now();
    if (now - lastNavigationTime >= throttleMs) {
      lastNavigationTime = now;
      action();
    }
  }

  function nextPage() {
    const config = getConfig();
    if (config.currentPage < config.totalPages - 1) {
      const page = config.currentPage + 1;
      callbacks.onNextPage?.(page);
      window.scrollTo({ top: 0, behavior: "smooth" });
    }
  }

  function prevPage() {
    const config = getConfig();
    if (config.currentPage > 0) {
      const page = config.currentPage - 1;
      callbacks.onPrevPage?.(page);
      window.scrollTo({ top: 0, behavior: "auto" });
    }
  }

  function nextImage() {
    const maxIndex = getMaxIndex();
    if (selectedIndex < maxIndex) {
      selectedIndex++;
      scrollToSelected();
    }
  }

  function prevImage() {
    if (selectedIndex > 0) {
      selectedIndex--;
      scrollToSelected();
    }
  }

  function moveDown() {
    const config = getConfig();
    const maxIndex = getMaxIndex();
    const newIndex = Math.min(selectedIndex + config.rowIndex, maxIndex);
    selectedIndex = newIndex;
    scrollToSelected();
  }

  function moveUp() {
    const config = getConfig();
    const newIndex = Math.max(selectedIndex - config.rowIndex, 0);
    selectedIndex = newIndex;
    scrollToSelected();
  }

  function resetSelect() {
    if (isModalOpen()) {
      getConfig().textureDialog?.classList.add("hidden");
    }
    selectedIndex = -1;
  }

  function selectIndex(index: number) {
    const maxIndex = getMaxIndex();
    if (index >= 0 && index <= maxIndex) {
      selectedIndex = index;
      scrollToSelected();
    }
  }

  function handleSubmit() {
    const config = getConfig();
    if (selectedIndex >= 0 && selectedIndex < config.items.length) {
      const item = config.items[selectedIndex];
      callbacks.onEnter?.(item);
      return item;
    }
    return null;
  }

  function handleCtrlEnter() {
    callbacks.onCtrlEnter?.();
  }

  // ============================================
  // KEYBOARD SETUP
  // ============================================
  const searchClass = getConfig().searchInputClass;

  useKeyboard([
    {
      key: "Enter",
      ctrl: true,
      onlyInClass: searchClass,
      handler: handleCtrlEnter,
      priority: 10,
    },
    {
      key: "ArrowLeft",
      alt: true,
      handler: prevPage,
      priority: 20,
    },
    {
      key: "ArrowRight",
      alt: true,
      handler: nextPage,
      priority: 20,
    },
    {
      key: "Enter",
      handler: (e) => {
        if (isModalOpen()) {
          if (e.target instanceof HTMLButtonElement) {
            e.target.click();
            return;
          }
        }

        if (!isModalOpen() && selectedIndex >= 0) {
          animateStamp();
          handleSubmit();
        }
      },
      priority: 15,
    },
    {
      key: "f",
      handler: (e) => {
        const config = getConfig();
        if (selectedIndex >= 0 && selectedIndex < config.items.length) {
          const item = config.items[selectedIndex];
          if (callbacks.onSetFavorite) {
            callbacks.onSetFavorite(item);
            animateStamp(true);
          }
          return item;
        }
      },
      when: () => !isModalOpen() && !isSearchFocused(),
      priority: 15,
    },
    {
      key: "ArrowRight",
      handler: () => cycleModalButton(+1),
      when: isModalOpen,
      priority: 15,
    },
    {
      key: "ArrowLeft",
      handler: () => cycleModalButton(-1),
      when: isModalOpen,
      priority: 15,
    },

    // Tab to toggle modal
    {
      key: "Tab",
      handler: () => {
        const modal = getConfig().textureDialog;
        if (!modal) return;
        console.log("TAB");
        if (modal.classList.contains("hidden"))
          modal.classList.remove("hidden");
        // else modal.classList.add('hidden');
      },
      when: () => getConfig().textureDialog != null,
      priority: 10,
    },
    {
      key: "Tab",
      handler: () => {
        const searchClass = getConfig().searchInputClass;
        if (searchClass) {
          const searchInput = document.querySelector<HTMLInputElement>(
            `.${searchClass}`,
          );
          searchInput?.focus();
        }
      },
      when: () => !isSearchFocused() && !getConfig().textureDialog,
      priority: 10,
    },
    {
      key: "Escape",
      onlyInClass: searchClass,
      handler: () => {
        const searchClass = getConfig().searchInputClass;
        if (searchClass) {
          const searchInput = document.querySelector<HTMLInputElement>(
            `.${searchClass}`,
          );
          if (searchInput && document.activeElement === searchInput) {
            searchInput.blur();
          }
        }
      },
      priority: 10,
    },
    {
      key: "Escape",
      handler: () => throttleNavigation(resetSelect),
      priority: 5,
    },
    {
      key: "ArrowRight",
      handler: () => throttleNavigation(nextImage),
      when: () => !isSearchFocused() && !isModalOpen(),
      priority: 5,
    },
    {
      key: "ArrowLeft",
      handler: () => throttleNavigation(prevImage),
      when: () => !isSearchFocused() && !isModalOpen(),
      priority: 5,
    },
    {
      key: "ArrowDown",
      handler: () => throttleNavigation(moveDown),
      when: () => !isSearchFocused() && !isModalOpen(),
      priority: 5,
    },
    {
      key: "ArrowUp",
      handler: () => throttleNavigation(moveUp),
      when: () => !isSearchFocused() && !isModalOpen(),
      priority: 5,
    },
  ]);

  return {
    get selectedIndex() {
      return selectedIndex;
    },
    set selectedIndex(value: number) {
      selectedIndex = value;
    },
    get maxIndex() {
      return getMaxIndex();
    },
    nextImage: () => throttleNavigation(nextImage),
    prevImage: () => throttleNavigation(prevImage),
    moveDown: () => throttleNavigation(moveDown),
    moveUp: () => throttleNavigation(moveUp),
    nextPage,
    prevPage,
    resetSelect: () => throttleNavigation(resetSelect),
    selectIndex,
    handleSubmit,
    handleCtrlEnter,
  };
}
