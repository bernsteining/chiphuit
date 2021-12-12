use js_sys::{Object, Uint8Array};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Event, File, FileList, FileReader, HtmlInputElement, Node};

pub fn set_keypad(document: &web_sys::Document, k: &Rc<RefCell<[bool; 16]>>) {
    // document
    //     .create_element("keypad")
    //     .expect("should have a keypad.");
    let keypad = document
        .get_element_by_id("keypad")
        .expect("should have a keypad.");
    keypad.set_class_name("keypad-base");

    for (index, key) in [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ]
    .iter()
    .enumerate()
    {
        let keypad_key = document.create_element("div").unwrap();
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

pub fn set_breakpoint(document: &web_sys::Document, b: &Rc<RefCell<bool>>) {
    document
        .create_element("breakpoint")
        .expect("should have a breakpoint.");
    let breakpoint = document
        .get_element_by_id("breakpoint")
        .expect("should have a breakpoint.");

    breakpoint.set_class_name("breakpoint");
    breakpoint.set_inner_html("play");

    let b1 = Rc::clone(&b);
    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        *b1.borrow_mut() ^= true;
    }) as Box<dyn FnMut(_)>);

    breakpoint
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget()
}

pub fn get_file_reader(document: &web_sys::Document) {
    let filereader = FileReader::new().unwrap().dyn_into::<FileReader>().unwrap();

    let onload = Closure::wrap(Box::new(move |event: Event| {
        let element = event.target().unwrap().dyn_into::<FileReader>().unwrap();
        let data = element.result().unwrap();
        let js_data = js_sys::Uint8Array::from(data);
        // let rust_str: String = js_data.to_string().into();
        console::log_1(&format!("plz1 {:?}", js_data).into());
    }) as Box<dyn FnMut(_)>);

    filereader.set_onloadend(Some(onload.as_ref().unchecked_ref()));
    onload.forget();

    let fileinput: HtmlInputElement = document
        .get_element_by_id("file-upload")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    fileinput.set_type("file");

    let callback = Closure::wrap(Box::new(move |event: Event| {
        let element = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let filelist = element.files().unwrap();

        let _file = filelist.get(0).expect("should have a file handle.");
        // filereader.read_as_array_buffer(&_file).unwrap();
        filereader.read_as_binary_string(&_file).unwrap();
    }) as Box<dyn FnMut(_)>);
    fileinput
        .add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())
        .unwrap();

    callback.forget();
}
