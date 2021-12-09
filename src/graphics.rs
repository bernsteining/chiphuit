use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData};

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    web_sys::window()
        .expect("should have a window.")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

pub fn get_context() -> web_sys::CanvasRenderingContext2d {
    web_sys::window()
        .unwrap()
        .document()
        .expect("Should have a doc.")
        .get_element_by_id("canvas")
        .expect("Should have a canvas.")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Should have a HTML canvas element.")
        .get_context("2d")
        .expect("Should have a 2D Context.")
        .expect("Should have a rendering canvas.")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Should have a rendering canvas.")
}

pub fn draw_screen(context: &CanvasRenderingContext2d, boolean_screen: [bool; 64 * 32]) {
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
    let graphic_screen =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut graphic_screen), WIDTH, HEIGHT)
            .unwrap();
    context.put_image_data(&graphic_screen, 0.0, 0.0);
}
