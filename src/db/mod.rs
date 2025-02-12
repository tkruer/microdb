use std::convert::TryInto;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::os::unix::fs::OpenOptionsExt;
use std::process::exit;

// ─── CONSTANTS ────────────────────────────────────────────────────────────────

// Original row constants:
pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

pub const ID_SIZE: usize = size_of::<u32>(); // 4 bytes
pub const USERNAME_SIZE: usize = COLUMN_USERNAME_SIZE; // 32 bytes
pub const EMAIL_SIZE: usize = COLUMN_EMAIL_SIZE; // 255 bytes
pub const ID_OFFSET: usize = 0;
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

pub const PAGE_SIZE: usize = 4096;
pub const TABLE_MAX_PAGES: usize = 100;
// (We no longer use ROWS_PER_PAGE/TABLE_MAX_ROWS for the B-tree.)

// ─── B-TREE NODE CONSTANTS ─────────────────────────────────────────────────────

// Node header (common to all nodes)
pub const NODE_TYPE_SIZE: usize = size_of::<u8>();
pub const NODE_TYPE_OFFSET: usize = 0;
pub const IS_ROOT_SIZE: usize = size_of::<u8>();
pub const IS_ROOT_OFFSET: usize = NODE_TYPE_SIZE;
pub const PARENT_POINTER_SIZE: usize = size_of::<u32>();
pub const PARENT_POINTER_OFFSET: usize = IS_ROOT_OFFSET + IS_ROOT_SIZE;
pub const COMMON_NODE_HEADER_SIZE: usize = NODE_TYPE_SIZE + IS_ROOT_SIZE + PARENT_POINTER_SIZE;

// Leaf node header layout.
pub const LEAF_NODE_NUM_CELLS_SIZE: usize = size_of::<u32>();
pub const LEAF_NODE_NUM_CELLS_OFFSET: usize = COMMON_NODE_HEADER_SIZE;
pub const LEAF_NODE_HEADER_SIZE: usize = COMMON_NODE_HEADER_SIZE + LEAF_NODE_NUM_CELLS_SIZE;

// Leaf node body layout.
pub const LEAF_NODE_KEY_SIZE: usize = size_of::<u32>();
pub const LEAF_NODE_KEY_OFFSET: usize = 0;
pub const LEAF_NODE_VALUE_SIZE: usize = ROW_SIZE;
pub const LEAF_NODE_VALUE_OFFSET: usize = LEAF_NODE_KEY_OFFSET + LEAF_NODE_KEY_SIZE;
pub const LEAF_NODE_CELL_SIZE: usize = LEAF_NODE_KEY_SIZE + LEAF_NODE_VALUE_SIZE;
pub const LEAF_NODE_SPACE_FOR_CELLS: usize = PAGE_SIZE - LEAF_NODE_HEADER_SIZE;
pub const LEAF_NODE_MAX_CELLS: usize = LEAF_NODE_SPACE_FOR_CELLS / LEAF_NODE_CELL_SIZE;

// ─── ROW DEFINITION ───────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.username, self.email)
    }
}

/// Serializes a row into the destination slice.
pub fn serialize_row(row: &Row, destination: &mut [u8]) {
    destination[ID_OFFSET..ID_OFFSET + ID_SIZE].copy_from_slice(&row.id.to_le_bytes());

    let mut username_bytes = [0u8; USERNAME_SIZE];
    let src = row.username.as_bytes();
    let copy_len = std::cmp::min(src.len(), USERNAME_SIZE);
    username_bytes[..copy_len].copy_from_slice(&src[..copy_len]);
    destination[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE].copy_from_slice(&username_bytes);

    let mut email_bytes = [0u8; EMAIL_SIZE];
    let src = row.email.as_bytes();
    let copy_len = std::cmp::min(src.len(), EMAIL_SIZE);
    email_bytes[..copy_len].copy_from_slice(&src[..copy_len]);
    destination[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE].copy_from_slice(&email_bytes);
}

/// Deserializes a row from the source slice.
pub fn deserialize_row(source: &[u8]) -> Row {
    let id = u32::from_le_bytes(source[ID_OFFSET..ID_OFFSET + ID_SIZE].try_into().unwrap());

    let username_bytes = &source[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE];
    let username = String::from_utf8_lossy(
        &username_bytes[..username_bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(USERNAME_SIZE)],
    )
    .to_string();

    let email_bytes = &source[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE];
    let email = String::from_utf8_lossy(
        &email_bytes[..email_bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(EMAIL_SIZE)],
    )
    .to_string();

    Row {
        id,
        username,
        email,
    }
}

// ─── PAGER AND TABLE STRUCTURES ───────────────────────────────────────────────

/// Pager handles file I/O and caches pages (each page is PAGE_SIZE bytes).
pub struct Pager {
    pub file: File,
    pub file_length: u32,
    pub num_pages: u32,
    pub pages: Vec<Option<Vec<u8>>>,
}

impl Pager {
    /// Returns a mutable reference to the requested page.
    pub fn get_page(&mut self, page_num: usize) -> &mut [u8] {
        if page_num >= TABLE_MAX_PAGES {
            eprintln!(
                "Tried to fetch page number out of bounds. {} > {}",
                page_num, TABLE_MAX_PAGES
            );
            exit(1);
        }

        if self.pages[page_num].is_none() {
            let mut page = vec![0u8; PAGE_SIZE];
            let mut num_pages = (self.file_length as usize) / PAGE_SIZE;
            if self.file_length as usize % PAGE_SIZE != 0 {
                num_pages += 1;
            }
            if page_num < num_pages {
                self.file
                    .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))
                    .expect("Error seeking file");
                let _ = self.file.read(&mut page).expect("Error reading file");
            }
            self.pages[page_num] = Some(page);
            if (page_num as u32) >= self.num_pages {
                self.num_pages = page_num as u32 + 1;
            }
        }
        self.pages[page_num].as_mut().unwrap().as_mut_slice()
    }
}

/// The Table now holds a Pager and, instead of a total row count,
/// uses a B-tree: the root page number indicates the root of the B-tree.
pub struct Table {
    pub pager: Pager,
    pub root_page_num: u32,
}

impl Table {
    /// Returns a mutable slice to the storage slot for a given row.
    /// (For compatibility with earlier code; in the B-tree, we access leaf cells instead.)
    pub fn row_slot(&mut self, _row_num: u32) -> &mut [u8] {
        // In our B-tree, we use leaf node functions instead.
        // Here we return the start of the root page.
        self.pager.get_page(self.root_page_num as usize)
    }
}

// ─── DATABASE OPEN/CLOSE FUNCTIONS ───────────────────────────────────────────

pub fn pager_open(filename: &str) -> Pager {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .mode(0o600)
        .open(filename)
        .expect("Unable to open file");
    let file_length = file.metadata().expect("Failed to get metadata").len() as u32;
    let mut pages = Vec::with_capacity(TABLE_MAX_PAGES);
    pages.resize_with(TABLE_MAX_PAGES, || None);
    let mut num_pages = (file_length as usize) / PAGE_SIZE;
    if file_length as usize % PAGE_SIZE != 0 {
        eprintln!("Db file is not a whole number of pages. Corrupt file.");
        exit(1);
    }
    Pager {
        file,
        file_length,
        num_pages: num_pages as u32,
        pages,
    }
}

pub fn db_open(filename: &str) -> Table {
    let mut pager = pager_open(filename);
    // For a new database, if no pages exist, initialize page 0 as a leaf node.
    if pager.num_pages == 0 {
        let root_node = pager.get_page(0);
        initialize_leaf_node(root_node);
    }
    Table {
        pager,
        root_page_num: 0,
    }
}

/// Flushes a page from memory to disk (writes the entire page).
pub fn pager_flush(pager: &mut Pager, page_num: usize) {
    if pager.pages[page_num].is_none() {
        eprintln!("Tried to flush null page");
        exit(1);
    }
    pager
        .file
        .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))
        .expect("Error seeking file");
    let page = pager.pages[page_num].as_ref().unwrap();
    let bytes_written = pager
        .file
        .write(&page[..PAGE_SIZE])
        .expect("Error writing file");
    if bytes_written != PAGE_SIZE {
        eprintln!(
            "Error writing: wrote {} bytes, expected {}",
            bytes_written, PAGE_SIZE
        );
        exit(1);
    }
}

/// Closes the database by flushing all pages and closing the file.
pub fn db_close(table: &mut Table) {
    let pager = &mut table.pager;
    for i in 0..(pager.num_pages as usize) {
        if pager.pages[i].is_some() {
            pager_flush(pager, i);
            pager.pages[i] = None;
        }
    }
    let result = pager.file.sync_all();
    if result.is_err() {
        eprintln!("Error closing db file.");
        exit(1);
    }
    // Pager file is closed when dropped.
}

// ─── LEAF NODE FUNCTIONS ──────────────────────────────────────────────────────

/// Returns a mutable pointer to the leaf node’s num_cells field.
/// (Here we treat a page as a mutable slice of bytes.)
pub fn leaf_node_num_cells(node: &mut [u8]) -> &mut u32 {
    // Safety: We assume that the slice is at least LEAF_NODE_NUM_CELLS_OFFSET + 4 bytes.
    unsafe { &mut *(node[LEAF_NODE_NUM_CELLS_OFFSET..].as_mut_ptr() as *mut u32) }
}

/// Returns a mutable slice to the cell at cell_num in a leaf node.
pub fn leaf_node_cell(node: &mut [u8], cell_num: usize) -> &mut [u8] {
    let offset = LEAF_NODE_HEADER_SIZE + cell_num * LEAF_NODE_CELL_SIZE;
    &mut node[offset..offset + LEAF_NODE_CELL_SIZE]
}

/// Returns a mutable pointer to the key in the given cell.
pub fn leaf_node_key(node: &mut [u8], cell_num: usize) -> &mut u32 {
    unsafe { &mut *(leaf_node_cell(node, cell_num)[..LEAF_NODE_KEY_SIZE].as_mut_ptr() as *mut u32) }
}

/// Returns a mutable slice to the value portion of the cell.
pub fn leaf_node_value<'a>(node: &'a mut [u8], cell_num: usize) -> &'a mut [u8] {
    let cell = leaf_node_cell(node, cell_num);
    &mut cell[LEAF_NODE_KEY_SIZE..]
}

/// Initializes a leaf node by setting its num_cells to 0.
pub fn initialize_leaf_node(node: &mut [u8]) {
    *leaf_node_num_cells(node) = 0;
}

// ─── CURSOR FOR B-TREE LEAF NODES ─────────────────────────────────────────────

pub struct Cursor {
    pub table: *mut Table, // raw pointer to avoid lifetime issues
    pub page_num: u32,
    pub cell_num: u32,
    pub end_of_table: bool,
}

impl Cursor {
    /// Returns a new cursor pointing to the start (cell 0) of the root leaf node.
    pub fn table_start(table: &mut Table) -> Self {
        let root_page = table.root_page_num;
        let root_node = table.pager.get_page(root_page as usize);
        let num_cells = *leaf_node_num_cells(root_node);
        Cursor {
            table,
            page_num: root_page,
            cell_num: 0,
            end_of_table: num_cells == 0,
        }
    }

    /// Returns a new cursor pointing just past the last cell of the root leaf node.
    pub fn table_end(table: &mut Table) -> Self {
        let root_page = table.root_page_num;
        let root_node = table.pager.get_page(root_page as usize);
        let num_cells = *leaf_node_num_cells(root_node);
        Cursor {
            table,
            page_num: root_page,
            cell_num: num_cells,
            end_of_table: true,
        }
    }

    /// Returns a mutable slice representing the value of the current cell.
    pub fn value(&mut self) -> &mut [u8] {
        // Get the root node, then return its value portion of the cell.
        let table = unsafe { &mut *self.table };
        let node = table.pager.get_page(self.page_num as usize);
        leaf_node_value(node, self.cell_num as usize)
    }

    /// Advances the cursor to the next cell.
    pub fn advance(&mut self) {
        let table = unsafe { &mut *self.table };
        let node = table.pager.get_page(self.page_num as usize);
        self.cell_num += 1;
        if self.cell_num >= *leaf_node_num_cells(node) {
            self.end_of_table = true;
        }
    }
}

// ─── STATEMENT, PREPARATION, EXECUTION ─────────────────────────────────────────

#[derive(Debug)]
pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

#[derive(Debug)]
pub enum PrepareResult {
    Success,
    NegativeId,
    StringTooLong,
    SyntaxError,
    UnrecognizedStatement,
}

#[derive(Debug)]
pub enum StatementType {
    Insert,
    Select,
}

#[derive(Debug)]
pub struct Statement {
    pub statement_type: StatementType,
    pub row_to_insert: Option<Row>,
}

#[derive(Debug)]
pub enum ExecuteResult {
    Success,
    TableFull,
    DuplicateKey,
}

pub fn print_row(row: &Row) {
    println!("{}", row);
}

/// Prepares an insert statement. Expects input: "insert <id> <username> <email>"
pub fn prepare_insert(input: &str, statement: &mut Statement) -> PrepareResult {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 4 {
        return PrepareResult::SyntaxError;
    }
    let id: i32 = match parts[1].parse() {
        Ok(num) => num,
        Err(_) => return PrepareResult::SyntaxError,
    };
    if id < 0 {
        return PrepareResult::NegativeId;
    }
    let username = parts[2].to_string();
    if username.len() > COLUMN_USERNAME_SIZE {
        return PrepareResult::StringTooLong;
    }
    let email = parts[3].to_string();
    if email.len() > COLUMN_EMAIL_SIZE {
        return PrepareResult::StringTooLong;
    }
    statement.statement_type = StatementType::Insert;
    statement.row_to_insert = Some(Row {
        id: id as u32,
        username,
        email,
    });
    PrepareResult::Success
}

/// Prepares a statement.
pub fn prepare_statement(input: &str, statement: &mut Statement) -> PrepareResult {
    if input.starts_with("insert") {
        prepare_insert(input, statement)
    } else if input.trim() == "select" {
        statement.statement_type = StatementType::Select;
        statement.row_to_insert = None;
        PrepareResult::Success
    } else {
        PrepareResult::UnrecognizedStatement
    }
}

/// Inserts a key/value pair into a leaf node.
/// (The key is the row id; the value is the serialized row.)
pub fn leaf_node_insert(cursor: &mut Cursor, key: u32, value: &Row) {
    let table = unsafe { &mut *cursor.table };
    let node = table.pager.get_page(cursor.page_num as usize);
    let num_cells = *leaf_node_num_cells(node) as usize;
    if num_cells >= LEAF_NODE_MAX_CELLS {
        println!("Need to implement splitting a leaf node.");
        exit(1);
    }
    // If insertion point is not at the end, shift cells to the right.
    if (cursor.cell_num as usize) < num_cells {
        // Instead of borrowing node twice, use copy_within on the entire slice.
        for i in (cursor.cell_num as usize..num_cells).rev() {
            let src_offset = LEAF_NODE_HEADER_SIZE + i * LEAF_NODE_CELL_SIZE;
            let dst_offset = LEAF_NODE_HEADER_SIZE + (i + 1) * LEAF_NODE_CELL_SIZE;
            node.copy_within(src_offset..src_offset + LEAF_NODE_CELL_SIZE, dst_offset);
        }
    }
    *leaf_node_num_cells(node) += 1;
    *leaf_node_key(node, cursor.cell_num as usize) = key;
    serialize_row(value, leaf_node_value(node, cursor.cell_num as usize));
}

/// Executes an insert statement by inserting the row into the leaf node.
pub fn execute_insert(statement: &Statement, table: &mut Table) -> ExecuteResult {
    let node = table.pager.get_page(table.root_page_num as usize);
    if *leaf_node_num_cells(node) >= (LEAF_NODE_MAX_CELLS as u32) {
        return ExecuteResult::TableFull;
    }
    if let Some(ref row) = statement.row_to_insert {
        let mut cursor = Cursor::table_end(table);
        leaf_node_insert(&mut cursor, row.id, row);
        ExecuteResult::Success
    } else {
        ExecuteResult::Success
    }
}

/// Executes a select statement by iterating over all leaf cells.
pub fn execute_select(_statement: &Statement, table: &mut Table) -> ExecuteResult {
    let mut cursor = Cursor::table_start(table);
    while !cursor.end_of_table {
        let node = table.pager.get_page(cursor.page_num as usize);
        let cell_offset = cursor.cell_num as usize;
        let key = *leaf_node_key(node, cell_offset);
        // For demonstration, we deserialize the row stored in this cell.
        let row = deserialize_row(leaf_node_value(node, cell_offset));
        print_row(&row);
        cursor.advance();
    }
    ExecuteResult::Success
}

/// Executes a statement.
pub fn execute_statement(statement: &Statement, table: &mut Table) -> ExecuteResult {
    match statement.statement_type {
        StatementType::Insert => execute_insert(statement, table),
        StatementType::Select => execute_select(statement, table),
    }
}

// ─── META COMMANDS ────────────────────────────────────────────────────────────

pub fn print_constants() {
    println!("ROW_SIZE: {}", ROW_SIZE);
    println!("COMMON_NODE_HEADER_SIZE: {}", COMMON_NODE_HEADER_SIZE);
    println!("LEAF_NODE_HEADER_SIZE: {}", LEAF_NODE_HEADER_SIZE);
    println!("LEAF_NODE_CELL_SIZE: {}", LEAF_NODE_CELL_SIZE);
    println!("LEAF_NODE_SPACE_FOR_CELLS: {}", LEAF_NODE_SPACE_FOR_CELLS);
    println!("LEAF_NODE_MAX_CELLS: {}", LEAF_NODE_MAX_CELLS);
}

pub fn print_leaf_node(node: &mut [u8]) {
    let num_cells = *leaf_node_num_cells(node);
    println!("leaf (size {})", num_cells);
    for i in 0..(num_cells as usize) {
        let key = *leaf_node_key(node, i);
        println!("  - {} : {}", i, key);
    }
}

/// Processes meta-commands. Recognizes ".exit", ".btree", and ".constants".
pub fn do_meta_command(input: &str, table: &mut Table) -> MetaCommandResult {
    if input == ".exit" {
        db_close(table);
        exit(0);
    } else if input == ".btree" {
        println!("Tree:");
        let root_node = table.pager.get_page(table.root_page_num as usize);
        print_leaf_node(root_node);
        return MetaCommandResult::Success;
    } else if input == ".constants" {
        println!("Constants:");
        print_constants();
        return MetaCommandResult::Success;
    } else {
        MetaCommandResult::UnrecognizedCommand
    }
}
