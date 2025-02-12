use crate::db::{db_close, print_constants, print_leaf_node, MetaCommandResult, Table};
use std::io::{self, Write};

pub struct Buffer {
    pub buffer: String,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            buffer: String::new(),
        }
    }

    pub fn print_prompt() {
        print!("db > ");
        io::stdout().flush().expect("Failed to flush stdout");
    }

    pub fn read_input(&mut self) {
        self.buffer.clear();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                eprintln!("Error reading input");
                std::process::exit(1);
            }
            Ok(_) => {
                self.buffer = input.trim_end().to_string();
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                std::process::exit(1);
            }
        }
    }

    /// Processes meta-commands. If the command is ".exit", closes the database and exits.
    pub fn do_meta_command(&self, table: &mut Table) -> MetaCommandResult {
        if self.buffer == ".exit" {
            db_close(table);
            std::process::exit(0);
        }
        if self.buffer == ".btree" {
            // Get the root page (a mutable slice of bytes) and print it.
            let root_page = table.pager.get_page(table.root_page_num as usize);
            print_leaf_node(root_page);
            MetaCommandResult::Success
        } else if self.buffer == ".constants" {
            println!("Constants:");
            print_constants();
            MetaCommandResult::Success
        } else {
            MetaCommandResult::UnrecognizedCommand
        }
    }
}
