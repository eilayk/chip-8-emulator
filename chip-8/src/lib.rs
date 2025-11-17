const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const NUM_KEYS: usize = 16;
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
    stack: [u8; STACK_SIZE],
    sp: u8,
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

    fn fetch_opcode(&mut self) -> u16 {
        // opcode is stored in two consecutive bytes
        let first_byte = self.data[self.pc as usize] as u16;
        let second_byte = self.data[(self.pc + 1) as usize] as u16;
        let opcode = first_byte << 8 | second_byte;
        // increment program counter by 2
        self.pc += 2;
        opcode
    }
}

impl Stack {
    pub fn push(&mut self, value: u8) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

pub struct Chip8 {
    memory: Memory,
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
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
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
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

    pub fn cycle(&mut self) {
        let opcode = self.memory.fetch_opcode();
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
                self.screen.fill(false);
            }
            (0, 0, 0xE, 0xE) => {
                // return from subroutine
                let return_address = self.stack.pop() as u16;
                self.memory.pc = return_address;
            }
            (1, _, _, _) => {
                // jump to address NNN
            }
            (2, _, _, _) => {
                // call subroutine at NNN
            }
            (3, _, _, _) => {
                // skip next instruction if Vx == NN
            }
            (4, _, _, _) => {
                // skip next instruction if Vx != NN
            }
            (5, _, _, 0) => {
                // skip next instruction if Vx == Vy
            }
            (6, _, _, _) => {
                // set Vx = NN
            }
            (7, _, _, _) => {
                // set Vx = Vx + NN
            }
            (8, _, _, 0) => {
                // set Vx = Vy
            }
            (8, _, _, 1) => {
                // set Vx = Vx OR Vy
            }
            (8, _, _, 2) => {
                // set Vx = Vx AND Vy
            }
            (8, _, _, 3) => {
                // set Vx = Vx XOR Vy
            }
            (8, _, _, 4) => {
                // set Vx = Vx + Vy, set VF = carry
            }
            (8, _, _, 5) => {
                // set Vx = Vx - Vy, set VF = NOT borrow
            }
            (8, _, _, 6) => {
                // Vx = Vx SHR 1, store dropped bit in VF
            }
            (8, _, _, 7) => {
                // set Vx = Vy - Vx, set VF = NOT borrow
            }
            (8, _, _, 0xE) => {
                // set Vx = Vx SHL 1, store dropped bit in VF
            }
            (9, _, _, 0) => {
                // skip next instruction if Vx != Vy
            }
            (0xA, _, _, _) => {
                // set I = NNN
            }
            (0xB, _, _, _) => {
                // jump to address NNN + V0
            }
            (0xC, _, _, _) => {
                // set Vx = random number AND NN
            }
            (0xD, _, _, _) => {
                // draw sprite at (Vx, Vy) with width 8 pixels and height N pixels
            }
            (0xE, _, 9, 0xE) => {
                // skip next instruction if key with the value of Vx is pressed
            }
            (0xE, _, 0xA, 1) => {
                // skip next instruction if key with the value of Vx is not pressed
            }
            (0xF, _, 0, 7) => {
                // set Vx = delay timer value
            }
            (0xF, _, 0, 0xA) => {
                // wait for a key press, then store the value of the key in Vx
            }
            (0xF, _, 1, 5) => {
                // set delay timer = Vx
            }
            (0xF, _, 1, 8) => {
                // set sound timer = Vx
            }
            (0xF, _, 1, 0xE) => {
                // set I = I + Vx
            }
            (0xF, _, 2, 9) => {
                // set I = location of sprite for digit Vx
            }
            (0xF, _, 3, 3) => {
                // store BCD representation of Vx in memory locations I, I+1, and I+2
            }
            (0xF, _, 5, 5) => {
                // store registers V0 through Vx in memory starting at location I
            }
            (0xF, _, 6, 5) => {
                // load registers V0 through Vx from memory starting at location I
            }
            (_, _, _, _) => {
                // unimplemented opcode
            }
        }
    }
}
