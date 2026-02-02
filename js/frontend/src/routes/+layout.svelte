<script lang="ts">
  import "$lib/layout.css";
  let { children } = $props();
</script>

<div tabindex="-1" class="absolute inset-0 overflow-hidden">
  {@render children()}
</div>
