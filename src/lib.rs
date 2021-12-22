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
        .expect("should a document.");

    let emulator_state = document
        .create_element("div")
        .expect("should have an emulator state in top right corner.");

    emulator_state.set_id("emulator_state");
    emulator_state.set_class_name("emulator_state");

    document
        .body()
        .expect("document should have a body")
        .append_child(&emulator_state)
        .unwrap();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let t1 = Rc::new(RefCell::new(None));

    *t1.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        graphics::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let mut emulator = cpu::Emulator::new();
    emulator.load_font();

    let mut game_loaded = false;

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

        if !game_loaded && !game.borrow().is_empty() {
            emulator.load_game(game.borrow().clone());
            game_loaded = !game_loaded;
        }

        match emulator.running {
            true => {
                for _ in 0..10 {
                    emulator.cycle();
                }
                document
                    .get_element_by_id("breakpoint")
                    .unwrap()
                    .set_inner_html("pause")
            }
            false => document
                .get_element_by_id("breakpoint")
                .unwrap()
                .set_inner_html("play"),
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
