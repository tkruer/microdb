pub mod buffer;
pub mod db;

use buffer::Buffer;
use db::{
    db_open, execute_statement, prepare_statement, ExecuteResult, MetaCommandResult, PrepareResult,
    Statement, StatementType, Table,
};

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
        Database
    }
}

/// The database that holds our table and engine.
#[derive(Clone, Debug, Default)]
pub struct Database;

impl Database {
    /// Run the interactive loop.
    pub fn run(&mut self) {
        // Create a new table and input buffer.
        if std::env::args().len() < 2 {
            println!("Must supply a database filename.");
            std::process::exit(1);
        }
        let filename = std::env::args().nth(1).unwrap();
        let mut table = db_open(&filename);
        let mut buffer = Buffer::new();

        loop {
            Buffer::print_prompt();
            buffer.read_input();

            if buffer.buffer.starts_with('.') {
                match buffer.do_meta_command(&mut table) {
                    MetaCommandResult::Success => continue,
                    MetaCommandResult::UnrecognizedCommand => {
                        println!("Unrecognized command '{}'", buffer.buffer);
                        continue;
                    }
                }
            }

            let mut statement = Statement {
                statement_type: StatementType::Select, // default value
                row_to_insert: None,
            };

            match prepare_statement(&buffer.buffer, &mut statement) {
                PrepareResult::Success => { /* proceed */ }
                PrepareResult::NegativeId => {
                    println!("ID must be positive.");
                    continue;
                }
                PrepareResult::StringTooLong => {
                    println!("String is too long.");
                    continue;
                }
                PrepareResult::SyntaxError => {
                    println!("Syntax error. Could not parse statement.");
                    continue;
                }
                PrepareResult::UnrecognizedStatement => {
                    println!("Unrecognized keyword at start of '{}'.", buffer.buffer);
                    continue;
                }
            }

            match execute_statement(&statement, &mut table) {
                ExecuteResult::Success => println!("Executed."),
                ExecuteResult::TableFull => println!("Error: Table full."),
                ExecuteResult::DuplicateKey => println!("Error: Duplicate key."),
            }
        }
    }
}
