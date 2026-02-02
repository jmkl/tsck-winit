<script lang="ts">
  import placeholder from "$lib/assets/yt-thumb.png";
  let { class: className = "", id, index } = $props();
  const quality = ["MD", "SM", "XS"];
  function fetchSource() {
    switch (index) {
      case 0:
        return `https://img.youtube.com/vi/${id}/maxresdefault.jpg`;
      case 1:
        return `https://img.youtube.com/vi/${id}/hqdefault.jpg`;
      case 2:
        return `https://img.youtube.com/vi/${id}/mqdefault.jpg`;
    }
  }
</script>

<div class="relative flex">
  <span
    class="absolute rounded-sm bg-error px-1 font-avantt text-xs font-bold text-error-content"
    >{quality[index]}</span
  >
  <img
    alt=""
    class={`w-max rounded-xl object-contain ${className}`}
    src={id != null ? fetchSource() : placeholder}
  />
</div>
