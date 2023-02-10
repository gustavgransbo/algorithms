struct Node<T: PartialOrd> {
    item: T,
    left: BinarySearchTree<T>,
    right: BinarySearchTree<T>,
}

impl<T: PartialOrd> Node<T> {
    fn new(item: T) -> Self {
        Node {
            item,
            left: BinarySearchTree::new(),
            right: BinarySearchTree::new(),
        }
    }
}

pub struct BinarySearchTree<T: PartialOrd> {
    node: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self { node: None }
    }

    pub fn insert(&mut self, item: T) {
        if let Some(node) = &mut self.node {
            if item > node.item {
                node.right.insert(item);
            } else if item < node.item {
                node.left.insert(item);
            }
        } else {
            self.node = Some(Box::new(Node::new(item)));
        }
    }

    fn find_and_delete_min(&mut self) -> T {
        if self.node.as_ref().unwrap().left.node.is_some() {
            self.node.as_mut().unwrap().left.find_and_delete_min()
        } else {
            let mut node = self.node.take().unwrap();
            self.node = node.right.node.take();
            node.item
        }
    }

    fn delete_node(&mut self) {
        let node = self.node.as_mut().unwrap();
        if node.left.node.is_some() && node.right.node.is_some() {
            node.item = node.right.find_and_delete_min();
        } else if node.left.node.is_some() {
            self.node = node.left.node.take();
        } else if node.right.node.is_some() {
            self.node = node.right.node.take();
        } else {
            self.node = None;
        }
    }

    pub fn delete(&mut self, item: T) {
        if let Some(node) = &mut self.node {
            if item > node.item {
                node.right.delete(item);
            } else if item < node.item {
                node.left.delete(item);
            } else {
                self.delete_node();
            }
        }
    }

    pub fn contains(&self, item: T) -> bool {
        if let Some(node) = &self.node {
            if item > node.item {
                node.right.contains(item)
            } else if item < node.item {
                node.left.contains(item)
            } else {
                true
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree_doesnt_contain_element() {
        let bst = BinarySearchTree::new();
        assert!(!bst.contains(1));
    }

    #[test]
    fn inserted_element_exists() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);
        assert!(bst.contains(1));
    }

    #[test]
    fn non_insterted_element_does_not_exist() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);
        assert!(!bst.contains(2));
    }

    #[test]
    fn two_inserted_elements_exist() {
        let mut bst = BinarySearchTree::new();
        bst.insert('a');
        bst.insert('c');

        assert!(bst.contains('a'));
        assert!(bst.contains('c'));
        assert!(!bst.contains('b'));
    }

    #[test]
    fn multiple_inserted_items_exist() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(6);
        bst.insert(3);
        bst.insert(4);
        bst.insert(2);
        bst.insert(1);

        for i in 1..7 {
            assert!(bst.contains(i));
        }
    }

    #[test]
    fn deleted_item_does_not_exist() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);
        bst.delete(1);

        assert!(!bst.contains(1));
    }

    #[test]
    fn delete_item_that_does_not_exist() {
        let mut bst = BinarySearchTree::new();
        bst.delete(1);
    }

    #[test]
    fn other_items_exist_after_root_with_only_right_child_deleted() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);
        bst.insert(3);
        bst.insert(2);
        bst.delete(1);

        assert!(bst.contains(2));
        assert!(bst.contains(3));

        assert!(!bst.contains(1));
    }

    #[test]
    fn other_items_exist_after_root_with_only_left_child_deleted() {
        let mut bst = BinarySearchTree::new();
        bst.insert(3);
        bst.insert(1);
        bst.insert(2);
        bst.delete(3);

        assert!(bst.contains(1));
        assert!(bst.contains(2));

        assert!(!bst.contains(3));
    }

    #[test]
    fn other_items_exist_after_root_with_two_children_deleted() {
        let mut bst = BinarySearchTree::new();
        bst.insert(2);
        bst.insert(1);
        bst.insert(5);
        bst.insert(3);
        // The roots succeeding node gets a right child
        bst.insert(4);

        bst.delete(2);

        assert!(bst.contains(1));
        assert!(bst.contains(3));
        assert!(bst.contains(4));
        assert!(bst.contains(5));
        assert!(!bst.contains(2));
    }
}
