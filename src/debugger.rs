//! # A module to view and modify the `Emulator` variables in the GUI.
use crate::cpu::Emulator;
use crate::utils::{append_to_body, change_view, document, EMULATOR_VARIABLES};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlTableCellElement, HtmlTableRowElement, window};

/// An `Emulator` debugger.
pub struct Debugger {
    pub element: web_sys::HtmlTableElement,
}

impl Debugger {
    /// Instanciate `Debugger` in the GUI with all necessary callbacks.
    pub fn new(emulator: &Emulator) -> Debugger {
        let debugger = create_element();
        fill_rows(&debugger);

        // 1st row
        edit(&debugger);
        commit(&debugger);

        // 2nd row
        trace(&debugger, &emulator.tracing);
        copy(&debugger, emulator);

        // 3rd row
        load(&debugger, emulator);
        dump(&debugger, emulator);

        // last row
        set_breakpoint_and_keypad_view(&debugger);
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
    append_to_body(&element);
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

/// Activate tracing mode of the VM, allowing to save the VM's internal state
/// at each CPU cycle, and to save it in JSON format.
fn trace(element: &web_sys::HtmlTableElement, tracing: &Rc<RefCell<bool>>) {
    let row = element
        .insert_row()
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap();

    let trace = row.insert_cell().unwrap();

    trace.set_class_name("debugger_button");
    trace.set_inner_html("trace");

    let trace_clone = Rc::clone(tracing);
    let trace_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        *trace_clone.borrow_mut() ^= true;
    }) as Box<dyn FnMut(_)>);

    trace
        .add_event_listener_with_callback("mousedown", trace_callback.as_ref().unchecked_ref())
        .unwrap();
    trace_callback.forget();
}

/// Copy the current VM state in JSON format to clipboard.
fn copy(element: &web_sys::HtmlTableElement, emulator: &Emulator) {
    let rows = element.rows();

    let copy = rows
        .get_with_index(rows.length() - 1)
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap()
        .insert_cell()
        .unwrap();

    copy.set_class_name("debugger_button");
    copy.set_inner_html("copy to 📋");

    let vm_state = serde_json::to_string_pretty(&emulator).unwrap();
    let copy_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        window().unwrap().navigator().clipboard().unwrap().write_text(&vm_state);
    }) as Box<dyn FnMut(_)>);

    copy
        .add_event_listener_with_callback("mousedown", copy_callback.as_ref().unchecked_ref())
        .unwrap();
    copy_callback.forget();
}

/// Load a JSON VM state in the `Emulator`.
fn load(element: &web_sys::HtmlTableElement, emulator: &Emulator){
    let row = element
        .insert_row()
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap();

    let trace = row.insert_cell().unwrap();

    trace.set_class_name("debugger_button");
    trace.set_inner_html("load");
}

/// Save all the traced VM states in JSON format to your disk.
fn dump(element: &web_sys::HtmlTableElement, emulator: &Emulator){

    // should pop a file dialog on click so the user can choose a path
    // where to save the JSON.

    let rows = element.rows();

    let dump = rows
        .get_with_index(rows.length() - 1)
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap()
        .insert_cell()
        .unwrap();

        dump.set_class_name("debugger_button");
        dump.set_inner_html("dump");

}


/// Set callbacks to allow modification of the `Emulator`'s fields.
fn edit(element: &web_sys::HtmlTableElement) {
    let edit = element
        .insert_row()
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap()
        .insert_cell()
        .unwrap();

    edit.set_class_name("debugger_button");
    edit.set_id("debugger_edit");
    edit.set_inner_html("edit");

    let edit_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let rows = document()
            .get_element_by_id("debugger")
            .unwrap()
            .dyn_into::<web_sys::HtmlTableElement>()
            .expect("should have an HtmlTableElement.")
            .rows();

        let range = 1..EMULATOR_VARIABLES.len();

        match rows
            .get_with_index(1)
            .unwrap()
            .has_attribute("contenteditable")
        {
            true => {
                for index in range {
                    get_value_cell_from_nth_row(&rows, index as u32)
                        .remove_attribute("contenteditable")
                        .unwrap()
                }
            }
            false => {
                for index in range {
                    get_value_cell_from_nth_row(&rows, index as u32)
                        .set_attribute("contenteditable", "true")
                        .unwrap()
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    edit.add_event_listener_with_callback("mousedown", edit_callback.as_ref().unchecked_ref())
        .unwrap();
    edit_callback.forget();
}

/// Set callback to modify an `Emulator` struct in the GUI.
fn commit(element: &web_sys::HtmlTableElement) {
    let rows = element.rows();

    let commit = rows
        .get_with_index(rows.length() - 1)
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap()
        .insert_cell()
        .unwrap();

    commit.set_class_name("debugger_button");
    commit.set_inner_html("commit");

    let commit_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        let rows = document()
            .get_element_by_id("debugger")
            .unwrap()
            .dyn_into::<web_sys::HtmlTableElement>()
            .expect("should have an HtmlTableElement.")
            .rows();

        let range = 1..EMULATOR_VARIABLES.len();

        if get_value_cell_from_nth_row(&rows, 1).has_attribute("contenteditable") {
            for index in range {
                get_value_cell_from_nth_row(&rows, index as u32)
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

/// Set button to go back to keypad view and to play/pause in debugger view.
fn set_breakpoint_and_keypad_view(element: &web_sys::HtmlTableElement) {
    let row = element
        .insert_row()
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap();

    let breakpoint = row.insert_cell().unwrap();

    breakpoint.set_class_name("debugger_button");
    breakpoint.set_id("debugger_breakpoint");
    breakpoint.set_inner_html("⏯");

    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        document()
            .get_element_by_id("breakpoint")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .click();
    }) as Box<dyn FnMut(_)>);

    breakpoint
        .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    let keypad = row.insert_cell().unwrap();

    keypad.set_class_name("debugger_button");
    keypad.set_id("keypad_view");
    keypad.set_inner_html("↩");

    let closure = change_view();

    keypad
        .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget()
}

/// A helper function to get the value of the `Debugger` at a specific row.
fn get_value_cell_from_nth_row(
    rows: &web_sys::HtmlCollection,
    row_index: u32,
) -> HtmlTableCellElement {
    rows.get_with_index(row_index)
        .unwrap()
        .dyn_into::<HtmlTableRowElement>()
        .unwrap()
        .cells()
        .item(1)
        .unwrap()
        .dyn_into::<HtmlTableCellElement>()
        .unwrap()
}
