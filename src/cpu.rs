use js_sys::Math::random;
use wasm_bindgen::prelude::*;

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
    first_nibble: u8,
    second_nibble: u8,
    third_nibble: u8,
    fourth_nibble: u8,
}

//write cpu struct with impls
pub struct Emulator {
    current_opcode: OpCode,
    memory: [u8; 4096],

    //regs
    registers: [u8; 16],
    index_register: u16,
    program_counter: u16,

    //display
    screen: [bool; 64 * 32],

    //stack related  padding-right: 960px;
    stack: [usize; 16],
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
            program_counter: 0,

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

    fn update_key_press(&mut self, key: String) {
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

    fn cycle(&mut self) {
        let opcode = self.fetch_opcode();

        self.update_timers();
    }

    fn process_opcode(&mut self, opcode: u16) {
        // use nom to parse opcodes?
        let first_nibble = (opcode & 0xF000 >> 12) as u8;
        let second_nibble = (opcode & 0x0F00 >> 8) as u8;
        let third_nibble = (opcode & 0x00F0 >> 4) as u8;
        let fourth_nibble = (opcode & 0x000F) as u8;

        self.current_opcode = OpCode {
            first_nibble,
            second_nibble,
            third_nibble,
            fourth_nibble,
        };

        match (first_nibble, second_nibble, third_nibble, fourth_nibble) {
            (0, _, _, _) => println!("call machine code routine"),
            (0, 0, 0xE, 0xE) => self.clear_screen(),
            (1, _, _, _) => self.goto(),
            (2, _, _, _) => self.call_subroutine(),
            (3, _, _, _) => self.skip_next_instruction_if_vx_equals_34(),
            (4, _, _, _) => self.skip_next_instruction_if_not_vx_equals_34(),
            (5, _, _, 0) => self.skip_next_instruction_if_vv_equals_vy(),
            (6, _, _, _) => self.set_vx_to_34(),
            (7, _, _, _) => self.add_34_to_vx(),
            (8, _, _, 0) => self.set_vx_as_vy(),
            (8, _, _, 1) => self.set_vx_as_vx_or_vy(),
            (8, _, _, 2) => self.set_vx_as_vx_and_vy(),
            (8, _, _, 3) => self.set_vx_as_vx_xor_vy(),
            (8, _, _, 4) => self.incr_vx_with_vy_and_handle_carry_flag(),
            (8, _, _, 5) => self.decr_vx_with_vy_and_handle_carry_flag(),
            (8, _, _, 6) => self.store_vx_lsb_in_vf_and_shift_vx_to_the_right(),
            (8, _, _, 7) => self.store_v3_minus_v2_in_v2_and_handle_carry_flag(),
            (8, _, _, 0xE) => self.store_v2_lsb_in_vf_and_shift_v2_to_the_right(),
            (9, _, _, 0) => self.skip_next_instruction_if_vx_neq_v2(),
            (0xA, _, _, _) => self.set_i_as_234(),
            (0xB, _, _, _) => self.set_pc_as_v0_plus_234(),
            (0xC, _, _, _) => self.set_vx_as_rand_and_34(),
            (0xD, _, _, _) => self.draw_sprite_at_vx_vy_of_4_pixels(),
            (0xE, _, 9, 0xE) => self.skip_next_instruction_if_vx_key_pressed(),
            (0xE, _, 0xA, 1) => self.skip_next_instruction_if_vx_key_isnt_pressed(),
            (0xF, _, 0, 7) => self.set_vx_as_delay_timer(),
            (0xF, _, 0, 0xA) => self.wait_for_keypress_then_store_it_in_vx(),
            (0xF, _, 1, 5) => self.set_delay_timer_as_vx(),
            (0xF, _, 1, 8) => self.set_sound_timer_as_vx(),
            (0xF, _, 1, 0xE) => self.increment_i_with_vx(),
            (0xF, _, 2, 9) => self.set_i_as_char_font_with_vx_index(),
            (0xF, _, 3, 3) => self.wtf(),
            (0xF, _, 5, 5) => self.store_v0_to_vx_in_memory_from_i(),
            (0xF, _, 6, 5) => self.fill_v0_to_vx_with_memory_from_i(),
            _ => {
                println!("Unknown opcode, instructions unclear, got stuck in the washing machine.")
            }
        }
    }

    // utils for factorization / readability

    fn get_third_and_fourth_nibbles_inline(&mut self) -> u8 {
        return self.current_opcode.third_nibble << 4 | self.current_opcode.fourth_nibble;
    }

    fn get_vx(&mut self) -> u8 {
        self.registers[self.current_opcode.second_nibble as usize]
    }

    fn get_vy(&mut self) -> u8 {
        self.registers[self.current_opcode.third_nibble as usize]
    }

    // fn get_three_last_nibbles(&mut self) -> u16 {}

    fn skip_next_instruction(&mut self) {}

    fn clear_screen(&mut self) {
        self.screen = [false; 64 * 32];
    }
    fn goto(&mut self) {}
    fn call_subroutine(&mut self) {}
    fn skip_next_instruction_if_vx_equals_34(&mut self) {
        if self.registers[self.current_opcode.second_nibble as usize]
            == self.get_third_and_fourth_nibbles_inline()
        {
            self.skip_next_instruction();
        }
    }
    fn skip_next_instruction_if_not_vx_equals_34(&mut self) {}
    fn skip_next_instruction_if_vv_equals_vy(&mut self) {}

    fn set_vx_to_34(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] =
            self.get_third_and_fourth_nibbles_inline();
    }

    fn add_34_to_vx(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] +=
            self.get_third_and_fourth_nibbles_inline();
    }

    fn set_vx_as_vy(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] =
            self.current_opcode.third_nibble;
    }

    fn set_vx_as_vx_or_vy(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] |= self.get_vy()
    }

    fn set_vx_as_vx_and_vy(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] &= self.get_vy()
    }

    fn set_vx_as_vx_xor_vy(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] ^= self.get_vy()
    }

    fn incr_vx_with_vy_and_handle_carry_flag(&mut self) {}
    fn decr_vx_with_vy_and_handle_carry_flag(&mut self) {}
    fn store_vx_lsb_in_vf_and_shift_vx_to_the_right(&mut self) {}
    fn store_v3_minus_v2_in_v2_and_handle_carry_flag(&mut self) {}
    fn store_v2_lsb_in_vf_and_shift_v2_to_the_right(&mut self) {}
    fn skip_next_instruction_if_vx_neq_v2(&mut self) {}
    fn set_i_as_234(&mut self) {}
    fn set_pc_as_v0_plus_234(&mut self) {}

    fn set_vx_as_rand_and_34(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] =
            ((random() * 255.0) as u8) & self.get_third_and_fourth_nibbles_inline()
    }

    fn draw_sprite_at_vx_vy_of_4_pixels(&mut self) {}
    fn skip_next_instruction_if_vx_key_pressed(&mut self) {}
    fn skip_next_instruction_if_vx_key_isnt_pressed(&mut self) {}

    fn set_vx_as_delay_timer(&mut self) {
        self.registers[self.current_opcode.second_nibble as usize] = self.delay_timer
    }

    fn wait_for_keypress_then_store_it_in_vx(&mut self) {}

    fn set_delay_timer_as_vx(&mut self) {
        self.delay_timer = self.get_vx()
    }

    fn set_sound_timer_as_vx(&mut self) {
        self.sound_timer = self.get_vx()
    }

    fn increment_i_with_vx(&mut self) {
        self.index_register += self.get_vx() as u16
    }

    fn set_i_as_char_font_with_vx_index(&mut self) {
        // self.index_register = FONTS[self.get_vx() as usize] as u16
    }
    fn wtf(&mut self) {}
    fn store_v0_to_vx_in_memory_from_i(&mut self) {}
    fn fill_v0_to_vx_with_memory_from_i(&mut self) {}
}

#[wasm_bindgen]
pub fn handle_input(key: String) {
    // logging on webpage
    // let keys = document().create_element("lol").unwrap();
    // keys.set_inner_html(&format!("<li>{}<li>", &key).to_string());
    // body().append_child(&keys).unwrap();

    //let text = format!("Keypress: {}", key);
}
