<script lang="ts">
  import { onMount } from "svelte";
  import {
    cicleRange,
    ScaleRange,
    setCaretPos,
    type TemplateLine,
  } from "./stringUtils";
  import { fade } from "svelte/transition";

  type Props = {
    texts: TemplateLine[];
    onChange?: (value: TemplateLine[]) => void;
    showEditor?: boolean;
    forceFocus?: boolean;
    registerFocus?: () => void;
  };

  let {
    texts,
    onChange,
    showEditor = $bindable(),
    forceFocus = $bindable(),
    registerFocus,
  }: Props = $props();
  let editMode = $state(showEditor);
  // svelte-ignore state_referenced_locally
  let tempText = $state(texts);

  let temptEdittext: HTMLDivElement | undefined = $state();

  $effect(() => {
    if (!editMode) {
      tempText = texts;
    }
  });

  $effect(() => {
    if (forceFocus) {
      editMode = true;
      temptEdittext?.focus();
    } else {
      temptEdittext?.blur();
      forceFocus = false;
    }
  });

  function getCaretPos(row: number, column: number | undefined) {
    let col = column ? column : 0;
    return col;
  }
  onMount(() => {
    if (forceFocus) {
      temptEdittext?.focus();
    } else {
      editMode = false;
    }
  });
</script>

<svelte:window
  onmouseout={(e) => {
    if (!e.relatedTarget && editMode) {
      temptEdittext?.blur();
    }
  }}
/>
<div class="relative flex h-auto w-full shrink-0 flex-col overflow-x-hidden">
  <!-- DISPLAY MODE - WITH BUTTON AND STUFF -->
  <div class="the-container">
    {#if !editMode}
      <div class="flex w-full flex-col">
        {#each texts as text, index}
          <div
            transition:fade={{ duration: 100 }}
            class="flex {text.include
              ? ''
              : 'italic opacity-20'}  border-error/10 w-full flex-row items-center gap-1 border-b px-2"
          >
            <a
              tabindex="-1"
              onblur={() => {}}
              href="/"
              oncontextmenu={(e) => {
                e.preventDefault();
                text.include = !text.include;
                if (onChange) {
                  onChange(texts);
                }
              }}
              ondblclick={(e) => {
                const x = e.clientX;
                const y = e.clientY;

                e.preventDefault();

                editMode = !editMode;
                setTimeout(() => {
                  const cursor =
                    document.caretPositionFromPoint(x, y)?.offset ?? 0;
                  if (temptEdittext) setCaretPos(temptEdittext, cursor);
                }, 100);
              }}
              class="ml-text hover:text-error line-clamp-1 flex-1"
            >
              {text.text}
            </a>
            <button
              tabindex="-1"
              onclick={() => {
                text.italic = !text.italic;
              }}
              class="{text.italic
                ? 'italic'
                : 'normal'} btn btn-soft btn-error h-3.75 w-3.75 rounded-xs p-0 text-[10px]"
            >
              I
            </button>
            <button
              tabindex="-1"
              onclick={(e) => {
                let denominator = e.shiftKey ? -1 : 1;

                text.scale = cicleRange(
                  text.scale + denominator,
                  0,
                  ScaleRange.length - 1,
                );
              }}
              class=" btn btn-soft btn-error h-3.75 w-6.25 rounded-xs p-0 text-[10px]"
            >
              {text.scale}
            </button>
          </div>
        {/each}
      </div>
      <!-- EDIT MODE -->
    {:else}
      <div class="container flex w-full flex-col px-2 whitespace-pre">
        <div
          bind:this={temptEdittext}
          role="cell"
          tabindex="-1"
          oninput={(e) => {
            const target = e.target as HTMLDivElement;
            let hide_all = false;
            texts = target.innerText
              .trim()
              .split("\n")
              .filter((s) => s !== "")
              .map((t, index) => {
                if (t.includes("$")) hide_all = true;
                const scale = texts[index]?.scale ?? 0;
                const italic = texts[index]?.italic ?? false;
                let include = !hide_all;
                if (texts.length > index) {
                  return {
                    id: index,
                    text: t,
                    scale,
                    italic,
                    include,
                  };
                } else {
                  return {
                    id: index,
                    text: t,
                    scale,
                    italic,
                    include,
                  };
                }
              });
            if (onChange) onChange(texts);
          }}
          onblur={() => {
            editMode = false;
            tempText = texts;
            if (onChange) {
              onChange(texts);
            }
          }}
          spellcheck={false}
          contenteditable={true}
          class="ml-text border-0 leading-0 ring-0 outline-0"
        >
          {tempText.map((t) => t.text.trim()).join("\n")}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .ml-text {
    padding: 10px 0px;
    margin: 0;
    font-family: "JetBrains Mono";
    font-size: 14px;
    font-weight: 400;
    line-height: 120%;
    letter-spacing: -0.8px;
    user-select: none;
    text-transform: uppercase;
  }
</style>
