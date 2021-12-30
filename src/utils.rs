use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
/// its id
pub fn append_element_to_another(element: &web_sys::Node, id: &str) {
    document()
        .get_element_by_id(id)
        .unwrap()
        .append_child(&element)
        .unwrap();
}

/// Util function to get a `web_sys::Document`.
pub fn document() -> web_sys::Document {
    web_sys::window()
        .unwrap()
        .document()
        .expect("should have a document.")
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
