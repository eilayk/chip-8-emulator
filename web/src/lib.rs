use chip_8::Chip8;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8Wasm {
    chip8: Chip8,
}

#[wasm_bindgen]
impl Chip8Wasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Chip8Wasm {
        let mut chip8 = Chip8::new();
        chip8.init();
        Chip8Wasm { chip8 }
    }

    /// Load ROM to memory
    pub fn load_rom(&mut self, data: &[u8]) {
        self.chip8.load_rom(data);
    }

    /// Run one cycle of the Chip-8 interpreter
    pub fn cycle(&mut self) {
        self.chip8.cycle();
    }

    /// Update timers
    pub fn tick_timers(&mut self) {
        self.chip8.tick_timers();
    }

    /// Set pressed keys
    pub fn set_pressed_keys(&mut self, keys: &[u8]) {
        let mut bool_keys = [false; 16];
        for (i, &k) in keys.iter().enumerate().take(16) {
            bool_keys[i] = k != 0;
        }
        self.chip8.set_pressed_keys(bool_keys);
    }

    /// Get display buffer
    pub fn get_display(&self) -> Vec<u8> {
        self.chip8.get_display().iter().map(|&b| b as u8).collect()
    }
}

impl Default for Chip8Wasm {
    fn default() -> Self {
        Self::new()
    }
}
