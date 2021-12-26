use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod cpu;
mod graphics;
mod input;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let context = graphics::get_context();
    let document = web_sys::window()
        .unwrap()
        .document()
        .expect("should have a document.");

    let emulator_state = graphics::set_emulator_state(&document);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let t1 = Rc::new(RefCell::new(None));

    *t1.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        graphics::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let mut emulator = cpu::Emulator::new();
    emulator.load_font();

    let k = Rc::new(RefCell::new([false; 16]));
    let b = Rc::new(RefCell::new(false));
    let v = Rc::new(RefCell::new(Vec::new()));

    input::set_keypad(&document, &k);
    input::set_breakpoint(&document, &b);
    input::get_file_reader(&document, &v);

    let k2 = Rc::clone(&k);
    let b2 = Rc::clone(&b);
    let game = Rc::clone(&v);

    // EVENT LOOP
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        set_timeout(t1.borrow().as_ref().unwrap(), 20);

        emulator.keypad = *k2.borrow_mut();
        emulator.running = *b2.borrow_mut();

        if !emulator.cartridge_loaded && !game.borrow().is_empty() {
            emulator.load_game(game.borrow().clone());
            emulator.cartridge_loaded = !emulator.cartridge_loaded;
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

fn set_timeout(f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    web_sys::window()
        .expect("should have a window.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}
