use crate::table::Table;

#[derive(Debug)]
pub struct Cursor {
    // Instead of a mutable reference, we store a raw pointer.
    // (Accessing the table later will require unsafe blocks.)
    pub table: *mut Table,
    pub page_num: u32,
    pub cell_num: u32,
    pub end_of_table: bool,
}

impl Cursor {
    /// Advances the cursor to the next cell.
    pub fn advance(&mut self) {
        // To get a mutable reference from our raw pointer, we use unsafe.
        unsafe {
            let table = &mut *self.table;
            let node = table.pager.get_page(self.page_num);
            self.cell_num += 1;
            if self.cell_num >= node.leaf_num_cells() {
                // For simplicity, we assume that if we’ve run out of cells, we are at the end.
                self.end_of_table = true;
            }
        }
    }
}
