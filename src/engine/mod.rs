use crate::{buffer::InputBuffer, table::Table};

#[derive(Debug)]
pub enum StatementType {
    Insert,
    Select,
}

#[derive(Debug, Default)]
pub struct Statement {
    pub statement_type: Option<StatementType>,
    pub row_to_insert: Option<Row>,
}

#[derive(Debug)]
pub enum ExecuteResult {
    Success,
    DuplicateKey,
}

#[derive(Debug)]
pub struct Row {
    pub id: i32,
    pub username: String,
    pub email: String,
}

fn print_row(row: &Row) {
    println!("Row: {:?}, {:?}, {:?}", row.id, row.username, row.email);
}

#[derive(Debug)]
pub enum PrepareResult {
    Success,
    UnrecognizedStatement,
    SyntaxError,
    NegativeId,
    StringTooLong,
}

pub fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    let buffer = input_buffer.buffer.trim();
    if buffer.to_ascii_lowercase().starts_with("insert") {
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

#[derive(Default)]
pub struct Engine;

impl Engine {
    /// Execute an insert statement.
    pub fn execute_insert(&mut self, statement: &Statement, table: &mut Table) -> ExecuteResult {
        let row_to_insert = statement
            .row_to_insert
            .as_ref()
            .expect("Insert requires a row");
        let key_to_insert = row_to_insert.id;

        // Obtain a cursor for the key.
        let cursor = table.table_find(key_to_insert as u32);
        // To avoid overlapping mutable borrows, extract the page number first.
        let page_num = cursor.page_num;
        let node = table.pager.get_page(page_num);
        if cursor.end_of_table {
            // Here you would insert into the leaf node.
            // e.g. leaf_node_insert(&mut cursor, key_to_insert as u32, row_to_insert);
            return ExecuteResult::Success;
        }
        let key_at_index = node.leaf_key(cursor.cell_num as usize);
        if key_at_index == key_to_insert as u32 {
            return ExecuteResult::DuplicateKey;
        }
        // Otherwise, perform the insertion.
        // e.g. leaf_node_insert(&mut cursor, key_to_insert as u32, row_to_insert);
        ExecuteResult::Success
    }

    /// Execute a select statement.
    pub fn execute_select(&mut self, _statement: &Statement, table: &mut Table) -> ExecuteResult {
        let mut cursor = table.table_start();
        while !cursor.end_of_table {
            print_row(&Row {
                id: 0,
                username: "dummy".to_string(),
                email: "dummy@example.com".to_string(),
            });
            cursor.advance();
        }
        ExecuteResult::Success
    }

    pub fn execute_statement(&mut self, statement: &Statement, table: &mut Table) -> ExecuteResult {
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
