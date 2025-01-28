use crate::{cursor::Cursor, pager::Pager};

pub struct Table {
    pager: Box<Pager>,
    root_page_num: u32,
}

impl Table {
    pub fn open(filename: &str) -> Self {
        let pager = Box::new(Pager::open(filename));

        let table = Table {
            pager,
            root_page_num: 0,
        };

        if table.pager.num_pages == 0 {
            let root_node = table.get_page(0);
            table.initialize_leaf_node(root_node);
            table.set_node_root(root_node, true);
        }
        table
    }

    pub fn close(&self) {
        println!("Closing database...");
    }

    pub fn get_page(&self, _page_num: u32) -> *mut u8 {
        std::ptr::null_mut()
    }

    fn initialize_leaf_node(&self, root_node: *mut u8) {
        println!("Initializing leaf node at address {:?}", root_node);
    }

    fn set_node_root(&self, _root_node: *mut u8, is_root: bool) {
        println!("Setting node root status to: {}", is_root);
    }

    pub fn find(&self, _key: i32) -> Option<Cursor> {
        Some(Cursor {
            page_num: 0,
            cell_num: 0,
            end_of_table: false,
        })
    }

    pub fn start(&self) -> Cursor {
        Cursor {
            page_num: 0,
            cell_num: 0,
            end_of_table: false,
        }
    }
}
