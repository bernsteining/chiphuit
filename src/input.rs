use crate::utils::{append_element_to_another, append_to_body, document};
use js_sys::JsString;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Event, FileReader, HtmlInputElement, HtmlLabelElement, Node};

pub fn set_keypad(k: &Rc<RefCell<[bool; 16]>>) {
    let keypad = document()
        .create_element("keypad")
        .expect("should have a keypad.");

    keypad.set_id("keypad");
    keypad.set_class_name("keypad-base");

    append_to_body(&keypad);

    for (index, key) in [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
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

        let k1 = Rc::clone(&k);
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            k1.borrow_mut()[index] ^= true;
        }) as Box<dyn FnMut(_)>);

        keypad_key
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget()
    }
}

pub fn set_breakpoint(b: &Rc<RefCell<bool>>) {
    let breakpoint = document()
        .create_element("breakpoint")
        .expect("should have a breakpoint.");

    breakpoint.set_id("breakpoint");
    breakpoint.set_class_name("breakpoint");
    breakpoint.set_inner_html("play");

    append_element_to_another(&breakpoint, "keypad");

    let b1 = Rc::clone(&b);

    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        *b1.borrow_mut() ^= true;

        let _breakpoint = document()
            .get_element_by_id("breakpoint")
            .expect("should have a breakpoint.");
        let button_content = _breakpoint.inner_html();

        match button_content.as_ref() {
            "play" => _breakpoint.set_inner_html("pause"),
            _ => _breakpoint.set_inner_html("play"),
        }
    }) as Box<dyn FnMut(_)>);

    breakpoint
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget()
}

pub fn set_file_reader(rom_buffer: &Rc<RefCell<Vec<u8>>>) {
    let filereader = FileReader::new().unwrap().dyn_into::<FileReader>().unwrap();
    let rom = Rc::clone(&rom_buffer);
    let onload = Closure::wrap(Box::new(move |event: Event| {
        let element = event.target().unwrap().dyn_into::<FileReader>().unwrap();
        let data = element.result().unwrap();
        let game_string: JsString = data.dyn_into::<JsString>().unwrap();
        let game_vec: Vec<u8> = game_string.iter().map(|x| x as u8).collect();
        *rom.borrow_mut() = game_vec.clone();
        console::log_1(&format!("game loaded: {:?}", game_string).into());
    }) as Box<dyn FnMut(_)>);

    filereader.set_onloadend(Some(onload.as_ref().unchecked_ref()));
    onload.forget();

    let fileinput: HtmlInputElement = document()
        .create_element("input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    fileinput.set_id("file-upload");
    fileinput.set_type("file");

    let label: HtmlLabelElement = document()
        .create_element("label")
        .unwrap()
        .dyn_into::<HtmlLabelElement>()
        .unwrap();

    label.set_html_for("file-upload");
    label.set_inner_text("Choose chip8 ROM");
    label.set_class_name("file-upload");

    append_element_to_another(&label, "keypad");
    append_to_body(&fileinput);

    let callback = Closure::wrap(Box::new(move |event: Event| {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let filelist = element.files().unwrap();

        let _file = filelist.get(0).expect("should have a file handle.");
        filereader.read_as_binary_string(&_file).unwrap();
    }) as Box<dyn FnMut(_)>);
    fileinput
        .add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())
        .unwrap();

    callback.forget();
}
