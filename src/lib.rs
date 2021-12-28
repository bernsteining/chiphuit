use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod cpu;
mod graphics;
mod input;
mod utils;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]

static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
/// Main function that initializes the emulator, the keypad, and the screen
/// before inserting the ROM in the Emulator to play.
pub fn main() -> Result<(), JsValue> {
    let document = web_sys::window()
        .unwrap()
        .document()
        .expect("should have a document.");

    let context = graphics::set_canvas(&document);

    let emulator_state = graphics::set_emulator_state(&document);

    let k = Rc::new(RefCell::new([false; 16]));
    let b = Rc::new(RefCell::new(false));
    let v = Rc::new(RefCell::new(Vec::new()));

    input::set_keypad(&document, &k);
    input::set_breakpoint(&document, &b);
    input::get_file_reader(&document, &v);

    let k2 = Rc::clone(&k);
    let b2 = Rc::clone(&b);
    let game = Rc::clone(&v);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let t1 = Rc::new(RefCell::new(None));

    *t1.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        graphics::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let mut emulator = cpu::Emulator::new();
    emulator.load_font();

    // EVENT LOOP
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        utils::set_timeout(t1.borrow().as_ref().unwrap(), 20);

        emulator.keypad = *k2.borrow_mut();
        emulator.running = *b2.borrow_mut();

        if !game.borrow().is_empty() {
            emulator.hotswap(game.borrow().clone());
            game.borrow_mut().clear();
        }

        if emulator.running {
            for _ in 0..10 {
                emulator.cycle();
            }
        }

        emulator_state.set_inner_html(&emulator.to_string());

        graphics::draw_screen(&context, emulator.screen);
    }) as Box<dyn FnMut()>));

    graphics::request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
