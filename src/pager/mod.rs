use crate::constants::{PAGE_SIZE, TABLE_MAX_PAGES};
use crate::node::Node;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct Pager {
    pub file: File,
    pub file_length: u32,
    pub num_pages: u32,
    pub pages: Vec<Option<Node>>,
}

impl Pager {
    pub fn open(filename: &str) -> Self {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .expect("Unable to open file");
        let file_length = file.metadata().unwrap().len() as u32;
        if (file_length as usize) % PAGE_SIZE != 0 {
            panic!("Db file is not a whole number of pages. Corrupt file.");
        }
        Self {
            file,
            file_length,
            num_pages: (file_length as usize / PAGE_SIZE) as u32,
            pages: vec![None; TABLE_MAX_PAGES], // use None, not &None
        }
    }

    /// Returns a mutable reference to the page (Node) for the given page number.
    pub fn get_page(&mut self, page_num: u32) -> &mut Node {
        if (page_num as usize) >= TABLE_MAX_PAGES {
            panic!(
                "Tried to fetch page number out of bounds: {} > {}",
                page_num, TABLE_MAX_PAGES
            );
        }

        if self.pages[page_num as usize].is_none() {
            let mut node = Node::new();

            // (For simplicity we assume a new page is zero‐initialized.
            // In a real system, you would load the page from disk if it exists.)
            if page_num < self.num_pages {
                self.file
                    .seek(SeekFrom::Start((page_num as usize * PAGE_SIZE) as u64))
                    .unwrap();
                let mut buf = vec![0u8; PAGE_SIZE];
                self.file.read_exact(&mut buf).unwrap();
                node.data.copy_from_slice(&buf);
            }
            self.pages[page_num as usize] = Some(node);
            if page_num >= self.num_pages {
                self.num_pages = page_num + 1;
            }
        }

        self.pages[page_num as usize]
            .as_mut()
            .expect("Page should be loaded")
    }

    /// Write all pages back to disk.
    pub fn flush_all(&mut self) {
        for page_num in 0..self.num_pages {
            if self.pages[page_num as usize].is_some() {
                self.flush(page_num);
            }
        }
    }

    pub fn flush(&mut self, page_num: u32) {
        if let Some(node) = &self.pages[page_num as usize] {
            self.file
                .seek(SeekFrom::Start((page_num as usize * PAGE_SIZE) as u64))
                .unwrap();
            self.file.write_all(&node.data).unwrap();
        } else {
            panic!("Tried to flush null page");
        }
    }
}
