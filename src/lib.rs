//! # chiphuit
//!
//! The `chiphuit` crate provides a [Chip8](https://en.wikipedia.org/wiki/CHIP-8)
//! emulator able to run in any web browser capable of executing WASM.
//!
//!
//! ## Rendering
//!
//! `chiphuit` uses the [Canvas API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)
//! to render the emulator's screen.
//!
//! ## Features
//!
//! - `chiphuit` runs the ROM supplied by the user through the UI , and allows
//! hotswapping the ROM at runtime.
//!
//! - `chiphuit` also has a breakpoint feature that allows the user to pause
//! the `Emulator` at any time.
//!
//! - `chiphuit` displays the `Emulator` variables next to the screen in order
//! to see its state at runtime.
//!
//! - `chiphuit` provides 2 ways to handle user input: A player can click the
//! virtual keypad on the UI to play, or use its own keyboard.

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod audio;
mod cpu;
mod debugger;
mod graphics;
mod input;
mod utils;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]

static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
/// Main function that initializes the emulator, the keypad, and the screen
/// before inserting the ROM in the Emulator to play.
pub fn main_wasm() -> Result<(), JsValue> {
    utils::set_document();
    let canvas = graphics::set_canvas();
    let audio_context = audio::FmOsc::new().unwrap();

    let mut emulator = cpu::Emulator::new();
    emulator.load_font();

    let debugger = debugger::Debugger::new();
    debugger.set_debugger(&emulator);

    input::set_keypad(&emulator.keypad);
    input::set_breakpoint(&emulator.running);
    input::set_file_upload_button();
    input::set_file_reader(&emulator.rom_buffer);
    input::set_debug();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let t1 = Rc::new(RefCell::new(None));
    *t1.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        graphics::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    // EVENT LOOP
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        utils::set_timeout(t1.borrow().as_ref().unwrap(), 30);

        if *emulator.running.borrow() {
            for _ in 0..10 {
                emulator.cycle();
                emulator.update_emulator_state(&debugger.element.rows());
                debugger.get_serialized_current_snapshot(&emulator);
                debugger.handle_tracing(&emulator);
                audio::sound(&mut emulator, &audio_context);
            }
            graphics::draw_screen(&canvas, emulator.screen);
            emulator.handle_snapshot_hotswap();
            emulator.handle_rom_hotswap();
        }
    }) as Box<dyn FnMut()>));

    graphics::request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
