//! # A module to view and modify the `Emulator` variables in the GUI.
use crate::cpu::Emulator;
use crate::utils::{
    append_element_to_another, append_to_body, change_view, document, EMULATOR_VARIABLES, read_user_file
};
use js_sys::JsString;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    console, window, Event, FileReader, HtmlElement, HtmlInputElement, HtmlLabelElement,
    HtmlTableCellElement, HtmlTableRowElement, File
};

/// An `Emulator` debugger.
pub struct Debugger {
    pub element: web_sys::HtmlTableElement,
    pub current_snapshot: Rc<RefCell<String>>,
    pub snapshots: Rc<RefCell<Vec<String>>>,
}

impl Debugger {
    /// Returns a Debugger that can be paired to an Emulator.
    ///
    /// # Arguments
    ///
    /// * `element` - A HTMLTableElement to render the Debugger in the GUI.
    /// * `current_snapshot` - Serialized Emulator snapshot during runtime.
    /// * `snapshots` - Stacked serialized Emulator snapshots, when tracing.
    pub fn new() -> Debugger {
        let debugger = create_element();

        Debugger {
            element: debugger,
            current_snapshot: Rc::new(RefCell::new(String::new())),
            snapshots: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Fill the Debugger elements in the GUI.
    pub fn set_debugger(self: &Debugger, emulator: &Emulator) {
        fill_rows(&self.element);

        // 1st row
        edit(&self.element);
        commit(&self.element);

        // 2nd row
        trace(&self.element, &emulator.tracing);
        copy(&self.element, &self.current_snapshot);

        // 3rd row
        load(&self.element);
        set_load_file_reader(&emulator.load_snapshot);
        dump(self);

        // last row
        set_breakpoint_and_keypad_view(&self.element);
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

/// Activate tracing mode of the VM, allowing to save the VM's internal snapshot
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

/// Copy the current VM snapshot in JSON format to clipboard.
fn copy(element: &web_sys::HtmlTableElement, snapshot: &Rc<RefCell<String>>) {
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

    let snapshot_clone = Rc::clone(snapshot);
    let copy_callback = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        window()
            .unwrap()
            .navigator()
            .clipboard()
            .unwrap()
            .write_text(&snapshot_clone.borrow().to_string());
    }) as Box<dyn FnMut(_)>);

    copy.add_event_listener_with_callback("mousedown", copy_callback.as_ref().unchecked_ref())
        .unwrap();
    copy_callback.forget();
}

/// Load a JSON VM snapshot in the `Emulator`.
fn load(element: &web_sys::HtmlTableElement) {
    let row = element
        .insert_row()
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap();

    let load = row.insert_cell().unwrap();

    load.set_id("load");
    load.set_class_name("debugger_button");

    let fileinput: HtmlInputElement = document()
        .create_element("input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();

    fileinput.set_id("load_upload");
    fileinput.set_type("file");
    append_element_to_another(&fileinput, "load");

    let label: HtmlLabelElement = document()
        .create_element("label")
        .unwrap()
        .dyn_into::<HtmlLabelElement>()
        .unwrap();

    label.set_html_for("load_upload");
    label.set_inner_text("load");
    append_element_to_another(&label, "load");
}

/// Set the button to allow the user to supply a VM snapshot to the `Emulator`.
pub fn set_load_file_reader(emulator_load_snapshot: &Rc<RefCell<Option<Emulator>>>) {
    let file_input = document().get_element_by_id("load").unwrap();
    // file_input.clone().dyn_into::<HtmlInputElement>().unwrap().set_type("file");
    let file_reader = FileReader::new().unwrap().dyn_into::<FileReader>().unwrap();

    let handle_load_event = load_user_snapshot(emulator_load_snapshot);
    file_reader.set_onloadend(Some(handle_load_event.as_ref().unchecked_ref()));
    handle_load_event.forget();

    let handle_read_event = read_user_file(file_reader);
    file_input
        .add_event_listener_with_callback("change", handle_read_event.as_ref().unchecked_ref())
        .unwrap();
    handle_read_event.forget();
}

/// Closure to load user input VM snapshot in the Emulator.
pub fn load_user_snapshot(
    emulator_load_snapshot: &Rc<RefCell<Option<Emulator>>>,
) -> Closure<dyn FnMut(Event)> {
    let load_snapshot = Rc::clone(emulator_load_snapshot);
    Closure::wrap(Box::new(move |event: Event| {
        let json: String = event
            .target()
            .unwrap()
            .dyn_into::<FileReader>()
            .unwrap()
            .result()
            .unwrap()
            .dyn_into::<JsString>()
            .unwrap()
            .into();

        let emulator: Result<Emulator, serde_json::Error> = serde_json::from_str(&json);

        match emulator {
            Ok(emulator) => *load_snapshot.borrow_mut() = Some(emulator),
            Err(error)=> console::log_1(
                &format!("The provided JSON failed to Deserialize into an Emulator structure, are you sure you provided a valid JSON?: {}", error).into(),
            ),
        }
    }))
}


/// Save all the traced VM snapshots in JSON format to your disk.
fn dump(debugger: &Debugger) {
    // should pop a file dialog on click so the user can choose a path
    // where to save the JSON.
    // check this https://docs.rs/web-sys/latest/web_sys/struct.File.html

    let rows = debugger.element.rows();

    let dump = rows
        .get_with_index(rows.length() - 1)
        .unwrap()
        .dyn_into::<web_sys::HtmlTableRowElement>()
        .unwrap()
        .insert_cell()
        .unwrap();

    dump.set_class_name("debugger_button");
    dump.set_inner_html("dump");

    // todo: Serialize Debugger.snapshots to JSON and allow to it save on disk.
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
