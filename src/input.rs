//! # A module to set and bind every element used for user input.
//! - The keypad
//! - The breakpoint
//! - The file input to handle the ROM
use crate::utils::{
    append_element_to_another, append_to_body, change_view, document, set_callback_to_button,
    set_callback_to_key, read_user_file
};
use js_sys::JsString;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, FileReader, HtmlInputElement, HtmlLabelElement, Node};

/// Set the keypad in the UI.
pub fn set_keypad(emulator_keypad: &Rc<RefCell<[bool; 16]>>) {
    let keypad = document()
        .create_element("keypad")
        .expect("should have a keypad.");

    keypad.set_id("keypad");
    keypad.set_class_name("keypad-base");

    append_to_body(&keypad);

    for (index, &key) in [
        "1", "2", "3", "4", "Q", "W", "E", "R", "A", "S", "D", "F", "Z", "X", "C", "V",
    ]
    .iter()
    .enumerate()
    {
        let keypad_key = document().create_element("div").unwrap();
        keypad_key.set_id(key);
        keypad_key.set_inner_html(key);
        keypad_key.set_class_name("key");
        keypad
            .append_child(&Node::from(keypad_key.clone()))
            .unwrap();

        // Handle clicks on virtual keypad
        set_callback_to_button(true, &keypad_key, emulator_keypad, index);
        set_callback_to_button(false, &keypad_key, emulator_keypad, index);

        // Handle keyboard events
        set_callback_to_key(true, key.to_string(), emulator_keypad, index);
        set_callback_to_key(false, key.to_string(), emulator_keypad, index);
    }
}

/// Set the breakpoint button in the UI.
pub fn set_breakpoint(emulator_breakpoint: &Rc<RefCell<bool>>) {
    let breakpoint = document()
        .create_element("breakpoint")
        .expect("should have a breakpoint.");

    breakpoint.set_id("breakpoint");
    breakpoint.set_class_name("breakpoint");
    breakpoint.set_inner_html("⏯");
    append_element_to_another(&breakpoint, "keypad");

    let breakpoint_clone = Rc::clone(emulator_breakpoint);
    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        *breakpoint_clone.borrow_mut() ^= true;
    }) as Box<dyn FnMut(_)>);

    breakpoint
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget()
}

/// Set the debug button in the UI.
pub fn set_debug() {
    let debug = document()
        .create_element("debug")
        .expect("should have a debug.");

    debug.set_id("debug");
    debug.set_class_name("debug");
    debug.set_inner_html("⚙");

    append_element_to_another(&debug, "keypad");

    let closure = change_view();

    debug
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget()
}

/// Set button for rom selection.
pub fn set_file_upload_button() {
    let fileinput: HtmlInputElement = document()
        .create_element("input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    fileinput.set_id("file-upload");
    fileinput.set_type("file");
    append_to_body(&fileinput);

    let label: HtmlLabelElement = document()
        .create_element("label")
        .unwrap()
        .dyn_into::<HtmlLabelElement>()
        .unwrap();

    label.set_html_for("file-upload");
    label.set_inner_text("Select ROM");
    label.set_class_name("file-upload");
    append_element_to_another(&label, "keypad");
}

/// Closure to load user input ROM in the Emulator.
pub fn load_user_rom(rom_buffer: &Rc<RefCell<Vec<u8>>>) -> Closure<dyn FnMut(Event)> {
    let rom = Rc::clone(rom_buffer);
    Closure::wrap(Box::new(move |event: Event| {
        *rom.borrow_mut() = event
            .target()
            .unwrap()
            .dyn_into::<FileReader>()
            .unwrap()
            .result()
            .unwrap()
            .dyn_into::<JsString>()
            .unwrap()
            .iter()
            .map(|x| x as u8)
            .collect();
    }))
}

/// Set the button to allow the user to supply a ROM to the `Emulator`.
pub fn set_file_reader(rom_buffer: &Rc<RefCell<Vec<u8>>>) {
    let file_input = document()
        .get_element_by_id("file-upload")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    let file_reader = FileReader::new().unwrap().dyn_into::<FileReader>().unwrap();

    let handle_load_event = load_user_rom(rom_buffer);
    file_reader.set_onloadend(Some(handle_load_event.as_ref().unchecked_ref()));
    handle_load_event.forget();

    let handle_read_event = read_user_file(file_reader);
    file_input
        .add_event_listener_with_callback("change", handle_read_event.as_ref().unchecked_ref())
        .unwrap();
    handle_read_event.forget();
}
