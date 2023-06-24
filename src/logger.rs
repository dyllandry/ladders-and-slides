use std::{fs::File, io::Write, cell::RefCell};

pub struct Logger {
    file: RefCell<File>,
}

impl Logger {
    pub fn new() -> Self {
        let file = File::create("ladders_and_slides.log").unwrap();
        Self { file: RefCell::new(file) }
    }

    pub fn log(&self, message: &str) {
        self.file.borrow_mut().write_all(message.as_bytes()).unwrap();
        self.file.borrow_mut().write_all(b"\n").unwrap();
    }
}
