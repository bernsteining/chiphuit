//! # A module to avoid re-writing `wasm-bindgen` functions.

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Element, Event, FileReader, HtmlElement, HtmlInputElement};

pub const EMULATOR_VARIABLES: [&str; 9] = [
    "current opcode",
    "registers",
    "index register",
    "program counter",
    "delay timer",
    "sound timer",
    "stack pointer",
    "stack",
    "running",
];

/// Util function to append a `web_sys::Node` to the body.
pub fn append_to_body(element: &web_sys::Node) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .expect("document should have a body")
        .append_child(element)
        .unwrap();
}

/// Util function to append a `web_sys::Node` to an HTML element selected by
/// its id.
pub fn append_element_to_another(element: &web_sys::Node, id: &str) {
    document()
        .get_element_by_id(id)
        .unwrap()
        .append_child(element)
        .unwrap();
}

/// Util function to get a `web_sys::Document`.
pub fn document() -> web_sys::Document {
    web_sys::window()
        .unwrap()
        .document()
        .expect("should have a document.")
}

/// Util function to set basic attributes of HTML page.
pub fn set_document() {
    document().set_title("chip8 emulator");

    let head = document().head().unwrap();

    let link = document().create_element("link").unwrap();
    link.set_attribute("rel", "stylesheet").unwrap();
    link.set_attribute("href", "chiphuit.css").unwrap();

    head.append_child(&link).unwrap();
}

/// Util function for the event loop.
pub fn set_timeout(f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    web_sys::window()
        .expect("should have a window.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}

/// Util function to put a string in the clipboard
pub fn to_clipboard(content: String) {
    window()
        .unwrap()
        .navigator()
        .clipboard()
        .unwrap()
        .write_text(&content);
}

/// Util function to set listeners and callbacks on buttons.
/// Handles clicks on virtual keypad.
pub fn set_callback_to_button(
    press: bool,
    button: &Element,
    keypad: &Rc<RefCell<[bool; 16]>>,
    index: usize,
) {
    let keypad_clone = Rc::clone(keypad);
    let mouse_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        keypad_clone.borrow_mut()[index] = press;
    }) as Box<dyn FnMut(_)>);

    button
        .add_event_listener_with_callback(
            match press {
                true => "mousedown",
                false => "mouseup",
            },
            mouse_callback.as_ref().unchecked_ref(),
        )
        .unwrap();
    mouse_callback.forget();

    let keypad_clone = Rc::clone(keypad);
    let touch_callback = Closure::wrap(Box::new(move |_event: web_sys::TouchEvent| {
        keypad_clone.borrow_mut()[index] = press;
    }) as Box<dyn FnMut(_)>);

    button
        .add_event_listener_with_callback(
            match press {
                true => "touchstart",
                false => "touchend",
            },
            touch_callback.as_ref().unchecked_ref(),
        )
        .unwrap();
    touch_callback.forget();
}

/// Util function to set listeners and callbacks on keyboard keys.
/// Handles user input done with the keyboard.
pub fn set_callback_to_key(
    press: bool,
    key: String,
    keypad: &Rc<RefCell<[bool; 16]>>,
    index: usize,
) {
    let keypad_clone = Rc::clone(keypad);
    let callback = Closure::wrap(Box::new(move |_event: web_sys::KeyboardEvent| {
        if _event.key().to_uppercase() == key {
            keypad_clone.borrow_mut()[index] = press;
        }
    }) as Box<dyn FnMut(_)>);

    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            match press {
                true => "keydown",
                false => "keyup",
            },
            callback.as_ref().unchecked_ref(),
        )
        .unwrap();
    callback.forget();
}

/// Util Closure to switch from keypad view to debugger view and vice-versa.
pub fn change_view() -> Closure<dyn FnMut(web_sys::MouseEvent)> {
    Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let debugger_style = document()
            .get_element_by_id("debugger")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .style();

        let keypad_style = document()
            .get_element_by_id("keypad")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .style();

        match debugger_style
            .get_property_value("display")
            .unwrap()
            .as_str()
        {
            "none" => {
                debugger_style.set_property("display", "flex").unwrap();
                keypad_style.set_property("display", "none").unwrap()
            }
            _ => {
                debugger_style.set_property("display", "none").unwrap();
                keypad_style.set_property("display", "grid").unwrap()
            }
        }
    }) as Box<dyn FnMut(_)>)
}

// Should write a set_gamepad_to_key here if we want to handle Gamepad events

/// Closure to read user input file.
pub fn read_user_file(filereader: FileReader) -> Closure<dyn FnMut(Event)> {
    Closure::wrap(Box::new(move |event: Event| {
        let file = event
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .files()
            .unwrap()
            .get(0)
            .unwrap();

        filereader.read_as_binary_string(&file).unwrap();
    }) as Box<dyn FnMut(_)>)
}

// Utils to allow JSON serialization of big arrays with serde
// shouldn't be necessary anymore when serde 2.0 gets released. Code snippet
// from  https://github.com/serde-rs/serde/issues/1937#issuecomment-812137971
pub mod arrays {
    use std::{convert::TryInto, marker::PhantomData};

    use serde::{
        de::{SeqAccess, Visitor},
        ser::SerializeTuple,
        Deserialize, Deserializer, Serialize, Serializer,
    };
    pub fn serialize<S: Serializer, T: Serialize, const N: usize>(
        data: &[T; N],
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        let mut s = ser.serialize_tuple(N)?;
        for item in data {
            s.serialize_element(item)?;
        }
        s.end()
    }

    struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

    impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
    where
        T: Deserialize<'de>,
    {
        type Value = [T; N];

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(&format!("an array of length {}", N))
        }

        #[inline]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // can be optimized using MaybeUninit
            let mut data = Vec::with_capacity(N);
            for _ in 0..N {
                match (seq.next_element())? {
                    Some(val) => data.push(val),
                    None => return Err(serde::de::Error::invalid_length(N, &self)),
                }
            }
            match data.try_into() {
                Ok(arr) => Ok(arr),
                Err(_) => unreachable!(),
            }
        }
    }
    pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
    }
}
