use crate::utils::{append_to_body, document, EMULATOR_VARIABLES};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlTableRowElement;
/// An `Emulator` debugger.
pub struct Debugger {
    pub element: web_sys::HtmlTableElement,
}

impl Debugger {
    /// Instanciate `Debugger` in the GUI with all necessary callbacks.
    pub fn new() -> Debugger {
        let debugger = create_element();
        set_show_hide_callback();
        fill_rows(&debugger);
        step_and_memory(&debugger);
        edit_and_commit(&debugger);
        Debugger { element: debugger }
    }
}

/// Create the `Debugger` GUI element.
fn create_element() -> web_sys::HtmlTableElement {
    let element = document()
        .create_element("table")
        .expect("should have an element.")
        .dyn_into::<web_sys::HtmlTableElement>()
        .expect("should have an HtmlTableElement.");
    element.set_id("debugger");
    element.set_class_name("debugger");
    element
}

/// Fill the `Debugger`'s table with the `Emulator`'s fields.
fn fill_rows(element: &web_sys::HtmlTableElement) {
    element.insert_row().unwrap();

    let rows = element.rows();

    let first_row = rows
        .item(0)
        .unwrap()
        .dyn_into::<HtmlTableRowElement>()
        .unwrap();

    first_row.insert_cell().unwrap().set_inner_html("variable");
    first_row.insert_cell().unwrap().set_inner_html("value");

    for variable in EMULATOR_VARIABLES.iter() {
        let row = element
            .insert_row()
            .unwrap()
            .dyn_into::<web_sys::HtmlTableRowElement>()
            .unwrap();

        let variable_cell = row.insert_cell().unwrap();

        // init value cell
        row.insert_cell().unwrap();

        variable_cell.set_inner_html(variable);
    }
}

/// Add step and memory viewer buttons to `Debugger`.
fn step_and_memory(element: &web_sys::HtmlTableElement) {
    let modify_emulator_row = element
        .insert_row()
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap();

    let step = modify_emulator_row.insert_cell().unwrap();

    let memory_viewer = modify_emulator_row.insert_cell().unwrap();

    step.set_class_name("debugger_button");
    step.set_id("step");
    step.set_inner_html("step");

    memory_viewer.set_class_name("debugger_button");
    memory_viewer.set_inner_html("memory viewer");

    append_to_body(&element);

    let step_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        // this should run cycle() on the `Emulator`.
    }) as Box<dyn FnMut(_)>);

    step.add_event_listener_with_callback("mousedown", step_callback.as_ref().unchecked_ref())
        .unwrap();
    step_callback.forget();

    let memory_viewer_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        // this should show a memory viewer in hexdump style of the
        // `Emulator` struct.
    }) as Box<dyn FnMut(_)>);

    memory_viewer
        .add_event_listener_with_callback(
            "mousedown",
            memory_viewer_callback.as_ref().unchecked_ref(),
        )
        .unwrap();
    memory_viewer_callback.forget();
}

/// Set callbacks to allow modification of the `Emulator`'s fields.
fn edit_and_commit(element: &web_sys::HtmlTableElement) {
    let modify_emulator_row = element
        .insert_row()
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap();

    let edit = modify_emulator_row.insert_cell().unwrap();

    let commit = modify_emulator_row.insert_cell().unwrap();

    edit.set_class_name("debugger_button");
    edit.set_id("debugger_edit");
    edit.set_inner_html("edit");

    commit.set_class_name("debugger_button");
    commit.set_inner_html("commit");

    append_to_body(&element);

    let edit_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let rows = document()
            .get_element_by_id("debugger")
            .unwrap()
            .dyn_into::<web_sys::HtmlTableElement>()
            .expect("should have an HtmlTableElement.")
            .rows();

        let range = 1..9;

        match rows
            .get_with_index(1)
            .unwrap()
            .has_attribute("contenteditable")
        {
            true => {
                for index in range {
                    rows.get_with_index(index)
                        .unwrap()
                        .remove_attribute("contenteditable")
                        .unwrap()
                }
            }
            false => {
                for index in range {
                    rows.get_with_index(index)
                        .unwrap()
                        .set_attribute("contenteditable", "true")
                        .unwrap()
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    edit.add_event_listener_with_callback("mousedown", edit_callback.as_ref().unchecked_ref())
        .unwrap();
    edit_callback.forget();

    let commit_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let rows = document()
            .get_element_by_id("debugger")
            .unwrap()
            .dyn_into::<web_sys::HtmlTableElement>()
            .expect("should have an HtmlTableElement.")
            .rows();

        let range = 1..9;

        if rows
            .get_with_index(1)
            .unwrap()
            .has_attribute("contenteditable")
        {
            for index in range {
                rows.get_with_index(index)
                    .unwrap()
                    .remove_attribute("contenteditable")
                    .unwrap()

                // find a way (generic if possible) to serialize (with serde?)
                // correclty the values collected and push them into the
                // Emulator struct.
            }
        }
    }) as Box<dyn FnMut(_)>);

    commit
        .add_event_listener_with_callback("mousedown", commit_callback.as_ref().unchecked_ref())
        .unwrap();
    commit_callback.forget();
}

/// Set callbacks to allow hiding the `Debugger` in the GUI.
fn set_show_hide_callback() {
    let callback = Closure::wrap(Box::new(move |_event: web_sys::KeyboardEvent| {
        let _e = document().get_element_by_id("debugger").unwrap();
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
}
