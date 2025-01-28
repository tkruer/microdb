const ORDER: usize = 3;

#[derive(Debug, Clone)]
struct BTree<K: Ord, V> {
    root: Node<K, V>,
}

#[derive(Debug, Clone)]
enum Node<K: Ord, V> {
    Internal(InternalNode<K, V>),
    Leaf(LeafNode<K, V>),
}

#[derive(Debug, Clone)]
struct InternalNode<K: Ord, V> {
    keys: Vec<K>,
    children: Vec<Node<K, V>>,
}

#[derive(Debug, Clone)]
struct LeafNode<K: Ord, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: Ord + Clone, V: Clone> BTree<K, V> {
    pub fn new() -> Self {
        BTree {
            root: Node::Leaf(LeafNode {
                keys: Vec::new(),
                values: Vec::new(),
            }),
        }
    }

    pub fn search(&self, key: &K) -> Option<V> {
        match &self.root {
            Node::Leaf(leaf) => Self::search_leaf(leaf, key),
            Node::Internal(internal) => Self::search_internal(internal, key),
        }
    }

    fn search_leaf(leaf: &LeafNode<K, V>, key: &K) -> Option<V> {
        leaf.keys
            .iter()
            .position(|k| k == key)
            .map(|i| leaf.values[i].clone())
    }

    fn search_internal(internal: &InternalNode<K, V>, key: &K) -> Option<V> {
        let index = internal
            .keys
            .iter()
            .position(|k| k >= key)
            .unwrap_or(internal.keys.len());
        Self::search_node(&internal.children[index], key)
    }

    fn search_node(node: &Node<K, V>, key: &K) -> Option<V> {
        match node {
            Node::Leaf(leaf) => Self::search_leaf(leaf, key),
            Node::Internal(internal) => Self::search_internal(internal, key),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut new_root = None;
        let result = match &mut self.root {
            Node::Leaf(leaf) => Self::insert_leaf(leaf, key, value),
            Node::Internal(internal) => Self::insert_internal(internal, key, value),
        };

        if let Some((split_key, right_node)) = result {
            new_root = Some(Node::Internal(InternalNode {
                keys: vec![split_key],
                children: vec![self.root.clone(), right_node],
            }));
        }

        if let Some(root) = new_root {
            self.root = root;
        }
    }

    fn insert_leaf(leaf: &mut LeafNode<K, V>, key: K, value: V) -> Option<(K, Node<K, V>)> {
        let pos = leaf
            .keys
            .iter()
            .position(|k| *k > key)
            .unwrap_or(leaf.keys.len());
        leaf.keys.insert(pos, key);
        leaf.values.insert(pos, value);

        if leaf.keys.len() >= ORDER {
            Some(Self::split_leaf(leaf))
        } else {
            None
        }
    }

    fn split_leaf(leaf: &mut LeafNode<K, V>) -> (K, Node<K, V>) {
        let mid = leaf.keys.len() / 2;
        let split_key = leaf.keys[mid].clone();

        let new_leaf = LeafNode {
            keys: leaf.keys.split_off(mid),
            values: leaf.values.split_off(mid),
        };

        (split_key, Node::Leaf(new_leaf))
    }

    fn insert_internal(
        internal: &mut InternalNode<K, V>,
        key: K,
        value: V,
    ) -> Option<(K, Node<K, V>)> {
        let index = internal
            .keys
            .iter()
            .position(|k| *k > key)
            .unwrap_or(internal.keys.len());
        let result = match &mut internal.children[index] {
            Node::Leaf(leaf) => Self::insert_leaf(leaf, key, value),
            Node::Internal(child) => Self::insert_internal(child, key, value),
        };

        if let Some((split_key, right_node)) = result {
            internal.keys.insert(index, split_key);
            internal.children.insert(index + 1, right_node);
        }

        if internal.keys.len() >= ORDER {
            Some(Self::split_internal(internal))
        } else {
            None
        }
    }

    fn split_internal(internal: &mut InternalNode<K, V>) -> (K, Node<K, V>) {
        let mid = internal.keys.len() / 2;
        let split_key = internal.keys[mid].clone();

        let new_internal = InternalNode {
            keys: internal.keys.split_off(mid + 1),
            children: internal.children.split_off(mid + 1),
        };

        (split_key, Node::Internal(new_internal))
    }

    pub fn traverse(&self) {
        match &self.root {
            Node::Leaf(leaf) => Self::print_leaf(leaf),
            Node::Internal(internal) => Self::print_internal(internal),
        }
    }

    fn print_leaf(_leaf: &LeafNode<K, V>) {
        println!("Leaf:");
    }

    fn print_internal(internal: &InternalNode<K, V>) {
        for child in &internal.children {
            match child {
                Node::Leaf(leaf) => Self::print_leaf(leaf),
                Node::Internal(child_internal) => Self::print_internal(child_internal),
            }
        }
    }
}
