use std::io::{self, Write};

pub fn prompt() {
    print!("[INFO] db > ");
    io::stdout().flush().unwrap();
}
