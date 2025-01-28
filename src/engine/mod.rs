use crate::{buffer::InputBuffer, constants::*, cursor::*, table::*};

#[derive(Debug)]
pub enum StatementType {
    Insert,
    Select,
}

#[derive(Debug, Default)]
pub struct Statement {
    statement_type: Option<StatementType>,
    row_to_insert: Option<Row>,
}

#[derive(Debug)]
pub enum ExecuteResult {
    Success,
    DuplicateKey,
}

#[derive(Debug)]
pub struct Row {
    id: i32,
    username: String,
    email: String,
}

pub struct Cursor {
    pub end_of_table: bool,
}

impl Cursor {
    pub fn advance(&mut self) {}

    pub fn value(&self) -> &[u8] {
        &[]
    }
}

pub struct Table;

impl Table {
    pub fn find(&self, _key: i32) -> Option<Cursor> {
        Some(Cursor {
            end_of_table: false,
        })
    }

    pub fn start(&self) -> Cursor {
        Cursor {
            end_of_table: false,
        }
    }

    pub fn get_page(&self, _page_num: u32) -> *mut u8 {
        std::ptr::null_mut()
    }
}

fn print_row(row: &Row) {
    println!("Row: {:?}, {:?}, {:?}", row.id, row.username, row.email);
}

fn leaf_node_insert(_cursor: &mut Cursor, _id: u32, _row_to_insert: &Row) {
    println!("Inserting row");
}

pub enum PrepareResult {
    Success,
    UnrecognizedStatement,
    SyntaxError,
    NegativeId,
    StringTooLong,
}

pub fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    let buffer = input_buffer.buffer.trim();

    if buffer.starts_with("insert") {
        prepare_insert(input_buffer, statement);
        PrepareResult::Success
    } else if buffer == "select" {
        statement.statement_type = Some(StatementType::Select);
        PrepareResult::Success
    } else {
        PrepareResult::UnrecognizedStatement
    }
}

pub fn prepare_insert(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    let parts: Vec<&str> = input_buffer.buffer.split_whitespace().collect();

    if parts.len() < 4 {
        return PrepareResult::SyntaxError;
    }

    let id: i32 = parts[1].parse().unwrap_or(-1);
    if id < 0 {
        return PrepareResult::NegativeId;
    }

    let username = parts[2];
    let email = parts[3];

    if username.len() > 255 || email.len() > 255 {
        return PrepareResult::StringTooLong;
    }

    statement.statement_type = Some(StatementType::Insert);
    statement.row_to_insert = Some(Row {
        id,
        username: username.to_string(),
        email: email.to_string(),
    });

    PrepareResult::Success
}

pub struct Engine;

impl Engine {
    pub fn execute_insert(&self, statement: &Statement, table: &Table) -> ExecuteResult {
        let row_to_insert = statement
            .row_to_insert
            .as_ref()
            .expect("Insert requires a row");
        let key_to_insert = row_to_insert.id;
        let mut cursor = table
            .find(key_to_insert)
            .expect("Cursor should not be None");

        let node = table.get_page(0);
        let num_cells = unsafe { *(node as *const u32) };

        if cursor.end_of_table {
            leaf_node_insert(&mut cursor, key_to_insert as u32, row_to_insert);
            return ExecuteResult::Success;
        }

        let key_at_index = unsafe { *(node as *const u32) };
        if key_at_index == key_to_insert as u32 {
            return ExecuteResult::DuplicateKey;
        }

        leaf_node_insert(&mut cursor, key_to_insert as u32, row_to_insert);
        ExecuteResult::Success
    }

    pub fn execute_select(&self, _statement: &Statement, table: &Table) -> ExecuteResult {
        let mut cursor = table.start();

        let mut row = Row {
            id: 0,
            username: String::new(),
            email: String::new(),
        };

        while !cursor.end_of_table {
            print_row(&row);
            cursor.advance();
        }

        ExecuteResult::Success
    }

    pub fn execute_statement(&self, statement: &Statement, table: &Table) -> ExecuteResult {
        match statement.statement_type {
            Some(StatementType::Insert) => self.execute_insert(statement, table),
            Some(StatementType::Select) => self.execute_select(statement, table),
            None => {
                println!("Error: No statement type specified.");
                ExecuteResult::Success
            }
        }
    }
}
