use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData};

use js_sys::Math::random;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

//write cpu struct with impls
pub struct Emulator {
    current_opcode: u32,
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
    fn new() -> Emulator {
        Emulator {
            current_opcode: 0,
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

    fn fetch_opcode(&self) -> u16 {
        (self.memory[self.program_counter as usize] as u16) << 8
            | self.memory[(self.program_counter as usize + 1) as usize] as u16
    }

    fn cycle(&self) {}

    fn process_opcode(&self, opcode: u16) {}
}

#[wasm_bindgen]
pub fn handle_input(key: String) {
    // logging on webpage
    // let keys = document().create_element("lol").unwrap();
    // keys.set_inner_html(&format!("<li>{}<li>", &key).to_string());
    // body().append_child(&keys).unwrap();

    //let text = format!("Keypress: {}", key);
}

//write opcodes pattern matching logic

// check this for the loop
// https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let t1 = Rc::new(RefCell::new(None));
    let t2 = t1.clone();

    let w = window();
    *t2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        request_animation_frame(&w, f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let w2 = window();

    // HARDCODE GAME ATM

    let break_out = "129ffcfc80a202ddc100eea204dba100eea2036002610587008610d67171086f388f174f00121770026f108f074f00121500ee22057d04220500ee22057dfc220500ee8080400168ff40ff68015ac0225300ee80b070fb61f880127005a203d0a100ee220b8b948a84220b4b0069014b3f69ff4a0068014a1f68ff4f0122434a1f228500ee00e06b1e6a142205220b221100eefe073e0012936e04fe1500ee6d1e6c1e6b406a1dc901490069ff68ff2205220b22116007e0a1223b6009e0a122332263229312b5";

    // INIT EMULATOR
    let emulator = Emulator::new();

    // EVENT LOOP
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        set_timeout(&w2, t1.borrow().as_ref().unwrap(), 167);

        // handle input here
        // compute cpu cycle
        // draw_screen(&context, screen_array, position_attribute_location as u32);

        let mut data = [0; 64 * 32 * 4];
        for (i, x) in data.iter_mut().enumerate() {
            *x = (random() * 255.0) as u8;
        }
        let data =
            ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), WIDTH, HEIGHT).unwrap();
        context.put_image_data(&data, 0.0, 0.0);
    }) as Box<dyn FnMut()>));

    request_animation_frame(&window(), g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(window: &web_sys::Window, f: &Closure<dyn FnMut()>) -> i32 {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn set_timeout(window: &web_sys::Window, f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}
