<script lang="ts">
	import { onMount } from 'svelte';
	import gsap from 'gsap';

	let {
		templateLines,
		class: className = '',
		isMini = $bindable(),
		typeface,
		canvasMargin = $bindable()
	} = $props();

	let canvasWrapper: HTMLDivElement | undefined = $state();
	let canvas: HTMLCanvasElement | undefined = $state();

	let displayWidth = $state(0);
	let displayHeight = $state(0);
	let canvasHeight = $state(0);
	let canvasWidth = $state(0);

	const RANGE_UNISANS = [17.85, 20.25, 22, 23.8, 26, 27.1];
	const RANGE_ANTON = [18.8, 21.6, 23.3, 25.2, 27.8, 29.1];

	// Wait for fonts that are declared in CSS
	async function waitForFonts() {
		try {
			// Force the browser to load the specific font weights/sizes we need
			await Promise.all([
				document.fonts.load('900 18px "Uni Sans"'),
				document.fonts.load('900 35px "Uni Sans"'),
				document.fonts.load('400 18px "Anton"'),
				document.fonts.load('italic 900 18px "Uni Sans"')
			]);

			// Double-check they're actually loaded
			const checks = [
				document.fonts.check('900 18px "Uni Sans"'),
				document.fonts.check('400 18px "Anton"')
			];

			if (checks.every(Boolean)) {
				console.log('✓ All fonts loaded successfully');
				return true;
			} else {
				console.warn('⚠ Some fonts may not be loaded, retrying...');
				await new Promise((resolve) => setTimeout(resolve, 300));
				return true;
			}
		} catch (error) {
			console.error('Font loading error:', error);
			// Continue anyway with fallback
			return true;
		}
	}
	function scaleRangeByFontName(index: number, typeface: string) {
		return typeface === 'font-unisans' ? RANGE_UNISANS[index] : RANGE_ANTON[index];
	}

	function getContainerWidth(): number {
		if (!canvasWrapper) return window.innerWidth;
		return (
			canvasWrapper.clientWidth || canvasWrapper.parentElement?.clientWidth || window.innerWidth
		);
	}

	function updateDimensions() {
		const containerWidth = getContainerWidth();

		// Only update canvas size when NOT in mini mode
		canvasWidth = containerWidth;
		canvasHeight = Math.round((9 / 16) * containerWidth);

		displayWidth = containerWidth;
		displayHeight = (9 / 16) * containerWidth;
	}

	function drawCanvas() {
		if (!canvas || canvasWidth === 0 || canvasHeight === 0) return;

		const ctx = canvas.getContext('2d', { alpha: false });
		if (!ctx) return;

		canvas.width = canvasWidth;
		canvas.height = canvasHeight;

		// Clear with black background
		ctx.fillStyle = '#121212';
		ctx.fillRect(0, 0, canvasWidth, canvasHeight);

		drawGrid(ctx);
		drawText(ctx);
	}

	function drawGrid(ctx: CanvasRenderingContext2D) {
		const resolution = isMini ? 16 : 8;
		ctx.strokeStyle = 'rgba(255, 255, 255, 0.063)';
		ctx.lineWidth = 1;

		ctx.beginPath();
		// Vertical lines
		for (let x = 0; x < canvasWidth; x += resolution) {
			ctx.moveTo(x, 0);
			ctx.lineTo(x, canvasHeight);
		}
		// Horizontal lines
		for (let y = 0; y < canvasHeight; y += resolution) {
			ctx.moveTo(0, y);
			ctx.lineTo(canvasWidth, y);
		}
		ctx.stroke();
	}
	function drawScaleLegend(ctx: CanvasRenderingContext2D, array: number[]) {
		const font_size = isMini ? 35 : 15;
		const w = isMini ? 35 : 15;
		const h = isMini ? 35 : 15;
		const gap = isMini ? 43 : 16;
		let x = 0;
		const y = 0;
		for (const a of array) {
			ctx.fillStyle = '#ac3e31';
			ctx.fillRect(x, y, w, h);
			ctx.fillStyle = '#000';
			ctx.font = `900 ${font_size}px "Uni Sans", "Arial Black"`;
			ctx.textAlign = 'center';
			ctx.textBaseline = 'middle';
			ctx.fillText(String(a + 1), x + w / 2, y + h / 2);
			x += gap;
		}
	}

	function drawText(ctx: CanvasRenderingContext2D) {
		ctx.save();
		drawScaleLegend(
			ctx,
			templateLines.filter((line: any) => line.include).map((l: any) => l.scale)
		);
		ctx.restore();

		const lineData = templateLines
			.filter((line: any) => line.include)
			.map((line: any) => {
				const fontSize = (canvasHeight / 169) * scaleRangeByFontName(line.scale, typeface);
				const isAnton = typeface === 'font-anton';

				return {
					words: line.text.split(' '),
					fontSize,
					lineHeight: isAnton ? fontSize : fontSize * 0.9,
					italic: line.italic && !isAnton,
					fontWeight: isAnton ? 'normal' : '900',
					wordGap: fontSize * (isMini ? 0.05 : 0.1)
				};
			});

		// Calculate total height
		const totalHeight = lineData.reduce((sum: any, data: any) => sum + data.lineHeight, 0);
		let currentY = canvasHeight - totalHeight;

		ctx.fillStyle = '#ffffff';
		ctx.textBaseline = 'top';

		for (const data of lineData) {
			const fontFamily = typeface === 'font-anton' ? 'Anton' : '"Uni Sans", sans-serif';
			const fontStyle = data.italic ? 'italic' : 'normal';
			ctx.font = `${fontStyle} ${data.fontWeight} ${data.fontSize}px ${fontFamily}`;

			// Calculate total width for positioning
			const totalWidth = data.words.reduce((sum: any, word: any, idx: any) => {
				const width = ctx.measureText(word.toUpperCase()).width;
				return sum + width + (idx < data.words.length - 1 ? data.wordGap : 0);
			}, 0);

			let currentX = typeface === 'font-anton' ? (canvasWidth - totalWidth) / 2 : 4;

			// Draw words
			for (const word of data.words) {
				const upperWord = word.toUpperCase();
				ctx.fillText(upperWord, currentX, currentY - 5);
				currentX += ctx.measureText(upperWord).width + data.wordGap;
			}

			currentY += data.lineHeight;
		}
	}
	async function sleep(duration: number) {
		return new Promise((resolve, _) => {
			setTimeout(() => {
				resolve(0);
			}, duration);
		});
	}

	// Initialize on mount
	onMount(async () => {
		// Set initial canvas size based on container
		const containerWidth = getContainerWidth();
		canvasWidth = containerWidth;
		canvasHeight = Math.round((9 / 16) * containerWidth);

		updateDimensions();
		await waitForFonts();
		document.fonts.ready.then(drawCanvas);
		await sleep(3000);
		console.log('done');
		rescaleCanvas();
	});

	// Handle window resize
	function handleResize() {
		updateDimensions();
		drawCanvas();
	}
	// React to prop changes
	$effect(() => {
		templateLines;
		typeface;

		updateDimensions();
		drawCanvas();
	});
	let timeline = gsap.timeline({
		defaults: { duration: 0.5, ease: 'elastic.out(2,1)' }
	});
	const trackingObj = { trackme: isMini ? 1 : 0 };
	function rescaleCanvas() {
		if (canvas) {
			const gap = 0;
			const padding = 5;
			const mini_size = 0.3;
			const min_x = window.innerWidth - canvas.width * mini_size - padding - gap;
			const min_y = canvas?.height * mini_size + 24 + padding + gap;
			// Clear previous timeline if it exists
			if (timeline) {
				timeline.kill();
			}
			timeline = gsap.timeline({
				defaults: { duration: 0.5, ease: 'elastic.out(2,1)' }
			});
			timeline
				.to(canvas, {
					scale: isMini ? mini_size : 1,

					y: isMini ? window.innerHeight - min_y : 0,
					x: isMini ? min_x : 0,
					transformOrigin: 'top left',
					onComplete: () => {}
				})
				.to(
					trackingObj,
					{
						trackme: isMini ? 0 : 1,
						onUpdate: () => {
							if (canvas) canvasMargin = trackingObj.trackme * canvas?.height;
						}
					},
					0
				);
		}
	}
	$effect(() => {
		isMini;
		rescaleCanvas();
	});
</script>

<svelte:window on:resize={handleResize} />

<div
	class={`${className} pointer-events-none absolute z-999 flex w-full shrink-0 items-start justify-start select-none`}
	bind:this={canvasWrapper}
>
	<div
		style=" width: {displayWidth}px; height: {displayHeight}px;"
		class="relative flex shrink-0 origin-top-right transition-all duration-100"
	>
		<canvas
			bind:this={canvas}
			width={canvasWidth}
			height={canvasHeight}
			style="width: 100%; height: 100%; image-rendering: {isMini ? 'crisp-edges' : 'auto'};"
			class="rounded border border-error/30 shadow-lg"
		></canvas>
	</div>
</div>
