use crate::constants::*;
use std::convert::TryInto;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Internal = 0,
    Leaf = 1,
}

impl NodeType {
    pub fn from(byte: u8) -> Self {
        match byte {
            0 => NodeType::Internal,
            1 => NodeType::Leaf,
            other => panic!("Invalid node type: {}", other),
        }
    }
}

/// A Node wraps an entire page (PAGE_SIZE bytes)
#[derive(Debug, Clone)]
pub struct Node {
    pub data: [u8; PAGE_SIZE],
}

impl Node {
    pub fn new() -> Self {
        Self {
            data: [0; PAGE_SIZE],
        }
    }

    pub fn internal_child(&self, cell_num: usize) -> u32 {
        let start = crate::constants::INTERNAL_NODE_HEADER_SIZE
            + cell_num * crate::constants::INTERNAL_NODE_CELL_SIZE;
        u32::from_le_bytes(self.data[start..start + 4].try_into().unwrap())
    }

    pub fn internal_key(&self, cell_num: usize) -> u32 {
        let start = crate::constants::INTERNAL_NODE_HEADER_SIZE
            + cell_num * crate::constants::INTERNAL_NODE_CELL_SIZE
            + 4;
        u32::from_le_bytes(self.data[start..start + 4].try_into().unwrap())
    }

    pub fn internal_num_keys(&self) -> u32 {
        u32::from_le_bytes(
            self.data[crate::constants::INTERNAL_NODE_NUM_KEYS_OFFSET
                ..crate::constants::INTERNAL_NODE_NUM_KEYS_OFFSET + 4]
                .try_into()
                .unwrap(),
        )
    }

    pub fn internal_right_child(&self) -> u32 {
        u32::from_le_bytes(
            self.data[crate::constants::INTERNAL_NODE_RIGHT_CHILD_OFFSET
                ..crate::constants::INTERNAL_NODE_RIGHT_CHILD_OFFSET + 4]
                .try_into()
                .unwrap(),
        )
    }

    /// Get the node’s type (stored at NODE_TYPE_OFFSET)
    pub fn get_type(&self) -> NodeType {
        NodeType::from(self.data[NODE_TYPE_OFFSET])
    }

    /// Set the node’s type.
    pub fn set_type(&mut self, node_type: NodeType) {
        self.data[NODE_TYPE_OFFSET] = node_type as u8;
    }

    /// Whether the node is marked as the root.
    pub fn is_root(&self) -> bool {
        self.data[IS_ROOT_OFFSET] != 0
    }

    pub fn set_root(&mut self, is_root: bool) {
        self.data[IS_ROOT_OFFSET] = if is_root { 1 } else { 0 };
    }

    /// Get the parent pointer (stored as little-endian u32)
    pub fn get_parent(&self) -> u32 {
        u32::from_le_bytes(
            self.data[PARENT_POINTER_OFFSET..PARENT_POINTER_OFFSET + 4]
                .try_into()
                .unwrap(),
        )
    }

    pub fn set_parent(&mut self, parent: u32) {
        self.data[PARENT_POINTER_OFFSET..PARENT_POINTER_OFFSET + 4]
            .copy_from_slice(&parent.to_le_bytes());
    }

    // --- Internal node helpers ---

    pub fn set_internal_num_keys(&mut self, num_keys: u32) {
        self.data[INTERNAL_NODE_NUM_KEYS_OFFSET..INTERNAL_NODE_NUM_KEYS_OFFSET + 4]
            .copy_from_slice(&num_keys.to_le_bytes());
    }

    pub fn set_internal_right_child(&mut self, child: u32) {
        self.data[INTERNAL_NODE_RIGHT_CHILD_OFFSET..INTERNAL_NODE_RIGHT_CHILD_OFFSET + 4]
            .copy_from_slice(&child.to_le_bytes());
    }

    /// Get a mutable slice to the cell at the given index.
    pub fn internal_cell_mut(&mut self, cell_num: usize) -> &mut [u8] {
        let start = INTERNAL_NODE_HEADER_SIZE + cell_num * INTERNAL_NODE_CELL_SIZE;
        &mut self.data[start..start + INTERNAL_NODE_CELL_SIZE]
    }

    // --- Leaf node helpers ---

    pub fn leaf_num_cells(&self) -> u32 {
        u32::from_le_bytes(
            self.data[LEAF_NODE_NUM_CELLS_OFFSET..LEAF_NODE_NUM_CELLS_OFFSET + 4]
                .try_into()
                .unwrap(),
        )
    }

    pub fn set_leaf_num_cells(&mut self, num: u32) {
        self.data[LEAF_NODE_NUM_CELLS_OFFSET..LEAF_NODE_NUM_CELLS_OFFSET + 4]
            .copy_from_slice(&num.to_le_bytes());
    }

    pub fn leaf_next_leaf(&self) -> u32 {
        u32::from_le_bytes(
            self.data[LEAF_NODE_NEXT_LEAF_OFFSET..LEAF_NODE_NEXT_LEAF_OFFSET + 4]
                .try_into()
                .unwrap(),
        )
    }

    pub fn set_leaf_next_leaf(&mut self, next: u32) {
        self.data[LEAF_NODE_NEXT_LEAF_OFFSET..LEAF_NODE_NEXT_LEAF_OFFSET + 4]
            .copy_from_slice(&next.to_le_bytes());
    }

    /// Get a mutable slice corresponding to the leaf cell at cell_num.
    pub fn leaf_cell_mut(&mut self, cell_num: usize) -> &mut [u8] {
        let start = LEAF_NODE_HEADER_SIZE + cell_num * LEAF_NODE_CELL_SIZE;
        &mut self.data[start..start + LEAF_NODE_CELL_SIZE]
    }

    /// In a leaf node the key is at the beginning of the cell.
    pub fn leaf_key(&self, cell_num: usize) -> u32 {
        let start = LEAF_NODE_HEADER_SIZE + cell_num * LEAF_NODE_CELL_SIZE;
        u32::from_le_bytes(self.data[start..start + 4].try_into().unwrap())
    }
}
