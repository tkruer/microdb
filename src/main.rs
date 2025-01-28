use std::env;

use microdb::{
    buffer::*,
    engine::{self, *},
    meta::*,
    table::{self, Table},
    util::*,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("[ERROR]: Did not supply a database name");
        std::process::exit(1);
    }

    let file_name = args[1].clone();
    let table = Table::open(&file_name);
    let mut input_buffer = InputBuffer::new();

    loop {
        prompt();
        input_buffer.read_input();

        if input_buffer.buffer.starts_with('.') {
            match do_meta_command(&input_buffer, &table) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::UnrecognizedCommand => {
                    eprintln!("Unrecognized command '{}'", input_buffer.buffer);
                    continue;
                }
            }
        }

        let mut statement: Statement = Statement::default();

        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::Success => {}
            PrepareResult::NegativeId => {
                eprintln!("ID must be positive.");
                continue;
            }
            PrepareResult::StringTooLong => {
                eprintln!("String is too long.");
                continue;
            }
            PrepareResult::SyntaxError => {
                eprintln!("Syntax error. Could not parse statement.");
                continue;
            }
            PrepareResult::UnrecognizedStatement => {
                eprintln!(
                    "Unrecognized keyword at start of '{}'.",
                    input_buffer.buffer
                );
                continue;
            }
        }
        match engine.execute_statement(&statement, table) {
            ExecuteResult::Success => println!("Executed."),
            ExecuteResult::DuplicateKey => eprintln!("Error: Duplicate key."),
        }
    }
}
