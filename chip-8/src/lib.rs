pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const NUM_KEYS: usize = 16;
const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const START_ADDRESS: u16 = 0x200;
const STACK_SIZE: usize = 16;

const FONTSET_SIZE: usize = 80;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

struct Stack {
    stack: [u16; STACK_SIZE],
    sp: u16,
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            stack: [0; STACK_SIZE],
            sp: 0,
        }
    }
}

struct Memory {
    data: [u8; MEMORY_SIZE],
    pc: u16,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            data: [0; MEMORY_SIZE],
            pc: START_ADDRESS,
        }
    }
}

impl Memory {
    fn init(&mut self) {
        // load fontset into memory
        self.data[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        // set program counter to start address
        self.pc = START_ADDRESS;
    }

    fn load_rom(&mut self, data: &[u8]) {
        let start = START_ADDRESS as usize;
        let end = start + data.len();
        if end > MEMORY_SIZE {
            panic!("ROM too large to fit in memory");
        }
        self.data[start..end].copy_from_slice(data);
    }

    fn fetch_opcode(&mut self) -> u16 {
        // opcode is stored in two consecutive bytes
        let first_byte = self.data[self.pc as usize] as u16;
        let second_byte = self.data[(self.pc + 1) as usize] as u16;
        let opcode = first_byte << 8 | second_byte;
        // increment program counter by 2
        self.next();
        opcode
    }

    fn next(&mut self) {
        self.pc += 2;
    }

    fn prev(&mut self) {
        self.pc -= 2;
    }

    fn get_bytes(&self, start: u16, length: usize) -> &[u8] {
        &self.data[start as usize..start as usize + length]
    }
}

impl Stack {
    pub fn push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

struct Screen {
    pixels: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            pixels: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }
}

impl Screen {
    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, height: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        for row in 0..height {
            if let Some(sprite_row_byte) = sprite.get(row) {
                for col in 0..8 {
                    let pixel_value = (sprite_row_byte >> (7 - col)) & 0x1;
                    let screen_x = x + col;
                    let screen_y = y + row;
                    if screen_x >= SCREEN_WIDTH || screen_y >= SCREEN_HEIGHT {
                        continue; // Skip pixels that are out of bounds
                    }
                    let index = screen_y * SCREEN_WIDTH + screen_x;
                    if pixel_value == 1 {
                        // Check for collision
                        if self.pixels[index] {
                            collision = true;
                        }

                        // Toggle pixel
                        self.pixels[index] ^= true;
                    }
                }
            }
        }
        collision
    }
}

pub struct Chip8 {
    memory: Memory,
    screen: Screen,
    v_registers: [u8; NUM_REGISTERS],
    i_register: u16,
    stack: Stack,
    pressed_keys: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            memory: Memory::default(),
            screen: Screen::default(),
            v_registers: [0; NUM_REGISTERS],
            i_register: 0,
            stack: Stack::default(),
            pressed_keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

impl Chip8 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self) {
        self.memory.init();
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.memory.load_rom(data);
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen.pixels
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // BEEP!
            }
            self.sound_timer -= 1;
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.memory.fetch_opcode();
        self.execute(opcode);
    }

    pub fn set_pressed_keys(&mut self, keys: [bool; NUM_KEYS]) {
        self.pressed_keys = keys;
    }

    pub fn execute(&mut self, opcode: u16) {
        // opcode split into 4 digits. Each is 4 bits
        let digit1 = (opcode & 0xF000) >> 12;
        let digit2 = (opcode & 0x0F00) >> 8;
        let digit3 = (opcode & 0x00F0) >> 4;
        let digit4 = opcode & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0xE, 0) => {
                // clear the display
                self.screen.clear();
            }
            (0, 0, 0xE, 0xE) => {
                // return from subroutine
                let return_address = self.stack.pop();
                self.memory.pc = return_address;
            }
            (1, _, _, _) => {
                // jump to address NNN
                let address = opcode & 0x0FFF;
                self.memory.pc = address;
            }
            (2, _, _, _) => {
                // call subroutine at NNN
                let address = opcode & 0x0FFF;
                self.stack.push(self.memory.pc);
                self.memory.pc = address;
            }
            (3, _, _, _) => {
                // skip next instruction if Vx == NN
                let x = digit2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                if self.v_registers[x] == nn {
                    self.memory.next();
                }
            }
            (4, _, _, _) => {
                // skip next instruction if Vx != NN
                let x = digit2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                if self.v_registers[x] != nn {
                    self.memory.next();
                }
            }
            (5, _, _, 0) => {
                // skip next instruction if Vx == Vy
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_registers[x] == self.v_registers[y] {
                    self.memory.next();
                }
            }
            (6, _, _, _) => {
                // set Vx = NN
                let x = digit2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v_registers[x] = nn;
            }
            (7, _, _, _) => {
                // set Vx = Vx + NN
                let x = digit2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v_registers[x] = self.v_registers[x].wrapping_add(nn);
            }
            (8, _, _, 0) => {
                // set Vx = Vy
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_registers[x] = self.v_registers[y];
            }
            (8, _, _, 1) => {
                // set Vx = Vx OR Vy
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_registers[x] |= self.v_registers[y];
            }
            (8, _, _, 2) => {
                // set Vx = Vx AND Vy
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_registers[x] &= self.v_registers[y];
            }
            (8, _, _, 3) => {
                // set Vx = Vx XOR Vy
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_registers[x] ^= self.v_registers[y];
            }
            (8, _, _, 4) => {
                // set Vx = Vx + Vy, set VF = carry
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (result, carry) = self.v_registers[x].overflowing_add(self.v_registers[y]);
                self.v_registers[x] = result;
                self.v_registers[0xF] = carry as u8;
            }
            (8, _, _, 5) => {
                // set Vx = Vx - Vy, set VF = NOT borrow
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (result, borrow) = self.v_registers[x].overflowing_sub(self.v_registers[y]);
                self.v_registers[x] = result;
                self.v_registers[0xF] = (!borrow) as u8;
            }
            (8, _, _, 6) => {
                // Vx = Vx SHR 1, store dropped bit in VF
                let x = digit2 as usize;
                self.v_registers[0xF] = self.v_registers[x] & 0x1;
                self.v_registers[x] >>= 1;
            }
            (8, _, _, 7) => {
                // set Vx = Vy - Vx, set VF = NOT borrow
                let x = digit2 as usize;
                let y = digit3 as usize;
                let (result, borrow) = self.v_registers[y].overflowing_sub(self.v_registers[x]);
                self.v_registers[x] = result;
                self.v_registers[0xF] = (!borrow) as u8;
            }
            (8, _, _, 0xE) => {
                // set Vx = Vx SHL 1, store dropped bit in VF
                let x = digit2 as usize;
                self.v_registers[0xF] = self.v_registers[x] >> 7;
                self.v_registers[x] <<= 1;
            }
            (9, _, _, 0) => {
                // skip next instruction if Vx != Vy
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_registers[x] != self.v_registers[y] {
                    self.memory.next();
                }
            }
            (0xA, _, _, _) => {
                // set I = NNN
                let address = opcode & 0x0FFF;
                self.i_register = address;
            }
            (0xB, _, _, _) => {
                // jump to address NNN + V0
                let nnn = opcode & 0x0FFF;
                self.memory.pc = nnn + self.v_registers[0] as u16;
            }
            (0xC, _, _, _) => {
                // set Vx = random number AND NN
                let x = digit2 as usize;
                let nn = (opcode & 0x00FF) as u8;
                self.v_registers[x] = rand::random::<u8>() & nn;
            }
            (0xD, _, _, _) => {
                // draw sprite at (Vx, Vy) with width 8 pixels and height N pixels
                let x = digit2 as usize;
                let y = digit3 as usize;
                let height = digit4 as usize;
                // register Vx contains x coordinate
                let vx = self.v_registers[x] as usize;
                let x_coor = vx % SCREEN_WIDTH;
                // register Vy contains y coordinate
                let vy = self.v_registers[y] as usize;
                let y_coor = vy % SCREEN_HEIGHT;

                // get sprite data from memory starting at I register
                let sprite = self.memory.get_bytes(self.i_register, height);

                // init vf to 0
                self.v_registers[0xF] = 0;
                // draw sprite on screen
                // record collision in vf
                self.v_registers[0xF] = self.screen.draw_sprite(x_coor, y_coor, height, sprite) as u8;
            }
            (0xE, _, 9, 0xE) => {
                // skip next instruction if key with the value of Vx is pressed
                let x = digit2 as usize;
                if self.pressed_keys[self.v_registers[x] as usize] {
                    self.memory.next();
                }
            }
            (0xE, _, 0xA, 1) => {
                // skip next instruction if key with the value of Vx is not pressed
                let x = digit2 as usize;
                if !self.pressed_keys[self.v_registers[x] as usize] {
                    self.memory.next();
                }
            }
            (0xF, _, 0, 7) => {
                // set Vx = delay timer value
                let x = digit2 as usize;
                self.v_registers[x] = self.delay_timer;
            }
            (0xF, _, 0, 0xA) => {
                // wait for a key press, then store the value of the key in Vx
                let x = digit2 as usize;
                // find first pressed key
                let pressed_key_option: Option<usize> = self.pressed_keys.iter().position(|&k| k);
                if let Some(pressed_key) = pressed_key_option {
                    // store key in Vx
                    self.v_registers[x] = pressed_key as u8;
                } else {
                    // no key pressed, decrement pc to repeat this instruction
                    self.memory.prev();
                }
            }
            (0xF, _, 1, 5) => {
                // set delay timer = Vx
                let x = digit2 as usize;
                self.delay_timer = self.v_registers[x];
            }
            (0xF, _, 1, 8) => {
                // set sound timer = Vx
                let x = digit2 as usize;
                self.sound_timer = self.v_registers[x];
            }
            (0xF, _, 1, 0xE) => {
                // set I = I + Vx
                let x = digit2 as usize;
                self.i_register = self.i_register.wrapping_add(self.v_registers[x] as u16);
            }
            (0xF, _, 2, 9) => {
                // set I = location of sprite for digit Vx
                // get digit from Vx
                let x = digit2 as usize;
                let digit = self.v_registers[x] as u16;
                // set I to the location of the sprite
                self.i_register = digit * 5; // each sprite is 5 bytes long
                
            }
            (0xF, _, 3, 3) => {
                // store BCD representation of Vx in memory locations I, I+1, and I+2
                let x = digit2 as usize;
                let value = self.v_registers[x];
                self.memory.data[self.i_register as usize] = value / 100;
                self.memory.data[self.i_register as usize + 1] = (value / 10) % 10;
                self.memory.data[self.i_register as usize + 2] = value % 10;
            }
            (0xF, _, 5, 5) => {
                // store registers V0 through Vx in memory starting at location I
                let x = digit2 as usize;
                for offset in 0..=x {
                    self.memory.data[self.i_register as usize + offset] = self.v_registers[offset];
                }
            }
            (0xF, _, 6, 5) => {
                // load registers V0 through Vx from memory starting at location I
                let x = digit2 as usize;
                for offset in 0..=x {
                    self.v_registers[offset] = self.memory.data[self.i_register as usize + offset];
                }
            }
            (_, _, _, _) => {
                // unimplemented opcode
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let mut chip8 = Chip8::new();
        chip8.init();
        // Fontset should be loaded
        assert_eq!(chip8.memory.data[0], 0xF0);
    }

    #[test]
    fn test_load_rom() {
        let mut chip8 = Chip8::new();
        chip8.init();
        let rom = [0x00, 0xE0]; // CLS
        chip8.load_rom(&rom);
        
        assert_eq!(chip8.memory.data[START_ADDRESS as usize], 0x00);
        assert_eq!(chip8.memory.data[START_ADDRESS as usize + 1], 0xE0);
    }
    
    #[test]
    fn test_cycle() {
        let mut chip8 = Chip8::new();
        chip8.init();
        // 6xNN: Set Vx = NN
        let rom = [0x60, 0xAA]; 
        chip8.load_rom(&rom);
        
        chip8.cycle();
        assert_eq!(chip8.v_registers[0], 0xAA);
    }
}
