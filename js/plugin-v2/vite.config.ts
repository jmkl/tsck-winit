import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { viteStaticCopy } from "vite-plugin-static-copy";
import { uxp } from "vite-uxp-plugin";
import { config } from "./uxp.config";

const mode = process.env.MODE;

export default defineConfig({
  plugins: [
    uxp(config, mode),
    viteStaticCopy({
      targets: [
        {
          src: "plugin/*",
          dest: ".",
        },
      ],
    }),

    svelte(),
  ],
  build: {
    sourcemap: mode && ["dev", "build"].includes(mode) ? true : false,
    minify: false,
    emptyOutDir: false,
    rollupOptions: {
      external: ["photoshop", "uxp", "fs", "os", "path", "process", "shell"],
      output: {
        format: "cjs",
      },
    },
  },
  publicDir: "public",
});
