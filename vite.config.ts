import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import vitetsconfigpaths from "vite-plugin-tsconfig-paths";
import topLevelAwait from "vite-plugin-top-level-await";
import { sveltekit } from "@sveltejs/kit/vite";
import { MagicRegExpTransformPlugin } from "magic-regexp/transform";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    MagicRegExpTransformPlugin.vite(),
    tailwindcss(),
    sveltekit(),
    vitetsconfigpaths(),
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
});
