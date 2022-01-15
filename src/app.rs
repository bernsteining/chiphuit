use std::cell::RefCell;
use std::rc::Rc;

pub struct App {
    pub breakpoint: Rc<RefCell<bool>>,
    pub rom_buffer: Rc<RefCell<Vec<u8>>>,
}

impl App {
    pub fn new() -> App {
        App {
            breakpoint: Rc::new(RefCell::new(false)),
            rom_buffer: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
