<script lang="ts">
  import gsap from "gsap";

  let {
    indexKe,
    color,
    class: className = "",
    click = undefined,
    strokeWidth = 1,
  } = $props();
  let nip = $state({ x: 0, y: 1, r: 18 });
  const SKIN_COLOR = "#ffdbac";
  let realColor = $state(SKIN_COLOR);
  let isOnclick = $state(false);
  function useButtonAnimation(node: HTMLElement) {
    // initial stroke width value
    let tl = gsap.timeline({
      defaults: {
        duration: 1,
        ease: "elastic.out(3,1)",
      },
    });
    let s = {
      y: { from: 0, to: -5 },
      x: { from: 0, to: 0 },
      scale: { from: 1, to: 1.2 },
      nipY: { from: 2, to: 0 },
      nipX: { from: -2, to: -3 },
      nipR: { from: 0, to: 18 },
    };
    function handleEnter() {
      if (isOnclick) return;
      tl.clear();
      let state = {
        y: s.y.from,
        scale: s.scale.from,
        nipY: s.nipY.from,
      };
      tl.to(state, {
        y: s.y.to,
        scale: s.scale.to,
        nipY: s.nipY.to,
        onUpdate: () => {
          realColor = color;
          gsap.set(node, { y: state.y, scale: state.scale });
          nip.y = state.nipY;
        },
      });
    }
    //!idle
    function idle() {}
    function handleLeave() {
      if (isOnclick) return;
      tl.clear();
      let state = { y: s.y.to, scale: s.scale.to, nipY: s.nipY.to };
      tl.to(state, {
        y: s.y.from,
        scale: s.scale.from,
        nipY: s.nipY.from,
        onUpdate: () => {
          realColor = SKIN_COLOR;
          gsap.set(node, { y: state.y, scale: state.scale });
          nip.y = state.nipY * state.scale;
          nip.r = state.scale * 4;
        },
      });
    }

    function clickme() {
      tl.clear();
      let state = {
        y: s.y.from,
        scale: s.scale.from,
        nipY: s.nipY.from,
        nipX: s.nipX.from,
      };
      tl.to(state, {
        y: s.y.to,
        scale: s.scale.to,
        nipY: s.nipY.to + 2,
        nipX: s.nipX.to,
        onUpdate: () => {
          realColor = color;
          gsap.set(node, { y: state.y, scale: state.scale });
          nip.y = state.nipY;
          nip.x = state.nipX;
        },
      });
      tl.to(state, {
        y: s.y.from,
        scale: s.scale.from,
        nipY: s.nipY.from,
        nipX: s.nipX.from,
        onUpdate: () => {
          realColor = SKIN_COLOR;
          gsap.set(node, { y: state.y, scale: state.scale });
          nip.y = state.nipY * state.scale;
          nip.x = state.nipX * state.scale;
          nip.r = state.scale * 4;
        },
      });
    }

    function handleClick() {
      isOnclick = true;
      tl.clear();
      let state = { nipY: s.nipY.from, scale: s.scale.from, y: s.y.from };
      tl.to(state, {
        scale: 1.3,
        y: 0,
        nipY: 1.2,
        duration: 1,
        onUpdate: () => {
          gsap.set(node, { y: state.y, scale: state.scale });
          nip.y = state.nipY;
          nip.r = state.scale * 1.3;
        },
        onComplete: () => {
          isOnclick = false;
          gsap.set(node, { y: s.y.from, scale: s.scale.from, x: s.x.from });
          nip.y = s.nipY.from;
          nip.r = s.nipR.from;
        },
      });
    }
    node.addEventListener("mouseenter", handleEnter);
    node.addEventListener("mouseleave", handleLeave);
    node.addEventListener("click", handleClick);

    return {
      destroy() {
        node.removeEventListener("mouseenter", handleEnter);
        node.removeEventListener("mouseleave", handleLeave);
        node.removeEventListener("click", handleClick);
      },
    };
  }
</script>

<div
  use:useButtonAnimation
  onkeydown={undefined}
  tabindex="-1"
  role="button"
  onclick={click}
  class="{className} even:*:scale-x-[-1] relative w-8 h-8 -mx-1 {[
    2, 4, 6,
  ].includes(indexKe)
    ? 'ml-3'
    : ''} focus:outline-0 ring-0"
  style="color:{color}"
>
  <svg class="boobs" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
    <circle cx="10" cy="10" r="8" fill={color} />
    <path
      fill="none"
      stroke="#1e0f00"
      stroke-linecap="round"
      stroke-miterlimit="10"
      stroke-width={strokeWidth}
      d="M16.2 4.2c4 8.5-1.1 14.2-7.2 14.2A8.5 8.5 0 0 1 .6 10C.6 7 2.3 2.8 6 .6"
    />
    <circle
      style="transform:translateY({nip.y}px) translateX({nip.x}px);"
      fill="#222"
      cx="9"
      cy="10"
      r="2"
      opacity=".2"
    />
    <circle
      style="transform:translateY({nip.y}px) translateX({nip.x}px);"
      fill="#fff"
      cx="8.7"
      cy="8.9"
      r="1"
    />
    <path
      style="transform:rotate({nip.y *
        15}deg) translateY({nip.y}px) translateX({nip.x}px);transform-origin:40% 50%;"
      fill="none"
      stroke="#1e0f00"
      stroke-linecap="round"
      stroke-miterlimit="10"
      stroke-width={strokeWidth}
      d="M11.6 10c0 1.3-1.2 2.5-2.5 2.5S6.5 11.3 6.5 10c0-1.1.8-2 1.8-2.4"
    />
  </svg>

  <!-- <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    class="absolute z-0"
    fill="currentColor"
    stroke="currentColor"
    stroke-width={strokeWidth}
  >
    <circle cx="12" cy="12" r="8" />
  </svg>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    class="absolute z-0"
    fill="#22222244"
  >
    <circle cx={nip.x + 12} cy={nip.y + 12} r={nip.r} />
  </svg>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    class="absolute z-0"
    fill="#22222244"
  >
    <circle cx={nip.x + 12} cy={nip.y + 12} r={nip.r / 2} />
  </svg> -->
</div>
