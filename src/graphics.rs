//! # A module to display the screen of our `Emulator` with the [Canvas API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API).

use crate::utils::{append_to_body, document, EMULATOR_VARIABLES};
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

/// Render the Emulator state in the browser to inspect its fields values at
/// runtime.
pub fn set_emulator_state() -> web_sys::HtmlCollection {
    let emulator_state = document()
        .create_element("table")
        .expect("should have an element.")
        .dyn_into::<web_sys::HtmlTableElement>()
        .expect("should have an HtmlTableElement.");

    emulator_state.set_id("emulator_state");
    emulator_state.set_class_name("emulator_state");

    let callback = Closure::wrap(Box::new(move |_event: web_sys::KeyboardEvent| {
        let _e = document().get_element_by_id("emulator_state").unwrap();
        if _event.key() == "Escape" {
            match _e.has_attribute("hidden") {
                true => _e.remove_attribute("hidden").unwrap(),
                false => _e.set_attribute("hidden", "").unwrap(),
            }
        }
    }) as Box<dyn FnMut(_)>);

    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
        .unwrap();
    callback.forget();

    emulator_state.create_t_body();

    // for table headers
    emulator_state.insert_row().unwrap();

    let rows = emulator_state.rows();

    rows.item(0)
        .unwrap()
        .set_inner_html("<th>variable</th><th>value</th>");

    for variable in EMULATOR_VARIABLES.iter() {
        let row = emulator_state
            .insert_row()
            .unwrap()
            .dyn_into::<web_sys::HtmlTableRowElement>()
            .unwrap();

        let variable_cell = row.insert_cell().unwrap();

        // init value cell
        row.insert_cell().unwrap();

        variable_cell.set_inner_html(variable);
    }

    // add edit n commit row
    let modify_emulator_row = emulator_state
        .insert_row()
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap();

    let edit = modify_emulator_row.insert_cell().unwrap();

    let commit = modify_emulator_row.insert_cell().unwrap();

    edit.set_class_name("emulator_state_button");
    edit.set_id("emulator_state_edit");
    edit.set_inner_html("edit");

    commit.set_class_name("emulator_state_button");
    commit.set_inner_html("commit");

    append_to_body(&emulator_state);

    let callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let rows = document()
            .get_element_by_id("emulator_state")
            .unwrap()
            .dyn_into::<web_sys::HtmlTableElement>()
            .unwrap()
            .rows();

        editcontent(&rows);
    }) as Box<dyn FnMut(_)>);

    // call editcontent in match arms

    edit.add_event_listener_with_callback("mousedown", callback.as_ref().unchecked_ref())
        .unwrap();
    callback.forget();

    emulator_state.rows()

    // edit and commit row should be visible only if running = false

    // commit should push these cells in the emulator struct
    // How? Emulator trait from serde?
}

/// Set the debugger editable for the user.
pub fn editcontent(rows: &web_sys::HtmlCollection) {
    match rows
        .get_with_index(1)
        .unwrap()
        .has_attribute("contenteditable")
    {
        true => {
            for index in 1..9 {
                rows.get_with_index(index)
                    .unwrap()
                    .remove_attribute("contenteditable")
                    .unwrap()
            }
        }
        false => {
            for index in 1..9 {
                rows.get_with_index(index)
                    .unwrap()
                    .set_attribute("contenteditable", "true")
                    .unwrap()
            }
        }
    }
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
/// let boolean_screen = [true, 64 * 32];
///
/// // turns all the pixels of the Emulator screen on.
/// draw_screen(context, boolean_screen);
/// ```
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
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&graphic_screen), WIDTH, HEIGHT)
            .unwrap();
    context.put_image_data(&graphic_screen, 0.0, 0.0);
}
