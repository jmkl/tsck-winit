// keyboard.svelte.ts
type KeyHandler = (event: KeyboardEvent) => void;

type Key =
  // Letters
  | "a"
  | "b"
  | "c"
  | "d"
  | "e"
  | "f"
  | "g"
  | "h"
  | "i"
  | "j"
  | "k"
  | "l"
  | "m"
  | "n"
  | "o"
  | "p"
  | "q"
  | "r"
  | "s"
  | "t"
  | "u"
  | "v"
  | "w"
  | "x"
  | "y"
  | "z"
  | "A"
  | "B"
  | "C"
  | "D"
  | "E"
  | "F"
  | "G"
  | "H"
  | "I"
  | "J"
  | "K"
  | "L"
  | "M"
  | "N"
  | "O"
  | "P"
  | "Q"
  | "R"
  | "S"
  | "T"
  | "U"
  | "V"
  | "W"
  | "X"
  | "Y"
  | "Z"
  // Numbers
  | "0"
  | "1"
  | "2"
  | "3"
  | "4"
  | "5"
  | "6"
  | "7"
  | "8"
  | "9"
  // Function keys
  | "F1"
  | "F2"
  | "F3"
  | "F4"
  | "F5"
  | "F6"
  | "F7"
  | "F8"
  | "F9"
  | "F10"
  | "F11"
  | "F12"
  // Navigation
  | "ArrowUp"
  | "ArrowDown"
  | "ArrowLeft"
  | "ArrowRight"
  | "Home"
  | "End"
  | "PageUp"
  | "PageDown"
  // Editing
  | "Backspace"
  | "Delete"
  | "Insert"
  | "Enter"
  | "Tab"
  | "Escape"
  | "Space"
  // Modifiers
  | "Shift"
  | "Control"
  | "Alt"
  | "Meta"
  | "CapsLock"
  // Symbols
  | "!"
  | "@"
  | "#"
  | "$"
  | "%"
  | "^"
  | "&"
  | "*"
  | "("
  | ")"
  | "-"
  | "_"
  | "="
  | "+"
  | "["
  | "]"
  | "{"
  | "}"
  | "\\"
  | "|"
  | ";"
  | ":"
  | "'"
  | '"'
  | ","
  | "."
  | "/"
  | "?"
  | "<"
  | ">"
  | "`"
  | "~"
  // Other
  | "PrintScreen"
  | "ScrollLock"
  | "Pause"
  | "ContextMenu"
  | "NumLock"
  | "Clear"
  // Allow any string for custom keys
  | (string & {});

const numToShifted: Record<string, string> = {
  "1": "!",
  "2": "@",
  "3": "#",
  "4": "$",
  "5": "%",
  "6": "^",
  "7": "&",
  "8": "*",
  "9": "(",
  "0": ")",
};
export function getShiftedNum(num: string) {
  return numToShifted[num];
}
export type KeyConfig = {
  key: Key | string; // Support sequences like "q q" or "j k"
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean;
  handler: KeyHandler;
  // Condition function - only execute if returns true
  when?: () => boolean;
  // Skip if input/textarea is focused (default: true)
  ignoreInputs?: boolean;
  // Only run when active element has this class
  onlyInClass?: string;
  // Only run when active element matches this selector
  onlyInSelector?: string;
  // Timeout for key sequences in ms (default: 1000)
  sequenceTimeout?: number;
  // Priority level (higher = runs first, default: 0)
  priority?: number;
  useCapture?: boolean; // NEW: Allow capture phase
};

export function useKeyboard(configs: KeyConfig[]) {
  let keySequence: string[] = [];
  let sequenceTimer: NodeJS.Timeout | null = null;

  const clearSequence = () => {
    keySequence = [];
    if (sequenceTimer !== null) {
      clearTimeout(sequenceTimer);
      sequenceTimer = null;
    }
  };

  // Sort configs by priority (higher first)
  const sortedConfigs = [...configs].sort(
    (a, b) => (b.priority ?? 0) - (a.priority ?? 0),
  );

  const handleKeyDown = (e: KeyboardEvent) => {
    // const target = e.target as HTMLElement;
    const target = document.activeElement as HTMLElement;
    for (const config of sortedConfigs) {
      // Check if should only run in specific class
      if (config.onlyInClass) {
        if (!target.classList.contains(config.onlyInClass)) {
          continue;
        }
        // If onlyInClass is set, we explicitly want to run in that input
        // So skip the ignoreInputs check below
      } else if (config.onlyInSelector) {
        // Check if should only run in specific selector
        if (!target.matches(config.onlyInSelector)) {
          continue;
        }
        // If onlyInSelector is set, we explicitly want to run in that element
        // So skip the ignoreInputs check below
      } else {
        // Only apply ignoreInputs check when NOT using onlyInClass/onlyInSelector
        const ignoreInputs = config.ignoreInputs !== false;
        if (ignoreInputs) {
          if (
            target.tagName === "INPUT" ||
            target.tagName === "TEXTAREA" ||
            target.isContentEditable
          ) {
            continue;
          }
        }
      }

      // Check custom condition
      if (config.when && !config.when()) {
        continue;
      }

      const keys = config.key.split(" ").filter((k) => k);
      const isSequence = keys.length > 1;

      if (isSequence) {
        // Handle key sequences
        const timeout = config.sequenceTimeout ?? 1000;

        // Clear existing timer
        if (sequenceTimer !== null) {
          clearTimeout(sequenceTimer);
        }

        // Add current key to sequence
        keySequence.push(e.key.toLowerCase());

        // Set new timer to clear sequence
        sequenceTimer = setTimeout(clearSequence, timeout);

        // Check if sequence matches
        const currentSequence = keySequence.join(" ");
        const targetSequence = keys.map((k) => k.toLowerCase()).join(" ");

        if (currentSequence === targetSequence) {
          e.preventDefault();
          clearSequence();
          config.handler(e);
          break;
        } else if (!targetSequence.startsWith(currentSequence)) {
          // If current sequence doesn't match any prefix, clear it
          clearSequence();
        }
      } else {
        // Handle single key
        const keyMatch = e.key.toLowerCase() === keys[0].toLowerCase();
        const ctrlMatch = config.ctrl
          ? e.ctrlKey
          : !e.ctrlKey || config.ctrl === undefined;
        const shiftMatch = config.shift
          ? e.shiftKey
          : !e.shiftKey || config.shift === undefined;
        const altMatch = config.alt
          ? e.altKey
          : !e.altKey || config.alt === undefined;
        const metaMatch = config.meta
          ? e.metaKey
          : !e.metaKey || config.meta === undefined;

        if (keyMatch && ctrlMatch && shiftMatch && altMatch && metaMatch) {
          e.preventDefault();
          e.stopPropagation(); // ADDED: Stop propagation
          clearSequence(); // Clear any ongoing sequence
          config.handler(e);
          break;
        }
      }
    }
  };

  $effect(() => {
    // Use capture phase if specified (defaults to false for bubble phase)
    const useCapture = configs.some((c) => c.useCapture) ?? false;
    window.addEventListener("keydown", handleKeyDown, useCapture);
    return () => {
      window.removeEventListener("keydown", handleKeyDown, useCapture);
      clearSequence();
    };
  });
}

// Usage Examples:
//
// <script lang="ts">
//   import { useKeyboard } from './keyboard.svelte';
//
//   let count = $state(0);
//   let isDrawerOpen = $state(false);
//
//   useKeyboard([
//     // Single keys
//     { key: 'ArrowUp', handler: () => count++ },
//     { key: 'ArrowDown', handler: () => count-- },
//
//     // Key sequences (vim-style)
//     { key: 'g g', handler: () => window.scrollTo(0, 0) }, // Go to top
//     { key: 'g i', handler: () => Logger.log("-",'Go to inbox') },
//
//     // Only run when input has specific class (with priority)
//     {
//       key: 'Enter',
//       ctrl: true,
//       handler: () => Logger.log("-",'Search submitted!'),
//       onlyInClass: 'image-search',
//       priority: 10  // Higher priority than parent
//     },
//
//     // Only run in specific selector
//     {
//       key: 'Escape',
//       handler: (e) => (e.target as HTMLInputElement).value = '',
//       onlyInSelector: 'input[type="text"]'
//     },
//
//     // Conditional sequences
//     {
//       key: 'Escape',
//       handler: () => isDrawerOpen = false,
//       when: () => isDrawerOpen
//     }
//   ]);
// </script>
//
// <input class="image-search" placeholder="Search images..." />
// <h1>Count: {count}</h1>
// <p>Try: ↑/↓ arrows, "g g", "q q", "j k"</p>
// <p>Focus the input and press Ctrl+Enter (priority 10)</p>

// ============================================
// Example: Parent vs Child Component Priority
// ============================================
//
// <!-- Parent.svelte -->
// <script lang="ts">
//   useKeyboard([
//     {
//       key: 'Enter',
//       ctrl: true,
//       handler: () => Logger.log("-",'Parent handler'),
//       priority: 0  // Lower priority
//     }
//   ]);
// </script>
//
// <ChildComponent />
//
// <!-- ChildComponent.svelte -->
// <script lang="ts">
//   useKeyboard([
//     {
//       key: 'Enter',
//       ctrl: true,
//       handler: () => Logger.log("-",'Child handler - this runs first!'),
//       onlyInClass: 'child-input',
//       priority: 10  // Higher priority
//     }
//   ]);
// </script>
//
// <input class="child-input" />
//
// When focused in .child-input and pressing Ctrl+Enter:
// - Child handler runs (priority 10, onlyInClass matches)
// - Parent handler is blocked by preventDefault

// ============================================
// Example: Cycling through buttons with arrow keys
// ============================================
//
// <script lang="ts">
//   import { useKeyboard } from './keyboard.svelte';
//
//   const buttons = [
//     { id: 1, label: 'Save', action: () => Logger.log("-",'Saved!') },
//     { id: 2, label: 'Delete', action: () => Logger.log("-",'Deleted!') },
//     { id: 3, label: 'Cancel', action: () => Logger.log("-",'Cancelled!') },
//   ];
//
//   let focusedIndex = $state(0);
//   let buttonRefs: HTMLButtonElement[] = [];
//
//   useKeyboard([
//     {
//       key: 'ArrowDown',
//       handler: () => {
//         focusedIndex = (focusedIndex + 1) % buttons.length;
//         buttonRefs[focusedIndex]?.focus();
//       }
//     },
//     {
//       key: 'ArrowUp',
//       handler: () => {
//         focusedIndex = (focusedIndex - 1 + buttons.length) % buttons.length;
//         buttonRefs[focusedIndex]?.focus();
//       }
//     },
//     {
//       key: 'Enter',
//       handler: () => {
//         buttonRefs[focusedIndex]?.click();
//       }
//     }
//   ]);
// </script>
//
// <div>
//   {#each buttons as btn, i}
//     <button
//       bind:this={buttonRefs[i]}
//       onclick={btn.action}
//       class:focused={focusedIndex === i}
//     >
//       {btn.label}
//     </button>
//   {/each}
// </div>
//
// <style>
//   .focused { outline: 2px solid blue; }
// </style>
