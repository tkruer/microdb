use std::{
    env::{self, args},
    io::{self, Write},
};

mod meta;
mod util;

const TABLE_MAX_PAGES: usize = 400;
const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;

#[derive(Debug)]
enum PrepareResult {
    Success,
    SyntaxError,
    NegativeId,
    StringTooLong,
    UnrecognizedStatement,
}

#[derive(Debug, Default)]
enum StatementType {
    #[default]
    Insert,
    Select,
}

#[derive(Debug, Default)]
struct Row {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Default)]
struct Statement {
    statement_type: StatementType,
    row_to_insert: Option<Row>,
}

#[derive(Debug)]
enum ExecuteResult {
    ExecuteSuccess,
    ExecuteDuplicateKey,
}

#[derive(Debug)]
struct Cursor {
    end_of_table: bool,
    page_num: u32,
    cell_num: u32,
}

impl Table {
    fn start(&self) -> Cursor {
        // Returns the cursor to start reading the table
        Cursor {
            end_of_table: false,
            page_num: 0,
            cell_num: 0,
        }
    }

    fn find(&self, key: u32) -> Cursor {
        // Find a row in the table by key
        Cursor {
            end_of_table: false,
            page_num: 0,
            cell_num: 0,
        }
    }

    fn pager(&self) -> &Pager {
        &self.pager
    }
}

impl Cursor {
    fn advance(&mut self) {
        self.cell_num += 1;
        if self.cell_num >= 10 {
            // Example condition to end the table
            self.end_of_table = true;
        }
    }

    fn value(&self) -> &Row {
        // Placeholder for cursor value
        &Row {
            id: 1,
            username: "John".to_string(),
            email: "john@example.com".to_string(),
        }
    }
}

impl Row {
    fn deserialize(cursor: &Cursor) -> Row {
        // Deserialize the cursor value into a row (stubbed for now)
        cursor.value().clone()
    }
}

impl ExecuteResult {
    fn execute_select(statement: &Statement, table: &Table) -> ExecuteResult {
        let mut cursor = table.start();
        let mut row: Row;

        while !cursor.end_of_table {
            row = Row::deserialize(&cursor);
            print_row(&row); // Print the row (assuming you have a print_row function)
            cursor.advance();
        }

        ExecuteResult::ExecuteSuccess
    }

    fn execute_insert(statement: &Statement, table: &Table) -> ExecuteResult {
        let row_to_insert = &statement.row_to_insert;
        let key_to_insert = row_to_insert.id;
        let mut cursor = table.find(key_to_insert);

        let node = table.pager();
        let num_cells = 10; // Example number of cells in the leaf node

        if cursor.cell_num < num_cells {
            let key_at_index = 1; // Example key_at_index
            if key_at_index == key_to_insert {
                return ExecuteResult::ExecuteDuplicateKey;
            }
        }

        leaf_node_insert(&mut cursor, row_to_insert.id, row_to_insert); // Assuming this function is implemented

        ExecuteResult::ExecuteSuccess
    }

    pub fn execute_statement(statement: &Statement, table: &Table) -> ExecuteResult {
        match statement.statement_type {
            StatementType::Insert => ExecuteResult::execute_insert(statement, table),
            StatementType::Select => ExecuteResult::execute_select(statement, table),
        }
    }
}

fn print_row(row: &Row) {
    println!("Row: {:?}, {:?}, {:?}", row.id, row.username, row.email);
}

fn leaf_node_insert(cursor: &mut Cursor, id: u32, row_to_insert: &Row) {
    // Placeholder for insert logic
    println!("Inserting row with id: {}", id);
}

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

fn do_meta_command(input_buffer: &InputBuffer, table: &Table) -> MetaCommandResult {
    match input_buffer.buffer.as_str() {
        ".exit" => {
            println!("Exiting...");
            table.close();
            std::process::exit(0);
        }
        ".btree" => {
            print_tree("pager_placeholder", 0, 0);
            MetaCommandResult::Success
        }
        ".constants" => {
            print_constants();
            MetaCommandResult::Success
        }
        _ => MetaCommandResult::UnrecognizedCommand,
    }
}

fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    if input_buffer.buffer.starts_with("insert") {
        prepare_insert(input_buffer, statement);
        PrepareResult::Success
    } else if input_buffer.buffer.trim() == "select" {
        PrepareResult::Success
    } else {
        PrepareResult::UnrecognizedStatement
    }
}

fn prepare_insert(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    statement.statement_type = StatementType::Insert;

    let parts: Vec<&str> = input_buffer.buffer.split_whitespace().collect();

    if parts.len() < 3 {
        return PrepareResult::SyntaxError;
    }

    let id_string = parts[1];
    let username = parts[2];
    let email = parts[3];

    let id: i32 = id_string.parse().unwrap_or(-1);
    if id < 0 {
        return PrepareResult::NegativeId;
    }
    if username.len() > COLUMN_USERNAME_SIZE {
        return PrepareResult::StringTooLong;
    }
    if email.len() > COLUMN_EMAIL_SIZE {
        return PrepareResult::StringTooLong;
    }

    if statement.row_to_insert.is_none() {
        statement.row_to_insert = Some(Row {
            id: 0,
            username: String::new(),
            email: String::new(),
        });
    }

    if let Some(row) = &mut statement.row_to_insert {
        row.id = id;
        row.username = username.to_string();
        row.email = email.to_string();
    }

    PrepareResult::Success
}

fn execute_statement(_statement: &str, _table: &Table) -> ExecuteResult {
    // Stub for executing statements
    ExecuteResult::Success
}

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
        util::prompt();
        input_buffer.read_input();

        if input_buffer.buffer.starts_with('.') {
            match meta::do_meta_command(&input_buffer, &table) {
                meta::MetaCommandResult::Success => continue,
                meta::MetaCommandResult::UnrecognizedCommand => {
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

        match execute_statement(&input_buffer.buffer, &table) {
            ExecuteResult::Success => println!("Executed."),
            ExecuteResult::DuplicateKey => eprintln!("Error: Duplicate key."),
        }
    }
}
