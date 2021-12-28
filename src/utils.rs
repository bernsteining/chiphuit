use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Util function to append a `web_sys::Node` to the body.
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

// Util function for the event loop.
pub fn set_timeout(f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    web_sys::window()
        .expect("should have a window.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}