<script lang="ts">
	import { invokePayload } from '$lib';
	import gsap from 'gsap';
	let closeBtn: HTMLOrSVGElement | undefined;
	let maxBtn: HTMLOrSVGElement | undefined = $state();
	let minimizeButton: HTMLOrSVGElement | undefined;
	const def = { duration: 0.5, ease: 'elastic.out(1,.3)' };
	let { maximizeBtn = false } = $props();
	function onClose(e: MouseEvent) {
		if (!closeBtn) return;
		// closeWindow();
		invokePayload({ type: 'CloseWindow' });
		gsap.to(closeBtn, {
			...def,
			strokeWidth: 16,
			transformOrigin: '50% 50%',
			onComplete: () => {
				if (!closeBtn) return;
				gsap.to(closeBtn, { ...def, strokeWidth: 4 });
			}
		});
	}
	function onMinimize(e: MouseEvent) {
		if (!minimizeButton) return;
		// setWindowMinimized(true);
		invokePayload({ type: 'Minimize' });

		gsap.to(minimizeButton, {
			...def,
			strokeWidth: 16,
			transformOrigin: '50% 50%',
			onComplete: () => {
				if (!minimizeButton) return;
				gsap.to(minimizeButton, { ...def, strokeWidth: 4 });
			}
		});
	}
	function onMax(e: MouseEvent) {
		if (!maxBtn) return;
		// setWindowMinimized(true);
		invokePayload({ type: 'Maximize' });

		gsap.to(maxBtn, {
			...def,
			strokeWidth: 16,
			transformOrigin: '50% 50%',
			onComplete: () => {
				if (!maxBtn) return;
				gsap.to(maxBtn, { ...def, strokeWidth: 4 });
			}
		});
	}
	function animateClose(node: HTMLElement, svgs: string[]) {
		// const svgs = ['#p1', '#p2'];
		const rot = 45;
		function enter() {
			svgs.forEach((c, i) => {
				gsap.to(c, {
					...def,
					rotate: i == 0 ? rot * 3 : -rot,

					transformOrigin: '50% 50%'
				});
			});
		}
		function leave() {
			svgs.forEach((c, i) => {
				gsap.to(c, {
					...def,
					rotate: i == 1 ? rot * 4 : -rot * 4,
					transformOrigin: '50% 50%'
				});
			});
		}
		function click() {
			svgs.forEach((c, i) => {
				gsap.to(c, {
					...def,
					strokeWidth: 16,
					transformOrigin: '50% 50%',
					onComplete: () => {
						gsap.to(c, { ...def, strokeWidth: 4 });
					}
				});
			});
		}

		node.addEventListener('mouseenter', enter);
		node.addEventListener('mouseleave', leave);

		return {
			destroy() {
				node.removeEventListener('mouseenter', enter);
				node.removeEventListener('mouseleave', leave);
			}
		};
	}
</script>

<div data-tsck-drag-region class="title-bar bg-base-300">
	<div data-tsck-drag-region class="flex flex-row justify-end gap-2 p-1 px-2">
		<div class="cursor-pointer" use:animateClose={['#p3']}>
			<svg
				bind:this={minimizeButton}
				tabindex="-1"
				onkeydown={() => {}}
				role="button"
				onclick={onMinimize}
				xmlns="http://www.w3.org/2000/svg"
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="4"
				class="ring-0 outline-0 hover:text-warning focus:outline-0"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path id="p3" d="M5 12h14" />
			</svg>
		</div>
		{#if maximizeBtn}
			<div use:animateClose={['#p5', '#p4']} class="cursor-pointer">
				<svg
					bind:this={maxBtn}
					onclick={onMax}
					tabindex="-1"
					role="button"
					onkeydown={() => {}}
					xmlns="http://www.w3.org/2000/svg"
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="4"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="ring-0 outline-0 hover:text-warning focus:outline-0"
					><path id="p5" d="M5 12h14" /><path id="p4" d="M12 5v14" /></svg
				>
			</div>
		{/if}
		<div use:animateClose={['#p1', '#p2']} class="cursor-pointer">
			<svg
				bind:this={closeBtn}
				tabindex="-1"
				onkeydown={() => {}}
				role="button"
				onclick={onClose}
				class="ring-0 outline-0 hover:text-warning focus:outline-0"
				xmlns="http://www.w3.org/2000/svg"
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="4"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path id="p1" d="M18 6 6 18" />
				<path id="p2" d="m6 6 12 12" />
			</svg>
		</div>
	</div>
</div>
