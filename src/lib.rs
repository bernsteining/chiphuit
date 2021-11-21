use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::num::ParseIntError;

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

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
    let rom_test = decode_hex(
        "124eeaacaaeaceaaaaaee0a0a0e0c04040e0e020c0e0e06020e0a0e0202060402040e080e0e0e0202020e0e0a0e0e0e020e040a0e0a0e0c080e0e080c080a040a0a0a202dab400eea202dab413dc680169056a0a6b01652a662ba216d8b4a23ed9b4a202362ba206dab46b06a21ad8b4a23ed9b4a206452aa202dab46b0ba21ed8b4a23ed9b4a2065560a202dab46b10a226d8b4a23ed9b4a20676ff462aa202dab46b15a22ed8b4a23ed9b4a2069560a202dab46b1aa232d8b4a23ed9b422426817691b6a206b01a20ad8b4a236d9b4a202dab46b06a22ad8b4a20ad9b4a2068750472aa202dab46b0ba22ad8b4a20ed9b4a206672a87b1472ba202dab46b10a22ad8b4a212d9b4a2066678671f87624718a202dab46b15a22ad8b4a216d9b4a2066678671f87634767a202dab46b1aa22ad8b4a21ad9b4a206668c678c87644718a202dab4682c69306a346b01a22ad8b4a21ed9b4a206668c6778876547eca202dab46b06a22ad8b4a222d9b4a20666e0866e46c0a202dab46b0ba22ad8b4a236d9b4a206660f86664607a202dab46b10a23ad8b4a21ed9b4a3e860006130f155a3e9f065a2064030a202dab46b15a23ad8b4a216d9b4a3e86689f633f265a2023001a2063103a2063207a206dab46b1aa20ed8b4a23ed9b4124813dc",
    )
    .unwrap();

    // INIT EMULATOR
    let mut emulator = cpu::Emulator::new();

    // LOAD FONT
    emulator.load_font();

    // LOAD GAME
    emulator.load_game(rom_test);

    // EVENT LOOP
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        set_timeout(&w2, t1.borrow().as_ref().unwrap(), 167);

        // handle input here
        // compute cpu cycle
        emulator.cycle();

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
