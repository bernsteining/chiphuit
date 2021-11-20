use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use js_sys::Math::random;
mod cpu;
mod graphics;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let context = graphics::get_context();
    let document = web_sys::window().unwrap().document().unwrap();
    let emulator_state = document
        .create_element("emulator_state")
        .expect("should have the emulator state element");

    emulator_state.set_class_name("emulator_state");

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let t1 = Rc::new(RefCell::new(None));
    let t2 = t1.clone();

    let w = graphics::window();
    *t2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        graphics::request_animation_frame(&w, f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let w2 = graphics::window();

    // HARDCODE GAME ATM
    let break_out = b"129ffcfc80a202ddc100eea204dba100eea2036002610587008610d67171086f388f174f00121770026f108f074f00121500ee22057d04220500ee22057dfc220500ee8080400168ff40ff68015ac0225300ee80b070fb61f880127005a203d0a100ee220b8b948a84220b4b0069014b3f69ff4a0068014a1f68ff4f0122434a1f228500ee00e06b1e6a142205220b221100eefe073e0012936e04fe1500ee6d1e6c1e6b406a1dc901490069ff68ff2205220b22116007e0a1223b6009e0a122332263229312b5".to_vec();

    // INIT EMULATOR
    let mut emulator = cpu::Emulator::new();

    // LOAD FONT
    emulator.load_font();

    // LOAD GAME
    emulator.load_game(break_out);

    // EVENT LOOP
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        set_timeout(&w2, t1.borrow().as_ref().unwrap(), 167);

        // handle input here
        // compute cpu cycle
        emulator.cycle();

        // random black and white screen
        let mut screen = [false; 64 * 32];
        for i in 0..64 * 32 {
            screen[i] = match random() {
                0.0..=0.5 => false,
                _ => true,
            }
        }
        emulator.screen = screen;

        emulator_state.set_inner_html(&emulator.to_string());
        document
            .body()
            .expect("document should have a body")
            .append_child(&emulator_state)
            .unwrap();

        graphics::draw_screen(&context, emulator.screen);
    }) as Box<dyn FnMut()>));

    graphics::request_animation_frame(&graphics::window(), g.borrow().as_ref().unwrap());

    Ok(())
}

fn set_timeout(window: &web_sys::Window, f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}
