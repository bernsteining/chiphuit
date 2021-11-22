use js_sys::Math::random;
use std::fmt;
use wasm_bindgen::prelude::*;

use web_sys::console;

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

pub struct OpCode {
    pub first_nibble: u8,
    pub second_nibble: u8,
    pub third_nibble: u8,
    pub fourth_nibble: u8,
}

//write cpu struct with impls
pub struct Emulator {
    pub current_opcode: OpCode,
    memory: [u8; 4096],

    //regs
    registers: [u8; 16],
    index_register: u16,
    program_counter: u16,

    //display
    pub screen: [bool; 64 * 32],

    //stack related
    pub stack: [u16; 16],
    stack_pointer: usize,

    //timers
    delay_timer: u8,
    sound_timer: u8,

    //input
    keypad: [bool; 16],
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            current_opcode: OpCode {
                first_nibble: 0 as u8,
                second_nibble: 0 as u8,
                third_nibble: 0 as u8,
                fourth_nibble: 0 as u8,
            },
            memory: [0; 4096],

            //regs
            registers: [0; 16],
            index_register: 0,
            program_counter: 512,

            //display
            screen: [false; 64 * 32],

            //stack related
            stack: [0; 16],
            stack_pointer: 0,

            //timers
            delay_timer: 0,
            sound_timer: 0,

            //input
            keypad: [false; 16],
        }
    }

    pub fn load_font(&mut self) {
        self.memory[0..80].copy_from_slice(&FONTS);
    }

    pub fn load_game(&mut self, game: Vec<u8>) {
        self.memory[512..512 + game.len()].copy_from_slice(&game);
    }

    fn update_timers(&mut self) {
        //update timers
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

    pub fn update_key_press(&mut self, key: String) {
        match key.as_str() {
            "A" => self.keypad[10] = true,
            "B" => self.keypad[11] = true,
            "C" => self.keypad[12] = true,
            "D" => self.keypad[13] = true,
            "E" => self.keypad[14] = true,
            "F" => self.keypad[15] = true,
            _ => self.keypad[key.parse::<usize>().unwrap()] = true,
        }
    }

    fn fetch_opcode(&self) -> u16 {
        (self.memory[self.program_counter as usize] as u16) << 8
            | self.memory[(self.program_counter as usize + 1) as usize] as u16
    }

    // utils for factorization / readability
    fn get_third_and_fourth_nibbles_inline(&mut self) -> u8 {
        return self.current_opcode.third_nibble << 4 | self.current_opcode.fourth_nibble;
    }

    fn get_second_third_fourth_nibbles_inline(&mut self) -> u16 {
        ((self.current_opcode.second_nibble as u16) << 8
            | (self.current_opcode.third_nibble as u16) << 4
            | self.current_opcode.fourth_nibble as u16)
            & 0x0FFF
    }

    fn get_vx(&mut self) -> u8 {
        self.registers[self.current_opcode.second_nibble as usize]
    }

    fn get_vy(&mut self) -> u8 {
        self.registers[self.current_opcode.third_nibble as usize]
    }

    // fn get_three_last_nibbles(&mut self) -> u16 {}
    fn skip_next_instruction(&mut self) {
        self.program_counter += 2;
    }

    // Calls machine code routine (RCA 1802 for COSMAC VIP) at
    // address NNN. Not necessary for most ROMs.
    fn _0NNN(&mut self) {}

    // Clears the screen.
    fn _00E0(&mut self) {
        self.screen = [false; 64 * 32];
    }

    // Returns from a subroutine.
    // return;
    fn _00EE(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer];
    }

    // Jumps to address NNN.
    // goto NNN.
    fn _1NNN(&mut self) {
        self.program_counter = self.get_second_third_fourth_nibbles_inline();
    }

    // Calls subroutine at NNN.
    // *(0xNNN)()
    fn _2NNN(&mut self) {
        self.stack[self.stack_pointer] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = self.get_second_third_fourth_nibbles_inline();
    }

    // Skips the next instruction if VX equals NN.
    // (Usually the next instruction is a jump to skip a code block)
    // if (Vx == NN)
    fn _3XNN(&mut self) {
        if self.get_vx() == self.get_third_and_fourth_nibbles_inline() {
            self.skip_next_instruction();
        }
    }

    // Skips the next instruction if VX does not equal NN.
    // (Usually the next instruction is a jump to skip a code block);
    // if (Vx != NN)
    fn _4XNN(&mut self) {
        if self.get_vx() != self.get_third_and_fourth_nibbles_inline() {
            self.skip_next_instruction();
        }
    }

    // Skips the next instruction if VX equals VY.
    // (Usually the next instruction is a jump to skip a code block).
    // if (Vx == Vy)
    fn _5XY0(&mut self) {
        if self.get_vx() == self.get_vy() {
            self.skip_next_instruction();
        }
    }

    // Sets VX to NN.
    // Vx = N
    fn _6XNN(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] =
            self.get_third_and_fourth_nibbles_inline();
    }

    // Adds NN to VX. (Carry flag is not changed);
    // Vx += NN
    fn _7XNN(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] +=
            self.get_third_and_fourth_nibbles_inline();
    }

    // Sets VX to the value of VY.
    // Vx = Vy
    fn _8XY0(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] = self.get_vy();
    }

    // Sets VX to VX or VY. (Bitwise OR operation).
    // Vx |= Vy
    fn _8XY1(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] |= self.get_vy();
    }

    // Sets VX to VX and VY. (Bitwise AND operation).
    // Vx &= Vy
    fn _8XY2(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] &= self.get_vy();
    }

    // Sets VX to VX xor VY.
    // Vx ^= Vy
    fn _8XY3(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] ^= self.get_vy();
    }

    // Adds VY to VX. VF is set to 1 when there's a carry,
    // and to 0 when there is not.
    // Vx += Vy
    fn _8XY4(&mut self) {
        let sum = (self.get_vx() + self.get_vy()) as u16;
        if sum > 255 {
            self.registers[15] = 1;
        }
        self.registers[self.current_opcode.second_nibble as usize] += self.get_vy();
    }

    // VY is subtracted from VX. VF is set to 0 when there's a borrow,
    // and 1 when there is not.
    // Vx -= Vy
    fn _8XY5(&mut self) {
        let substraction = (self.get_vx() - self.get_vy()) as i8;
        if substraction < 0 {
            self.registers[15] = 0;
        } else {
            self.registers[15] = 1;
        }
        self.registers[self.current_opcode.second_nibble as usize] = substraction as u8;
    }

    // Stores the least significant bit of VX in VF and then shifts
    // VX to the right by 1.
    // Vx >>= 1
    fn _8XY6(&mut self) {
        self.registers[15] = 00000001u8 & self.get_vx();
        self.registers[self.current_opcode.second_nibble as usize] >>= 1;
    }

    // Sets VX to VY minus VX. VF is set to 0 when there's a borrow,
    // and 1 when there is not.
    // Vx = Vy - Vx
    fn _8XY7(&mut self) {
        let substraction = (self.get_vy() - self.get_vx()) as i8;
        if substraction < 0 {
            self.registers[15] = 0;
            self.registers[self.current_opcode.second_nibble as usize] = substraction as u8;
        } else {
            self.registers[15] = 1;
            self.registers[self.current_opcode.second_nibble as usize] = substraction as u8;
        }
    }

    // Stores the most significant bit of VX in VF
    // and then shifts VX to the left by 1.
    // Vx <<= 1
    fn _8XYE(&mut self) {
        self.registers[15] = 128 & self.get_vx();
        self.registers[self.current_opcode.second_nibble as usize] <<= 1;
    }

    // Skips the next instruction if VX does not equal VY.
    // (Usually the next instruction is a jump to skip a code block)
    // if (Vx != Vy)
    fn _9XY0(&mut self) {
        if self.get_vx() != self.get_vy() {
            self.skip_next_instruction();
        }
    }

    // Sets I to the address NNN.
    // I = NNN
    fn ANNN(&mut self) {
        self.index_register = self.get_second_third_fourth_nibbles_inline();
    }

    // Jumps to the address NNN plus V0.
    // PC = V0 + NNN
    fn BNNN(&mut self) {
        self.program_counter =
            self.registers[0] as u16 + self.get_second_third_fourth_nibbles_inline();
    }

    // Sets VX to the result of a bitwise and operation on a random number
    // (Typically: 0 to 255) and NN. Vx = rand() & NN
    fn CXNN(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] =
            ((random() * 255.0) as u8) & self.get_third_and_fourth_nibbles_inline();
    }

    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and
    // a height of N pixels. Each row of 8 pixels is read as bit-coded starting
    // from memory location I; I value does not change after the execution of
    // this instruction. As described above, VF is set to 1 if any screen pixels
    // are flipped from set to unset when the sprite is drawn, and to 0 if that
    // does not happen
    // draw(Vx, Vy, N)
    fn DXYN(&mut self) {
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

                if previous_state == true && self.screen[index] == false {
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

    // Skips the next instruction if the key stored in VX is pressed.
    // (Usually the next instruction is a jump to skip a code block);
    // if (key() == Vx)
    fn EX9E(&mut self) {
        if self.keypad[self.get_vx() as usize] {
            self.skip_next_instruction();
        }
    }

    // Skips the next instruction if the key stored in VX is not pressed.
    // (Usually the next instruction is a jump to skip a code block).
    // if (key() != Vx)
    fn EXA1(&mut self) {
        if !self.keypad[self.get_vx() as usize] {
            self.skip_next_instruction();
        }
    }

    // Sets VX to the value of the delay timer.
    // Vx = get_delay()
    fn FX07(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] = self.delay_timer;
    }

    // A key press is awaited, and then stored in VX. (Blocking Operation.
    // All instruction halted until next key event);
    // Vx = get_key()
    fn FX0A(&mut self) {
        // self.program_counter -= 2;
        // if self.keypad[self.get_vx() as usize] == true {
        //     self.registers[self.current.second_nibble as usize] =  ;
        //     self.program_counter +=2;
        // }
    }

    // Sets the delay timer to VX.
    // delay_timer(Vx)
    fn FX15(&mut self) {
        self.delay_timer = self.get_vx();
    }

    // Sets the sound timer to VX.
    // sound_timer(Vx)
    fn FX18(&mut self) {
        self.sound_timer = self.get_vx();
    }

    // Adds VX to I. VF is not affected.
    // I += Vx
    fn FX1E(&mut self) {
        self.index_register += self.get_vx() as u16;
    }

    // Sets I to the location of the sprite for the character in VX.
    // Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    // I = sprite_addr[Vx]
    fn FX29(&mut self) {
        self.index_register = self.get_vx() as u16 * 5;
    }

    // Stores the binary-coded decimal representation of VX, with the most
    // significant of three digits at the address in I, the middle digit at I
    // plus 1, and the least significant digit at I plus 2. (In other words, take
    // the decimal representation of VX, place the hundreds digit in memory at
    // location in I, the tens digit at location I+1, and the ones digit at
    // location I+2.).
    // set_BCD(Vx)
    // *(I+0) = BCD(3);
    // *(I+1) = BCD(2);
    // *(I+2) = BCD(1);
    fn FX33(&mut self) {
        self.memory[self.index_register as usize] = self.get_vx() / 100;
        self.memory[self.index_register as usize + 1] = (self.get_vx() / 10) % 10;
        self.memory[self.index_register as usize + 2] = self.get_vx() % 10;
    }

    // Stores from V0 to VX (including VX) in memory, starting at address I.
    // The offset from I is increased by 1 for each value written, but I
    // itself is left unmodified
    // reg_dump(Vx, &I)
    fn FX55(&mut self) {
        for i in 0..self.current_opcode.second_nibble + 1 {
            self.memory[(self.index_register + i as u16) as usize] = self.registers[i as usize];
        }
    }

    // Fills from V0 to VX (including VX) with values from memory, starting at
    // address I. The offset from I is increased by 1 for each value written,
    // but I itself is left unmodified.
    // reg_load(Vx, &I)
    fn FX65(&mut self) {
        for i in 0..self.current_opcode.second_nibble + 1 {
            self.registers[i as usize] = self.memory[(self.index_register + i as u16) as usize];
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();

        self.process_opcode(opcode);

        self.update_timers();
    }

    pub fn process_opcode(&mut self, opcode: u16) {
        // use nom to parse opcodes?
        let first_nibble = ((opcode & 0xF000) >> 12) as u8;
        let second_nibble = ((opcode & 0x0F00) >> 8) as u8;
        let third_nibble = ((opcode & 0x00F0) >> 4) as u8;
        let fourth_nibble = (opcode & 0x000F) as u8;

        self.current_opcode = OpCode {
            first_nibble,
            second_nibble,
            third_nibble,
            fourth_nibble,
        };

        self.program_counter += 2;

        match (first_nibble, second_nibble, third_nibble, fourth_nibble) {
            (0, 0, 0xE, 0xE) => self._00EE(),
            (0, 0, 0xE, 0) => self._00E0(),
            (0, _, _, _) => self._0NNN(),
            (1, _, _, _) => self._1NNN(),
            (2, _, _, _) => self._2NNN(),
            (3, _, _, _) => self._3XNN(),
            (4, _, _, _) => self._4XNN(),
            (5, _, _, 0) => self._5XY0(),
            (6, _, _, _) => self._6XNN(),
            (7, _, _, _) => self._7XNN(),
            (8, _, _, 0) => self._8XY0(),
            (8, _, _, 1) => self._8XY1(),
            (8, _, _, 2) => self._8XY2(),
            (8, _, _, 3) => self._8XY3(),
            (8, _, _, 4) => self._8XY4(),
            (8, _, _, 5) => self._8XY5(),
            (8, _, _, 6) => self._8XY6(),
            (8, _, _, 7) => self._8XY7(),
            (8, _, _, 0xE) => self._8XYE(),
            (9, _, _, 0) => self._9XY0(),
            (0xA, _, _, _) => self.ANNN(),
            (0xB, _, _, _) => self.BNNN(),
            (0xC, _, _, _) => self.CXNN(),
            (0xD, _, _, _) => self.DXYN(),
            (0xE, _, 9, 0xE) => self.EX9E(),
            (0xE, _, 0xA, 1) => self.EXA1(),
            (0xF, _, 0, 7) => self.FX07(),
            (0xF, _, 0, 0xA) => self.FX0A(),
            (0xF, _, 1, 5) => self.FX15(),
            (0xF, _, 1, 8) => self.FX18(),
            (0xF, _, 1, 0xE) => self.FX1E(),
            (0xF, _, 2, 9) => self.FX29(),
            (0xF, _, 3, 3) => self.FX33(),
            (0xF, _, 5, 5) => self.FX55(),
            (0xF, _, 6, 5) => self.FX65(),
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

// Display trait to print Emulator state next to the screen
impl fmt::Display for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"<table style="border-spacing: 30px 5px;"><tr><th>variable</th><th>value</th></tr>"#,
        );

        write!(
            f,
            "<tr><td>current opcode</td><td>{:X}{:X}{:X}{:X}</td></tr>",
            self.current_opcode.first_nibble,
            self.current_opcode.second_nibble,
            self.current_opcode.third_nibble,
            self.current_opcode.fourth_nibble
        );

        write!(
            f,
            "<tr><td>registers</td><td>{}</td></tr>",
            self.registers
                .iter()
                .map(|&x| format!("{:3X},", x))
                .collect::<String>()
        );

        write!(
            f,
            "<tr><td>index register</td> <td>{}</td></tr>",
            self.index_register
        );
        write!(
            f,
            "<tr><td>program counter</td> <td>{}</td></tr>",
            self.program_counter
        );
        write!(
            f,
            "<tr><td>delay timer</td> <td>{}</td> </tr>",
            self.delay_timer
        );

        write!(
            f,
            "<tr><td>sound timer</td> <td>{}</td></tr>",
            self.sound_timer
        );

        write!(
            f,
            "<tr><td>stack pointer</td> <td>{}</td></tr>",
            self.stack_pointer
        );
        write!(
            f,
            "<tr><td>stack</td> <td>{}<td></tr>",
            self.stack
                .iter()
                .map(|&x| format!("{},", x))
                .collect::<String>()
        );
        write!(f, "</table>",)
    }
}

// returns an array of booleans according to a byte's bits
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
