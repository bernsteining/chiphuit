use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use js_sys::Math::random;
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
        "1225535041434520494e56414445525320302e39312042792044617669642057494e544552600061006208a3ddd0187108f21e3120122d700861003040122d69056c156e002391600af015f0073000124b23917e0112456600681c69006a046b0a6c046d3c6e0f00e023752351fd156004e09e127d2375380078ff23756006e09e128b23753839780123753600129f6005e09e12e96601651b8480a3d9d451a3d9d45175ff35ff12ad660012e9d4513f0112e9d45166008340730383b562f883226208330012c9237d8206430812d3331012d5237d8206331812dd237d8206432012e7332812e9237d3e0013077906491869006a046b0a6c047df46e0f00e023512375fd15126ff7073700126ffd1523518ba43b12131b7c026afc3b0213237c026a0423513c18126f00e0a4dd60146108620fd01f7008f21e302c133360fff015f00730001341f00a00e0a706fe651225a3c1f91e610823698106236981062369810623697bd000ee80e080123000dbc67b0c00eea3d9601cd80400ee23518e2323516005f018f015f0073000138900ee6a008de06b04e9a11257a60cfd1ef06530ff13af6a006b046d016e011397a50af01edbc67b087d017a013a07139700ee3c7effff99997effff2424e77eff3c3c7edb81423c7effdb10387cfe00007f003f007f0000000101010303030300003f20202020202020203f0808ff0000fe00fc00fe0000007e4242626262620000ff0000000000000000ff0000ff007d00417d057d7d0000c2c2c6446c28380000ff0000000000000000ff0000ff00f71014f7f7040400007c44fec2c2c2c20000ff0000000000000000ff0000ff00ef2028e8e82f2f0000f985c5c5c5c5f90000ff0000000000000000ff0000ff00be00203020bebe0000f704e7858584f40000ff0000000000000000ff0000ff00007f003f007f000000ef28ef00e0606f0000ff0000000000000000ff0000ff0000fe00fc00fe000000c000c0c0c0c0c00000fc0404040404040404fc1010fff981b98b9a9afa00fa8a9a9a9b99f8e62525f434343400171434373626c7df50505cd8d8df00df111f121b19d97c44fe868686fc84fe8282fefe80c0c0c0fefc82c2c2c2fcfe80f8c0c0fefe80f0c0c0c0fe80be8686fe8686fe8686861010101010101818184848789c90b0c0b09c8080c0c0c0feee9292868686fe82868686867c828686867cfe82fec0c0c07c82c2cac47afe86fe909c84fec0fe0202fefe10303030308282c2c2c2fe828282ee38108686969292ee8244383844828282fe303030fe021ef080fe0000000006060000006060c00000000000001818181800187cc60c1800180000fefe0000fe82868686fe080808181818fe02fec0c0fefe021e0606fe84c4c4fe0404fe80fe0606fec0c0c0fe82fefe02020606067c44fe8686fefe82fe06060644fe4444fe44a8a8a8a8a8a8a86c5a000c18a8304e7e001218666ca85a665424660048481812a80690a812007e3012a884304e721866a8a8a8a8a8a8905478a848786c72a812186c72665490a8722a18a8304e7e001218666ca87254a85a66187e184e72a8722a183066a8304e7e006c30544e9ca8a8a8a8a8a8a848547e18a890547866a86c2a305aa88430722aa8d8a8004e12a8e4a2a8004e12a86c2a545472a88430722aa8de9ca8722a18a80c54485a78721866a866185a5466726ca8722a0072a8722a18a8304e7e001218666ca8006618a8304e0c6618006c304e24a8722a183066a81e54660c189ca824545412a842780c3ca8aea8a8a8a8a8a8a8ff000000000000000000000000000000",
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
        set_timeout(&w2, t1.borrow().as_ref().unwrap(), 1);

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
