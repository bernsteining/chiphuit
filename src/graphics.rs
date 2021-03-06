//! # A module to display the screen of our `Emulator` with the [Canvas API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API).

use crate::utils::{append_to_body, document};
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

/// Set the canvas in the browser that will be be used to render the chip8
/// Emulator screen.
pub fn set_canvas() -> web_sys::CanvasRenderingContext2d {
    let canvas: web_sys::HtmlCanvasElement = document()
        .create_element("canvas")
        .expect("Should have an element named canvas.")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Should have a Canvas element.");

    canvas.set_id("canvas");

    canvas.set_width(WIDTH);
    canvas.set_height(HEIGHT);

    append_to_body(&canvas);

    canvas
        .get_context("2d")
        .expect("Should have a 2D Context.")
        .expect("Should have a rendering canvas.")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Should have a rendering canvas.")
}

/// Render the chip8 Emulator screen in the browser using the Canvas API.
///
/// Since every pixel of a chip8 `Emulator` screen (64x32)
/// has only 2 possible values (turned off or turned on), these are represented
/// in memory by bools. This function iterates over the pixel states of the
/// `Emulator` and draws pixels on the Canvas.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let context = set_canvas();
/// let screen = [true, 64 * 32];
///
/// // turns all the pixels of the Emulator screen on.
/// draw_screen(&context, screen);
/// ```
pub fn draw_screen(context: &CanvasRenderingContext2d, screen: [bool; 64 * 32]) {
    let rgba_screen: Vec<u8> = screen
        .iter()
        .flat_map(|x| match x {
            false => [0u8; 4],
            true => [255u8; 4],
        })
        .collect();

    let frame =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&rgba_screen), WIDTH, HEIGHT).unwrap();

    context.put_image_data(&frame, 0.0, 0.0).unwrap();
}
