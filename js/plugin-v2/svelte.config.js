import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
export default {
  preprocess: vitePreprocess(),
  compilerOptions: {
    customElement: false,
  },
  onwarn: (warning, handler) => {
    if (warning.code && warning.code.startsWith("a11y_")) return;
    handler(warning);
  },
};
