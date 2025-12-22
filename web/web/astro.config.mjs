// @ts-check
import { defineConfig } from 'astro/config';
import react from '@astrojs/react';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

import tailwindcss from "@tailwindcss/vite";

// https://astro.build/config
export default defineConfig({
  integrations: [react()],
  vite: {
    plugins: [wasm(), topLevelAwait(), tailwindcss()]
  }
});