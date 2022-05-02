//! # A module to emulate the chip8 architecture, and process its opcodes logic.
use js_sys::Math::random;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlCollection, HtmlTableRowElement};

/// Chip8 fonts set.
pub const FONTS: [u8; 80] = [
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

/// A struct to access a chip8's opcodes nibbles.
pub struct OpCode {
    pub first_nibble: u8,
    pub second_nibble: u8,
    pub third_nibble: u8,
    pub fourth_nibble: u8,
}

impl OpCode {
    /// Util function to read the 3rd and 4th nibbles of the opcode in a single
    /// u8.
    fn get_third_and_fourth_nibbles_inline(&mut self) -> u8 {
        self.third_nibble << 4 | self.fourth_nibble
    }

    /// Util function to read the 2nd, 3rd and 4th nibbles of the opcode
    /// in a single u16.
    fn get_second_third_fourth_nibbles_inline(&mut self) -> u16 {
        ((self.second_nibble as u16) << 8
            | (self.third_nibble as u16) << 4
            | self.fourth_nibble as u16)
            & 0x0FFF
    }
}

/// Display Trait to print the `Emulator`'s current `OpCode` in the debugger.
impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:X}{:X}{:X}{:X}",
            self.first_nibble, self.second_nibble, self.third_nibble, self.fourth_nibble
        )
    }
}

///  A struct containing all the fields necessary to emulate chip8.
pub struct Emulator {
    pub current_opcode: OpCode,
    memory: [u8; 4096],

    registers: [u8; 16],
    index_register: u16,
    program_counter: u16,

    pub screen: [bool; 64 * 32],

    pub stack: [u16; 16],
    stack_pointer: usize,

    delay_timer: u8,
    sound_timer: u8,

    pub keypad: Rc<RefCell<[bool; 16]>>,

    pub rom_buffer: Rc<RefCell<Vec<u8>>>,

    pub running: Rc<RefCell<bool>>,
}

/// Print trait to display an `Emulator`'s specific fields into the debugger
/// during runtime.
pub trait Print {
    fn printables(&self) -> Vec<String>;
}

/// Print Trait for `Emulator` in order to display its internal
/// variables in the debugger during runtime.
impl Print for Emulator {
    fn printables(&self) -> Vec<String> {
        vec![
            format!("{}", self.current_opcode),
            self.registers
                .iter()
                .map(|&x| format!("{:3X},", x))
                .collect::<String>(),
            format!("{}", self.index_register),
            format!("{}", self.program_counter),
            format!("{}", self.delay_timer),
            format!("{}", self.sound_timer),
            format!("{}", self.stack_pointer),
            self.stack
                .iter()
                .map(|&x| format!("{},", x))
                .collect::<String>(),
            format!("{:?}", self.running.borrow()),
        ]
    }
}

impl Emulator {
    //! Creates a new empty `Emulator`.
    //!
    //! Returns an Emulator struct initialized with a memory buffer filled
    //! with 0s, a blank screen display, and a program counter set to 512 ready
    //! to process a chip8 ROM whenever a ROM is loaded into memory with the
    //! method load_game.
    pub fn new() -> Emulator {
        Emulator {
            current_opcode: OpCode {
                first_nibble: 0_u8,
                second_nibble: 0_u8,
                third_nibble: 0_u8,
                fourth_nibble: 0_u8,
            },
            memory: [0; 4096],

            registers: [0; 16],
            index_register: 0,
            program_counter: 512,

            screen: [false; 64 * 32],

            stack: [0; 16],
            stack_pointer: 0,

            delay_timer: 0,
            sound_timer: 0,

            keypad: Rc::new(RefCell::new([false; 16])),

            running: Rc::new(RefCell::new(false)),

            rom_buffer: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Update emulator state in the GUI.
    pub fn update_emulator_state(&self, emulator_state: &HtmlCollection) {
        // The index + 1 offset is to skip the first elements of the HtmlTable
        // entries which are static.
        for (index, printable) in self.printables().iter().enumerate() {
            emulator_state
                .get_with_index((index + 1) as u32)
                .unwrap()
                .dyn_into::<HtmlTableRowElement>()
                .unwrap()
                .cells()
                .item(1)
                .unwrap()
                .set_inner_html(printable);
        }
    }

    /// Loads the default font set into the `Emulator` instance's memory from
    /// offset 0 to 80.
    pub fn load_font(&mut self) {
        self.memory[0..80].copy_from_slice(&FONTS);
    }

    /// Loads the ROM into the `Emulator` instance's memory at offset
    /// 512.
    pub fn load_rom(&mut self) {
        let rom_length = self.rom_buffer.borrow().len();
        self.memory[512..512 + rom_length].copy_from_slice(&self.rom_buffer.borrow());
        self.rom_buffer.borrow_mut().clear();
    }

    /// Hotswaps the ROM intro the `Emulator` instance's memory at
    /// offset 512. This allows to change the game ran by the `Emulator` at
    /// runtime without reload the page. `Emulator` fields are reinitialized as
    /// if `Emulator::new()` was called in order to have a fresh `Emulator`
    /// state.
    pub fn hotswap(&mut self) {
        self.memory = [0; 4096];
        self.screen = [false; 64 * 32];
        self.registers = [0; 16];
        self.index_register = 0;
        self.program_counter = 512;
        self.stack = [0; 16];
        self.stack_pointer = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;

        self.load_rom();
    }

    /// Decrements `Emulator` timers.
    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }

        match self.sound_timer {
            1 => {
                println!("BEEP");
                self.sound_timer -= 1
            }
            _ => self.sound_timer -= 1,
        }
    }

    /// Fetches `Emulator` opcodes from memory in order to process them.
    fn fetch_opcode(&mut self) {
        let opcode = (self.memory[self.program_counter as usize] as u16) << 8
            | self.memory[(self.program_counter as usize + 1) as usize] as u16;

        self.current_opcode = OpCode {
            first_nibble: ((opcode & 0xF000) >> 12) as u8,
            second_nibble: ((opcode & 0x0F00) >> 8) as u8,
            third_nibble: ((opcode & 0x00F0) >> 4) as u8,
            fourth_nibble: (opcode & 0x000F) as u8,
        };
    }

    /// Get the value of the Xth register with X being the value of the second
    /// nibble of the opcode.
    fn get_vx(&mut self) -> u8 {
        self.registers[self.current_opcode.second_nibble as usize]
    }

    /// Get the value of the Yth register with Y being the value of the third
    /// nibble of the opcode.
    fn get_vy(&mut self) -> u8 {
        self.registers[self.current_opcode.third_nibble as usize]
    }

    /// Skip the next instruction by incrementing the program counter by 2.
    fn skip_next_instruction(&mut self) {
        self.program_counter += 2;
    }

    /// Calls machine code routine (RCA 1802 for COSMAC VIP) at
    /// address NNN. Not necessary for most ROMs.
    fn _0nnn(&mut self) {}

    /// Clears the screen.
    fn _00e0(&mut self) {
        self.screen = [false; 64 * 32];
    }

    /// Returns from a subroutine.
    /// return;
    fn _00ee(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer];
    }

    /// Jumps to address NNN.
    /// goto NNN.
    fn _1nnn(&mut self) {
        self.program_counter = self.current_opcode.get_second_third_fourth_nibbles_inline();
    }

    /// Calls subroutine at NNN.
    /// *(0xNNN)()
    fn _2nnn(&mut self) {
        self.stack[self.stack_pointer] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = self.current_opcode.get_second_third_fourth_nibbles_inline();
    }

    /// Skips the next instruction if VX equals NN.
    /// (Usually the next instruction is a jump to skip a code block)
    /// if (vx == NN)
    fn _3xnn(&mut self) {
        if self.get_vx() == self.current_opcode.get_third_and_fourth_nibbles_inline() {
            self.skip_next_instruction();
        }
    }

    /// Skips the next instruction if VX does not equal NN.
    /// (Usually the next instruction is a jump to skip a code block);
    /// if (vx != NN)
    fn _4xnn(&mut self) {
        if self.get_vx() != self.current_opcode.get_third_and_fourth_nibbles_inline() {
            self.skip_next_instruction();
        }
    }

    /// Skips the next instruction if VX equals VY.
    /// (Usually the next instruction is a jump to skip a code block).
    /// if (vx == vy)
    fn _5xy0(&mut self) {
        if self.get_vx() == self.get_vy() {
            self.skip_next_instruction();
        }
    }

    /// Sets VX to NN.
    /// vx = N
    fn _6xnn(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] =
            self.current_opcode.get_third_and_fourth_nibbles_inline();
    }

    /// Adds NN to VX. (Carry flag is not changed);
    /// vx += NN
    fn _7xnn(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] +=
            self.current_opcode.get_third_and_fourth_nibbles_inline();
    }

    /// Sets VX to the value of VY.
    /// vx = vy
    fn _8xy0(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] = self.get_vy();
    }

    /// Sets VX to VX or VY. (Bitwise OR operation).
    /// vx |= vy
    fn _8xy1(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] |= self.get_vy();
    }

    /// Sets VX to VX and VY. (Bitwise AND operation).
    /// vx &= vy
    fn _8xy2(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] &= self.get_vy();
    }

    /// Sets VX to VX xor VY.
    /// vx ^= vy
    fn _8xy3(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] ^= self.get_vy();
    }

    /// Adds VY to VX. VF is set to 1 when there's a carry,
    /// and to 0 when there is not.
    /// vx += vy
    fn _8xy4(&mut self) {
        let sum = (self.get_vx() + self.get_vy()) as u16;
        if sum > 255 {
            self.registers[15] = 1;
        }
        self.registers[self.current_opcode.second_nibble as usize] += self.get_vy();
    }

    /// VY is subtracted from VX. VF is set to 0 when there's a borrow,
    /// and 1 when there is not.
    /// vx -= vy
    fn _8xy5(&mut self) {
        let substraction = (self.get_vx() - self.get_vy()) as i8;
        if substraction < 0 {
            self.registers[15] = 0;
        } else {
            self.registers[15] = 1;
        }
        self.registers[self.current_opcode.second_nibble as usize] = substraction as u8;
    }

    /// Stores the least significant bit of VX in VF and then shifts
    /// VX to the right by 1.
    /// vx >>= 1
    fn _8xy6(&mut self) {
        self.registers[15] = 1u8 & self.get_vx();
        self.registers[self.current_opcode.second_nibble as usize] >>= 1;
    }

    /// Sets VX to VY minus VX. VF is set to 0 when there's a borrow,
    /// and 1 when there is not.
    /// vx = vy - vx
    fn _8xy7(&mut self) {
        let substraction = (self.get_vy() - self.get_vx()) as i8;
        self.registers[15] = match substraction < 0 {
            true => 0,
            false => 1,
        };
        self.registers[self.current_opcode.second_nibble as usize] = substraction as u8;
    }

    /// Stores the most significant bit of VX in VF
    /// and then shifts VX to the left by 1.
    /// vx <<= 1
    fn _8xye(&mut self) {
        self.registers[15] = 128 & self.get_vx();
        self.registers[self.current_opcode.second_nibble as usize] <<= 1;
    }

    /// Skips the next instruction if VX does not equal VY.
    /// (Usually the next instruction is a jump to skip a code block)
    /// if (vx != vy)
    fn _9xy0(&mut self) {
        if self.get_vx() != self.get_vy() {
            self.skip_next_instruction();
        }
    }

    /// Sets I to the address NNN.
    /// I = NNN
    fn annn(&mut self) {
        self.index_register = self.current_opcode.get_second_third_fourth_nibbles_inline();
    }

    /// Jumps to the address NNN plus V0.
    /// PC = V0 + NNN
    fn bnnn(&mut self) {
        self.program_counter =
            self.registers[0] as u16 + self.current_opcode.get_second_third_fourth_nibbles_inline();
    }

    /// Sets VX to the result of a bitwise and operation on a random number
    /// (Typically: 0 to 255) and NN. vx = rand() & NN
    fn cxnn(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] =
            ((random() * 255.0) as u8) & self.current_opcode.get_third_and_fourth_nibbles_inline();
    }

    /// Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and
    /// a height of N pixels. Each row of 8 pixels is read as bit-coded starting
    /// from memory location I; I value does not change after the execution of
    /// this instruction. As described above, VF is set to 1 if any screen pixels
    /// are flipped from set to unset when the sprite is drawn, and to 0 if that
    /// does not happen
    /// draw(vx, vy, N)
    fn dxyn(&mut self) {
        let height = self.current_opcode.fourth_nibble;
        let x = self.get_vx();
        let y = self.get_vy();
        let mut collision = false;

        for row in 0..height {
            let row_pixels: [bool; 8] =
                u8_to_bools(self.memory[(self.index_register as usize) + row as usize]);

            for i in 0..8 {
                let index = (x as u16 + i as u16 + (y as u16 + row as u16) * 64) as usize;

                let previous_state = self.screen[index];
                self.screen[index] ^= row_pixels[i as usize];

                if previous_state && !self.screen[index] {
                    collision = true;
                }
            }
        }
        if collision {
            self.registers[15] = 1;
        } else {
            self.registers[15] = 0;
        }
    }

    /// Skips the next instruction if the key stored in VX is pressed.
    /// (Usually the next instruction is a jump to skip a code block);
    /// if (key() == vx)
    fn ex9e(&mut self) {
        if self.keypad.borrow()[self.registers[self.current_opcode.second_nibble as usize] as usize]
        {
            self.skip_next_instruction();
        }
    }

    /// Skips the next instruction if the key stored in VX is not pressed.
    /// (Usually the next instruction is a jump to skip a code block).
    /// if (key() != vx)
    fn exa1(&mut self) {
        if !self.keypad.borrow()
            [self.registers[self.current_opcode.second_nibble as usize] as usize]
        {
            self.skip_next_instruction();
        }
    }

    /// Sets VX to the value of the delay timer.
    /// vx = get_delay()
    fn fx07(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] = self.delay_timer;
    }

    /// A key press is awaited, and then stored in VX. (Blocking Operation).
    /// All instruction halted until next key event);
    /// vx = get_key()
    fn fx0a(&mut self) {
        self.program_counter -= 2;
        if self.keypad.borrow()[self.registers[self.current_opcode.second_nibble as usize] as usize]
        {
            self.registers[self.current_opcode.second_nibble as usize] = self.get_vx();
            self.program_counter += 2;
        }
    }

    /// Sets the delay timer to VX.
    /// delay_timer(vx)
    fn fx15(&mut self) {
        self.delay_timer = self.get_vx();
    }

    /// Sets the sound timer to VX.
    /// sound_timer(vx)
    fn fx18(&mut self) {
        self.sound_timer = self.get_vx();
    }

    /// Adds VX to I. VF is not affected.
    /// I += vx
    fn fx1e(&mut self) {
        self.index_register += self.get_vx() as u16;
    }

    /// Sets I to the location of the sprite for the character in VX.
    /// Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    /// I = sprite_addr[vx]
    fn fx29(&mut self) {
        self.index_register = self.get_vx() as u16 * 5;
    }

    /// Stores the binary-coded decimal representation of VX, with the most
    /// significant of three digits at the address in I, the middle digit at I
    /// plus 1, and the least significant digit at I plus 2. (In other words, take
    /// the decimal representation of VX, place the hundreds digit in memory at
    /// location in I, the tens digit at location I+1, and the ones digit at
    /// location I+2.).
    /// set_BCD(vx)
    /// *(I+0) = BCD(3);
    /// *(I+1) = BCD(2);
    /// *(I+2) = BCD(1);
    fn fx33(&mut self) {
        self.memory[self.index_register as usize] = self.get_vx() / 100;
        self.memory[self.index_register as usize + 1] = (self.get_vx() / 10) % 10;
        self.memory[self.index_register as usize + 2] = self.get_vx() % 10;
    }

    /// Stores from V0 to VX (including VX) in memory, starting at address I.
    /// The offset from I is increased by 1 for each value written, but I
    /// itself is left unmodified
    /// reg_dump(vx, &I)
    fn fx55(&mut self) {
        for i in 0..self.current_opcode.second_nibble + 1 {
            self.memory[(self.index_register + i as u16) as usize] = self.registers[i as usize];
        }
    }

    /// Fills from V0 to VX (including VX) with values from memory, starting at
    /// address I. The offset from I is increased by 1 for each value written,
    /// but I itself is left unmodified.
    /// reg_load(vx, &I)
    fn fx65(&mut self) {
        for i in 0..self.current_opcode.second_nibble + 1 {
            self.registers[i as usize] = self.memory[(self.index_register + i as u16) as usize];
        }
    }

    pub fn cycle(&mut self) {
        self.fetch_opcode();

        self.process_opcode();

        self.update_timers();
    }

    pub fn process_opcode(&mut self) {
        self.program_counter += 2;

        match (
            self.current_opcode.first_nibble,
            self.current_opcode.second_nibble,
            self.current_opcode.third_nibble,
            self.current_opcode.fourth_nibble,
        ) {
            (0, 0, 0xE, 0xE) => self._00ee(),
            (0, 0, 0xE, 0) => self._00e0(),
            (0, _, _, _) => self._0nnn(),
            (1, _, _, _) => self._1nnn(),
            (2, _, _, _) => self._2nnn(),
            (3, _, _, _) => self._3xnn(),
            (4, _, _, _) => self._4xnn(),
            (5, _, _, 0) => self._5xy0(),
            (6, _, _, _) => self._6xnn(),
            (7, _, _, _) => self._7xnn(),
            (8, _, _, 0) => self._8xy0(),
            (8, _, _, 1) => self._8xy1(),
            (8, _, _, 2) => self._8xy2(),
            (8, _, _, 3) => self._8xy3(),
            (8, _, _, 4) => self._8xy4(),
            (8, _, _, 5) => self._8xy5(),
            (8, _, _, 6) => self._8xy6(),
            (8, _, _, 7) => self._8xy7(),
            (8, _, _, 0xE) => self._8xye(),
            (9, _, _, 0) => self._9xy0(),
            (0xA, _, _, _) => self.annn(),
            (0xB, _, _, _) => self.bnnn(),
            (0xC, _, _, _) => self.cxnn(),
            (0xD, _, _, _) => self.dxyn(),
            (0xE, _, 9, 0xE) => self.ex9e(),
            (0xE, _, 0xA, 1) => self.exa1(),
            (0xF, _, 0, 7) => self.fx07(),
            (0xF, _, 0, 0xA) => self.fx0a(),
            (0xF, _, 1, 5) => self.fx15(),
            (0xF, _, 1, 8) => self.fx18(),
            (0xF, _, 1, 0xE) => self.fx1e(),
            (0xF, _, 2, 9) => self.fx29(),
            (0xF, _, 3, 3) => self.fx33(),
            (0xF, _, 5, 5) => self.fx55(),
            (0xF, _, 6, 5) => self.fx65(),
            _ => {
                self.screen = [true; 2048];
                console::log_1(
                    &format!(
                        "Unknown opcode: {:X}{:X}{:X}{:X}, instructions unclear, got stuck in the washing machine.",
                        self.current_opcode.first_nibble,
                        self.current_opcode.second_nibble,
                        self.current_opcode.third_nibble,
                        self.current_opcode.fourth_nibble
                    )
                    .into(),
                );
            }
        }
    }
}

/// Returns an array of booleans according to a byte's bits.
fn u8_to_bools(byte: u8) -> [bool; 8] {
    [
        0b10000000 & byte == 0b10000000,
        0b01000000 & byte == 0b01000000,
        0b00100000 & byte == 0b00100000,
        0b00010000 & byte == 0b00010000,
        0b00001000 & byte == 0b00001000,
        0b00000100 & byte == 0b00000100,
        0b00000010 & byte == 0b00000010,
        0b00000001 & byte == 0b00000001,
    ]
}
