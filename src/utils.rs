use wasm_bindgen::prelude::*;

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
