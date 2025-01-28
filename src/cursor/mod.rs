#[derive(Debug)]
pub struct Cursor {
    pub end_of_table: bool,
    pub page_num: u32,
    pub cell_num: u32,
}

impl Cursor {
    pub fn advance(&mut self) {
        self.cell_num += 1;
        self.end_of_table = self.cell_num > 10; // Example condition
    }

    fn value(&self) -> *const u8 {
        self.page_num as *const u8
    }
}
