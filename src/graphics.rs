use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub fn request_animation_frame(window: &web_sys::Window, f: &Closure<dyn FnMut()>) -> i32 {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
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
