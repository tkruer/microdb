use std::io::{self, Write};

#[derive(Debug, Default)]
pub struct InputBuffer {
    pub buffer: String,
    pub buffer_len: usize,
    // TODO: change later?
    pub input_len: isize,
}

impl InputBuffer {
    pub fn new() -> Self {
        InputBuffer {
            buffer: String::new(),
            buffer_len: 0,
            input_len: 0,
        }
    }

    pub fn read_input(&mut self) {
        self.buffer.clear();
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut self.buffer).is_err() {
            eprintln!("Error reading input");
            std::process::exit(1);
        }

        if self.buffer.ends_with('\n') {
            self.buffer.pop();
            if self.buffer.ends_with('\r') {
                self.buffer.pop();
            }
        }
    }
}
