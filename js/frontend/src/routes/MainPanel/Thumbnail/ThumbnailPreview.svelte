<script lang="ts">
  import { onMount } from "svelte";

  let {
    templateLines,
    class: className = "",
    typeface,
    canvasMargin = $bindable(),
  } = $props();

  let canvas: HTMLCanvasElement | undefined = $state();
  let imageDataUrl = $state("");

  const MINI_SIZE = 0.5;
  const PADDING = 5;
  const GAP = 0;

  // Fixed canvas dimensions (adjust these to your needs)
  const BASE_WIDTH = 192 * 2;
  const BASE_HEIGHT = 108 * 2;

  const RANGE_UNISANS = [17.85, 20.25, 22, 23.8, 26, 27.1];
  const RANGE_ANTON = [18.8, 21.6, 23.3, 25.2, 27.8, 29.1];

  // Wait for fonts that are declared in CSS
  async function waitForFonts() {
    try {
      await Promise.all([
        document.fonts.load('900 18px "Uni Sans"'),
        document.fonts.load('900 35px "Uni Sans"'),
        document.fonts.load('400 18px "Anton"'),
        document.fonts.load('italic 900 18px "Uni Sans"'),
      ]);

      const checks = [
        document.fonts.check('900 18px "Uni Sans"'),
        document.fonts.check('400 18px "Anton"'),
      ];

      if (checks.every(Boolean)) {
        console.log("✓ All fonts loaded successfully");
        return true;
      } else {
        console.warn("⚠ Some fonts may not be loaded, retrying...");
        await new Promise((resolve) => setTimeout(resolve, 300));
        return true;
      }
    } catch (error) {
      console.error("Font loading error:", error);
      return true;
    }
  }

  function scaleRangeByFontName(index: number, typeface: string) {
    return typeface === "font-unisans"
      ? RANGE_UNISANS[index]
      : RANGE_ANTON[index];
  }

  function drawCanvas() {
    if (!canvas) return;

    const ctx = canvas.getContext("2d", { alpha: false });
    if (!ctx) return;

    canvas.width = BASE_WIDTH;
    canvas.height = BASE_HEIGHT;

    // Clear with black background
    ctx.fillStyle = "#121212";
    ctx.fillRect(0, 0, BASE_WIDTH, BASE_HEIGHT);

    drawGrid(ctx);
    drawText(ctx);

    // Convert canvas to image
    imageDataUrl = canvas.toDataURL("image/png");
  }

  function drawGrid(ctx: CanvasRenderingContext2D) {
    const resolution = 16;
    ctx.strokeStyle = "rgba(255, 255, 255, 0.063)";
    ctx.lineWidth = 1;

    ctx.beginPath();
    // Vertical lines
    for (let x = 0; x < BASE_WIDTH; x += resolution) {
      ctx.moveTo(x, 0);
      ctx.lineTo(x, BASE_HEIGHT);
    }
    // Horizontal lines
    for (let y = 0; y < BASE_HEIGHT; y += resolution) {
      ctx.moveTo(0, y);
      ctx.lineTo(BASE_WIDTH, y);
    }
    ctx.stroke();
  }

  function drawScaleLegend(ctx: CanvasRenderingContext2D, array: number[]) {
    const font_size = 28;
    const w = 35;
    const h = 35;
    const gap = 43;
    let x = 0;
    const y = 0;
    for (const a of array) {
      ctx.fillStyle = "#ac3e31";
      ctx.fillRect(x, y, w, h);
      ctx.fillStyle = "#000";
      ctx.font = `900 ${font_size}px "Uni Sans", "Arial Black"`;
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(String(a + 1), x + w / 2, y + h / 2);
      x += gap;
    }
  }

  function drawText(ctx: CanvasRenderingContext2D) {
    ctx.save();
    drawScaleLegend(
      ctx,
      templateLines
        .filter((line: any) => line.include)
        .map((l: any) => l.scale),
    );
    ctx.restore();

    const lineData = templateLines
      .filter((line: any) => line.include)
      .map((line: any) => {
        const fontSize =
          (BASE_HEIGHT / 169) * scaleRangeByFontName(line.scale, typeface);
        const isAnton = typeface === "font-anton";

        return {
          words: line.text.split(" "),
          fontSize,
          lineHeight: isAnton ? fontSize : fontSize * 0.9,
          italic: line.italic && !isAnton,
          fontWeight: isAnton ? "normal" : "900",
          wordGap: fontSize * 0.05,
        };
      });

    // Calculate total height
    const totalHeight = lineData.reduce(
      (sum: any, data: any) => sum + data.lineHeight,
      0,
    );
    let currentY = BASE_HEIGHT - totalHeight;

    ctx.fillStyle = "#ffffff";
    ctx.textBaseline = "top";

    for (const data of lineData) {
      const fontFamily =
        typeface === "font-anton" ? "Anton" : '"Uni Sans", sans-serif';
      const fontStyle = data.italic ? "italic" : "normal";
      ctx.font = `${fontStyle} ${data.fontWeight} ${data.fontSize}px ${fontFamily}`;

      // Calculate total width for positioning
      const totalWidth = data.words.reduce((sum: any, word: any, idx: any) => {
        const width = ctx.measureText(word.toUpperCase()).width;
        return sum + width + (idx < data.words.length - 1 ? data.wordGap : 0);
      }, 0);

      let currentX =
        typeface === "font-anton" ? (BASE_WIDTH - totalWidth) / 2 : 4;

      // Draw words
      for (const word of data.words) {
        const upperWord = word.toUpperCase();
        ctx.fillText(upperWord, currentX, currentY - 5);
        currentX += ctx.measureText(upperWord).width + data.wordGap;
      }

      currentY += data.lineHeight;
    }
  }

  // Calculate display dimensions and position
  const displayWidth = $derived(BASE_WIDTH * MINI_SIZE);
  const displayHeight = $derived(BASE_HEIGHT * MINI_SIZE);
  const positionX = $derived.by(() => {
    if (typeof window === "undefined") return 0;
    return window.innerWidth - displayWidth - PADDING - GAP;
  });
  const positionY = $derived.by(() => {
    if (typeof window === "undefined") return 0;
    return window.innerHeight - displayHeight - 24 - PADDING - GAP;
  });

  // Initialize on mount
  onMount(async () => {
    await waitForFonts();
    await document.fonts.ready;
    drawCanvas();
  });

  // React to prop changes - only redraw when needed
  $effect(() => {
    templateLines;
    typeface;

    if (canvas) {
      drawCanvas();
    }
  });
</script>

<!-- Hidden canvas for rendering -->
<canvas bind:this={canvas} class="hidden"></canvas>

<!-- Positioned image wrapper -->
{#if imageDataUrl}
  <div
    class={`${className} pointer-events-none fixed right-0 bottom-0 z-999 w-1/2 select-none`}
  >
    <img
      src={imageDataUrl}
      alt="preview"
      class="border-error/30 h-full w-full rounded border shadow-lg"
      style="image-rendering: auto;"
    />
  </div>
{/if}
