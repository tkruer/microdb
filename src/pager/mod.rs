pub struct Pager {
    file_desc: i32,
    file_len: u32,
    pub num_pages: u32,
    // WARN:: type_cvoid ? pages -> Consider changing this later on?
    pages: u32,
}

impl Pager {
    pub fn open(_filename: &str) -> Self {
        Pager {
            file_desc: 0,
            file_len: 0,
            num_pages: 0,
            pages: 0,
        }
    }
}
