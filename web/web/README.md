# Chip-8 Emulator Web Interface

This is the web interface for the Chip-8 Emulator, built with Astro, React, and WebAssembly.

## Project Structure

- `src/components/Chip8Emulator.tsx`: The main React component that interfaces with the Wasm module and renders to the Canvas.
- `public/rom.ch8`: Default ROM loaded on start.
- `astro.config.mjs`: Astro configuration with React and Wasm integration.

## Setup

1.  **Build the WebAssembly module:**
    Ensure you have `wasm-pack` installed.
    ```bash
    cd ../
    wasm-pack build --target web --out-dir pkg
    ```
    (Note: This has already been done during setup)

2.  **Install dependencies:**
    ```bash
    npm install
    ```

3.  **Run locally:**
    ```bash
    npm run dev
    ```

4.  **Build for production (Cloudflare Pages):**
    ```bash
    npm run build
    ```
    The output directory is `dist/`.

## Controls

- Keypad:
  - 1 2 3 4
  - Q W E R
  - A S D F
  - Z X C V