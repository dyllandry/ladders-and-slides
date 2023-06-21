use std::{fs::File, io::Write};

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new() -> Self {
        let file = File::create("ladders_and_slides.log").unwrap();
        Self { file }
    }

    pub fn log(&mut self, message: &str) {
        self.file.write_all(message.as_bytes()).unwrap();
        self.file.write_all(b"\n").unwrap();
    }
}
