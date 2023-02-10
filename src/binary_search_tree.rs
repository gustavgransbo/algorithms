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
}
