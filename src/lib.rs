use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData};

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

        // random black and white screen
        let mut screen = [false; 64 * 32];
        for i in 0..64 * 32 {
            screen[i] = match random() {
                0.0..=0.5 => false,
                _ => true,
            }
        }

        draw_screen(&context, screen);
    }) as Box<dyn FnMut()>));

    graphics::request_animation_frame(&graphics::window(), g.borrow().as_ref().unwrap());

    Ok(())
}

fn draw_screen(context: &CanvasRenderingContext2d, boolean_screen: [bool; 64 * 32]) {
    let mut graphic_screen = [0; 64 * 32 * 4];
    for (i, x) in boolean_screen.iter().enumerate() {
        if x == &false {
            graphic_screen[4 * i + 1] = 0;
            graphic_screen[4 * i + 2] = 0;
            graphic_screen[4 * i + 3] = 0;
            graphic_screen[4 * i] = 0;
        } else {
            graphic_screen[4 * i + 1] = 255;
            graphic_screen[4 * i + 2] = 255;
            graphic_screen[4 * i + 3] = 255;
            graphic_screen[4 * i] = 255;
        }
    }
    let graphic_screen = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut graphic_screen),
        graphics::WIDTH,
        graphics::HEIGHT,
    )
    .unwrap();
    context.put_image_data(&graphic_screen, 0.0, 0.0);
}

fn set_timeout(window: &web_sys::Window, f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}
