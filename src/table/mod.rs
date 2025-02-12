use crate::{cursor::Cursor, node::NodeType, pager::Pager};

#[derive(Debug)]
pub struct Table {
    pub pager: Pager,
    pub root_page_num: u32,
}

impl Table {
    /// Open a database file and initialize the table.
    pub fn open(filename: &str) -> Self {
        let mut pager = Pager::open(filename);
        let root_page_num = 0;
        if pager.num_pages == 0 {
            let root = pager.get_page(0);
            // initialize as a leaf node
            root.set_type(NodeType::Leaf);
            root.set_root(true);
            // Set additional fields for an empty leaf node…
        }
        Self {
            pager,
            root_page_num,
        }
    }

    /// Find the correct cursor position for the given key.
    pub fn table_find(&mut self, key: u32) -> Cursor {
        let root = self.pager.get_page(self.root_page_num);
        if root.get_type() == NodeType::Leaf {
            Self::leaf_node_find(self, self.root_page_num, key)
        } else {
            // For simplicity, assume the tree is a leaf.
            // Otherwise you’d recursively search internal nodes.
            unimplemented!("Internal node search not implemented");
        }
    }

    /// Find the correct leaf node for a key (binary search).
    pub fn leaf_node_find(&mut self, page_num: u32, key: u32) -> Cursor {
        let node = self.pager.get_page(page_num);
        let num_cells = node.leaf_num_cells();
        let mut min_index = 0;
        let mut max_index = num_cells;
        while min_index != max_index {
            let index = (min_index + max_index) / 2;
            let key_at_index = node.leaf_key(index as usize);
            if key == key_at_index {
                return Cursor {
                    table: self,
                    page_num,
                    cell_num: index,
                    end_of_table: false,
                };
            }
            if key < key_at_index {
                max_index = index;
            } else {
                min_index = index + 1;
            }
        }
        Cursor {
            table: self,
            page_num,
            cell_num: min_index,
            end_of_table: false,
        }
    }

    /// Returns a cursor at the start of the table.
    pub fn table_start(&mut self) -> Cursor {
        // We assume table_find(0) returns a cursor.
        let cursor = self.table_find(0);
        // To avoid overlapping mutable borrows, extract the needed value in a block.
        let num_cells = {
            let node = self.pager.get_page(cursor.page_num);
            node.leaf_num_cells()
        };
        Cursor {
            end_of_table: num_cells == 0,
            ..cursor
        }
    }

    /// (Optional) A method to close the table (flush pages, etc.)
    pub fn close(&mut self) {
        self.pager.flush_all();
    }
}
