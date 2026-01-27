<script lang="ts">
	import RangeSlider from 'svelte-range-slider-pips';
	let {
		min,
		max,
		value = $bindable(),
		step,
		title,
		onChange,
		onAssign,
		resetValue,
		class: className = ''
	} = $props();
</script>

<div class="range-container py-0 {className}">
	<div class="label-container flex flex-row items-center justify-between px-2 select-none">
		<button
			onclick={resetValue}
			class="btn border-0 bg-transparent text-[8px] font-bold italic btn-ghost btn-xs"
		>
			{title}
		</button>
		<div class="pointer-events-none badge badge-soft badge-xs badge-error">
			{value}
		</div>
	</div>

	<RangeSlider
		on:change={(e) => {
			if (onChange) onChange(e.detail.value);
		}}
		on:stop={(e) => {
			if (onAssign) onAssign(e.detail.value);
		}}
		{min}
		{max}
		{step}
		id="rfslider"
		range="min"
		bind:value
		springValues={{ stiffness: 0.2, damping: 0.4 }}
	/>
</div>
