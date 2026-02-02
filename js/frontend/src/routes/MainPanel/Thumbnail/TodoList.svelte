<script lang="ts">
  import { type TemplateLine } from "./stringUtils";
  import { extractUrls } from "./stringUtils";
  import { useKeyboard } from "$lib/useKeyboard.svelte";
  import gsap from "gsap";
  import { onDestroy, onMount, untrack } from "svelte";
  import AddTodo from "$lib/components/Icon/AddTodo.svelte";
  import Recycle from "$lib/components/Icon/Recycle.svelte";
  import SmallCheckMark from "$lib/components/Icon/Small-CheckMark.svelte";
  import IconButton from "$lib/components/IconButton.svelte";
  import { breakWord } from "./stringUtils";
  import { cicleRange } from "./stringUtils";
  import Apply from "$lib/components/Icon/Apply.svelte";
  import CheckMark from "$lib/components/Icon/CheckMark.svelte";
  import DoubleCheckMark from "$lib/components/Icon/DoubleCheckMark.svelte";
  import Close from "$lib/components/Icon/Close.svelte";
  import Save from "$lib/components/Icon/Save.svelte";
  import { bubblePop } from "$lib/animation";
  import { GetAppsState } from "$lib/AppState.svelte";
  import { TodoStatus, type TodoType } from "./TodoHelper";
  import Reload from "$lib/components/Icon/Reload.svelte";
  const ctx = GetAppsState();
  let menuEl: HTMLDivElement | undefined = $state();
  let cursor: HTMLDivElement | undefined = $state();
  let cursorY = 0;
  let todoContent: HTMLElement[] = $state([]);
  let filteredTodo = $derived(ctx.todoList.filter((t) => t.status < 3));
  let todolistContainer: HTMLDivElement | undefined = $state();
  let { appendTodo, focusMe = $bindable() } = $props();
  let timeline = gsap.timeline();
  let clipboardTexts = $state([""]);
  let clipboarHelperDialog = $state(false);
  function setTodoItem(node: HTMLElement, index: number) {
    if (todoContent[index] !== node) {
      todoContent[index] = node;
    }
  }
  onDestroy(() => {});

  let activeTodo = $state(-1);
  useKeyboard([
    {
      key: "Escape",
      onlyInClass: "todolist",
      handler: (e) => {
        todolistContainer?.blur();
        focusMe = false;
      },
    },
    {
      key: "ArrowUp",
      onlyInClass: "todolist",
      handler: (e) => {
        activeTodo = cicleRange(activeTodo - 1, 0, filteredTodo.length - 1);
        todoContent[activeTodo]?.scrollIntoView({
          behavior: "smooth",
          block: "nearest",
        });
      },
    },
    {
      key: "ArrowDown",
      onlyInClass: "todolist",
      handler: (e) => {
        activeTodo = cicleRange(activeTodo + 1, 0, filteredTodo.length - 1);

        todoContent[activeTodo]?.scrollIntoView({
          behavior: "smooth",
          block: "nearest",
        });
      },
    },
    {
      key: "ArrowLeft",
      onlyInClass: "todolist",
      handler: (e) => {
        filteredTodo[activeTodo].status = 0;
        const todo = filteredTodo[activeTodo];
        ctx.todoHelper?.updateStatus(todo.status, todo.messageId);
      },
    },
    {
      key: "ArrowRight",
      onlyInClass: "todolist",
      handler: (e) => {
        filteredTodo[activeTodo].status = 1;
        const todo = filteredTodo[activeTodo];
        ctx.todoHelper?.updateStatus(todo.status, todo.messageId);
      },
    },
    {
      key: "Enter",
      onlyInClass: "todolist",
      handler: (e) => {
        pushTodo(filteredTodo[activeTodo]);
      },
      priority: 30,
      useCapture: true,
    },
  ]);

  function pushTodo(todo: TodoType) {
    if (todo.status > 0) return;
    const todos: TemplateLine[] = breakWord(extractUrls(todo.message).text)
      .split("\n")
      .filter((td) => td != "")
      .map((td, index) => {
        return {
          id: index,
          text: td,
          scale: 0,
          italic: false,
          include: true,
        };
      });
    if (appendTodo) appendTodo(todos);
  }
  function addTodo() {
    // let content = Array.from({ length: 10 }, (_, i) => 'HELLO FROM TODO APPS ' + i);
    // invokePayload<FrontEndEvent>({ type: 'AddTodos', value: content });
  }
  onMount(() => {
    addTodo();
    // invokePayloadWithCallback<FrontEndEvent, Array<TodoType>>(
    // 	{ type: 'FetchTodos' },
    // 	(error, result) => {
    // 		if (!error && result != null) {
    // 			Logger.warn('Chat counter', result.length);
    // 		}
    // 	}
    // );
    if (menuEl) {
      gsap.set(menuEl, {
        scale: 0,
      });
    }
  });
  $effect(() => {
    if (focusMe) todolistContainer?.focus();
  });

  function animateContextMenu(show: boolean, e: MouseEvent) {
    let rect;
    if (e.currentTarget == window) {
      rect = { top: 0, left: 0 };
    } else {
      rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    }
    let x = e.clientX - rect.left,
      y = e.clientY - rect.top;
    if (menuEl) {
      timeline.clear();
      if (!show) {
        timeline.to(menuEl, {
          scale: 0,
          opacity: 0,
          duration: 0.1,
        });
      } else {
        timeline.fromTo(
          menuEl,
          {
            duration: 0.2,
            ease: "power1.in",
            transformOrigin: "top left",
            x: x,
            y: y,
            scale: 0,
            opacity: 0,
          },
          {
            duration: 0.2,
            ease: "power1.in",
            transformOrigin: "top left",
            x: x,
            y: y,
            scale: 1,
            opacity: 1,
          },
        );
      }
    }
  }
</script>

<svelte:window onblur={(e: any) => animateContextMenu(false, e)} />
{#snippet renderMessage(text: string)}
  {@const msg = extractUrls(text)}
  {msg.text}
  {#each msg.urls as url}
    <a
      tabindex="-1"
      onclick={(e) => {
        e.stopPropagation();
        e.preventDefault();
        window.open(url);
        //ctx.loadYoutubeThumbnailFetcherPage(url);
      }}
      class="btn btn-soft btn-xs btn-error absolute top-px left-px p-0.5 text-[8px]"
      href={url}
    >
      link
    </a>
  {/each}
{/snippet}
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  bind:this={menuEl}
  class="bg-base-300 absolute z-1000 flex w-auto flex-col gap-2 rounded-md p-1 shadow-[0_2px_2px_black]"
>
  <button
    tabindex="-1"
    class="btn btn-xs btn-ghost btn-error justify-end text-[10px] *:size-2.5"
    onclick={async (e) => {
      location.reload();
    }}
  >
    Reload
    <Reload />
  </button>
  <button
    tabindex="-1"
    class="btn btn-xs btn-ghost btn-error justify-end text-[10px] *:size-2.5"
    onclick={async (e) => {
      animateContextMenu(false, e);
      ctx.todoHelper?.deleteAll().then((result) => {
        ctx.todoUpdate();
      });
    }}
  >
    Delete All
    <Recycle />
  </button>
  <button
    tabindex="-1"
    class="btn btn-xs btn-ghost btn-error justify-end text-[10px] *:size-2.5"
    onclick={async (e) => {
      animateContextMenu(false, e);

      clipboardTexts = (await navigator.clipboard.readText())
        .split(/\r?\n/)
        .filter((t) => t.trim() !== "");
      clipboarHelperDialog = true;
    }}
  >
    Paste Clipboard
    <AddTodo />
  </button>
</div>

{#if clipboarHelperDialog}
  <div
    transition:bubblePop={{}}
    class="sticky z-999 flex h-45 w-full items-center justify-center p-4"
  >
    <div
      class="bg-base-200 font-jetbrains relative flex h-full w-full flex-col overflow-y-auto rounded-md p-4 text-[8px] shadow-[0_0px_0px_2px_var(--color-error),0_5px_10px_0px_rgba(0,0,0,.1)] select-auto"
    >
      <div class="absolute top-2 right-4">
        <IconButton
          class="btn-square btn-soft btn-error  h-6! w-6! p-1!"
          onclick={() => {
            ctx.todoHelper
              ?.addTodoFromClipboard(clipboardTexts.join("\n"))
              .then(() => {
                ctx.todoUpdate();
                clipboarHelperDialog = false;
              });
          }}
          icon={Save}
        />
        <IconButton
          icon={Close}
          onclick={() => {
            clipboarHelperDialog = false;
          }}
          class="btn-square  btn-soft btn-error  h-6!  w-6! p-1!"
        />
      </div>
      {#each clipboardTexts as clipboard, index}
        <div class="flex w-full flex-col">
          <div class="odd:bg-base-300 rounded-sm px-1 py-1">
            {clipboard}
          </div>
          <button
            title="something"
            onclick={(e) => {
              console.log(index, index + 1);
              if (index >= 0 && index + 1 < clipboardTexts.length) {
                let prev = clipboardTexts[index];
                let next = clipboardTexts[index + 1];
                clipboardTexts[index] = prev + " " + next;
                console.log(prev);
                clipboardTexts.splice(index + 1, 1);
              }
            }}
            class="separator hover:bg-error pointer-events-auto h-1.25 w-full cursor-n-resize text-transparent"
          >
            mush
          </button>
        </div>
      {/each}
    </div>
  </div>
{/if}

<svelte:document />
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  onclick={(e) => {
    animateContextMenu(false, e);
  }}
  oncontextmenu={(e) => {
    e.preventDefault();
    animateContextMenu(true, e);
  }}
  bind:this={todolistContainer}
  tabindex="-2"
  class="todolist border-error flex h-full min-h-0 w-full flex-col overflow-x-hidden overflow-y-auto p-0 [overflow-anchor:none] focus:outline-0"
>
  {#if ctx.todoList}
    {#each filteredTodo as todo, index}
      <div
        use:setTodoItem={index}
        class="flex h-auto {activeTodo == index
          ? ''
          : ''} group border-error odd:bg-base-200 relative w-full flex-row items-center transition-all"
      >
        <div
          bind:this={cursor}
          class="absolute left-0 h-full {activeTodo == index && focusMe
            ? 'w-1'
            : 'w-0'} bg-error transition-all duration-75"
        ></div>

        {#if todo.status == TodoStatus.Done}
          <SmallCheckMark size={14} class="text-error absolute left-1" />
        {/if}
        <a
          tabindex="-1"
          href="/"
          class="todo-text relative min-h-5.5 cursor-pointer text-left transition-all select-none {todo.status ==
          1
            ? 'mask-[linear-gradient(to_bottom,#ffffff22_10%,transparent)] px-4 py-1 pl-6 italic line-through'
            : ' mask-[linear-gradient(to_bottom,black_60%,transparent)] px-4 py-2'} group-hover:text-neutral-content line-clamp-1 flex-1 wrap-break-word"
        >
          {@render renderMessage(todo.message)}
        </a>

        <!-- <SwipeRemove
          status={todo.status}
          onDelete={() => {}}
          class="todo-text text-left select-none cursor-grabbing  p-4 mask-[linear-gradient(to_bottom,black_60%,transparent)]'} group-hover:text-neutral-content line-clamp-1 flex-1 wrap-break-word"
        >
          {@render renderMessage(todo.message)}
        </SwipeRemove> -->
        <div
          class="bg-base-300 absolute right-1 flex h-full flex-row items-center justify-center gap-1 opacity-0 backdrop-blur-sm transition-opacity duration-500 ease-in-out group-hover:opacity-100"
        >
          <IconButton
            icon={todo.status == 0 ? DoubleCheckMark : CheckMark}
            onclick={() => {
              todo.status = todo.status == 0 ? 1 : 0;
              ctx.todoHelper?.updateStatus(todo.status, todo.messageId);
            }}
            class="text-error btn-error h-4! w-4! p-0.5!"
          />

          <IconButton
            icon={Apply}
            onclick={() => {
              pushTodo(todo);
            }}
            class="text-error btn-error h-4! w-4! p-0.5!"
          />
          <IconButton
            onclick={() => {
              todo.status = TodoStatus.Deleted;
              ctx.todoHelper?.updateStatus(todo.status, todo.messageId);
            }}
            class="text-error btn-error h-4! w-4! p-0.5!"
            icon={Recycle}
          />
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .todo-text {
    font-family: "JetBrains Mono";
    font-size: 9px;
    font-weight: 200;
    line-height: 140%;
    text-transform: uppercase;
  }
</style>
