/* tslint:disable */
/* eslint-disable */

export class Chip8Wasm {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Get display buffer
   */
  get_display(): Uint8Array;
  /**
   * Update timers
   */
  tick_timers(): void;
  /**
   * Set pressed keys
   */
  set_pressed_keys(keys: Uint8Array): void;
  constructor();
  /**
   * Run one cycle of the Chip-8 interpreter
   */
  cycle(): void;
  /**
   * Load ROM to memory
   */
  load_rom(data: Uint8Array): void;
}
