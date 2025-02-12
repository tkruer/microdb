pub mod btree;
pub mod buffer;
pub mod constants;
pub mod cursor;
pub mod engine;
pub mod meta;
pub mod node;
pub mod pager;
pub mod serializer;
pub mod table;
pub mod util;

use crate::buffer::*;
use crate::engine::*;
use crate::meta::*;
use crate::table::*;
use crate::util::*;

/// The MicroDB builder.
#[derive(Clone, Debug, Default)]
pub struct MicroDB;

impl MicroDB {
    /// Create a new MicroDB builder.
    pub fn new() -> Self {
        MicroDB
    }

    /// Consume the builder by passing in command-line arguments.
    /// This method extracts the database file name from the arguments,
    /// opens the table, and creates a new Database instance.
    pub fn builder(self, args: Vec<String>) -> Database {
        if args.len() < 2 {
            eprintln!("[ERROR]: Did not supply a database name");
            std::process::exit(1);
        }
        let file_name = args[1].clone();
        let table = Table::open(&file_name);
        let engine = Engine::default();
        Database { table, engine }
    }
}

/// The database that holds our table and engine.
pub struct Database {
    pub table: Table,
    pub engine: Engine,
}

impl Database {
    /// Run the interactive loop.
    pub fn run(&mut self) {
        let mut input_buffer = InputBuffer::new();

        loop {
            prompt();
            input_buffer.read_input();

            if input_buffer.buffer.starts_with('.') {
                match do_meta_command(&input_buffer, &mut self.table) {
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
            match self.engine.execute_statement(&statement, &mut self.table) {
                ExecuteResult::Success => println!("Executed."),
                ExecuteResult::DuplicateKey => eprintln!("Error: Duplicate key."),
            }
        }
    }
}
