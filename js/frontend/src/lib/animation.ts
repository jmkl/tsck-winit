import gsap from "gsap";
export function bubblePop(
  node: HTMLElement,
  {
    delay = 0,
    scale = 0,
    y = 0,
    x = 0,
    duration = 0.8,
    direction = "y",
    ease = "elastic.out(2,1)",
    transformOrigin = "bottom center",
  },
) {
  return {
    duration: duration * 1000,
    css: () => "",
    easing: (t: number) => {
      const easeFn = gsap.parseEase(ease);
      return easeFn(t);
    },
    tick: (t: number) => {
      const axis = direction === "y" ? "y" : "x";

      gsap.set(node, {
        opacity: t,
        [axis]: (axis === "y" ? y : x) * (1 - t),
        scale: scale + (1 - scale) * t,
        transformOrigin,
      });
    },
  };
}

export function slideMe(
  node: HTMLElement,
  {
    delay = 0,
    x = 0,
    y = 0,
    duration = 0.8,
    ease = "elastic.out(2,1)",
    transformOrigin = "bottom center",
  },
) {
  return {
    duration: duration * 1000,
    delay: delay * 1000,
    css: () => "",
    easing: (t: number) => {
      const easeFn = gsap.parseEase(ease);
      return easeFn(t);
    },
    tick: (t: number) => {
      gsap.set(node, {
        opacity: t,
        y: y * (1 - t),
        x: x * (1 - t),
        transformOrigin,
      });
    },
  };
}
