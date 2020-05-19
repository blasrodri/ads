use core::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
struct BinaryTree<K, V> {
    root: Option<Node<K, V>>,
}

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Rc<RefCell<Node<K, V>>>>,
    right: Option<Rc<RefCell<Node<K, V>>>>,
}

impl<K, V> BinaryTree<K, V> {
    #[allow(dead_code)]
    fn insert(&mut self, node: Node<K, V>)
    where
        K: Eq + Clone + PartialOrd,
        V: Eq + Clone,
    {
        if self.root.is_none() {
            self.root = Some(node);
            return;
        }
        if node.key == self.root.as_ref().unwrap().key {
            // modify the element only
            self.root.as_mut().unwrap().value = node.value;
            return;
        } else if self.root.as_ref().unwrap().key < node.key {
            if self.root.as_ref().unwrap().right.is_none() {
                self.root.as_mut().unwrap().right = Some(Rc::new(RefCell::new(node)));
            } else {
                self.root
                    .as_mut()
                    .unwrap()
                    .right
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .insert(node);
            }
            return;
        }
        if self.root.as_ref().unwrap().left.is_none() {
            self.root.as_mut().unwrap().left = Some(Rc::new(RefCell::new(node)));
        } else {
            self.root
                .as_mut()
                .unwrap()
                .left
                .as_mut()
                .unwrap()
                .borrow_mut()
                .insert(node);
        }
    }
}

impl<K, V> Node<K, V> {
    #[allow(dead_code)]
    fn insert(&mut self, node: Node<K, V>)
    where
        K: PartialOrd + Eq + Clone,
        V: Eq + Clone,
    {
        if self.key == node.key {
            self.value = node.value;
            return;
        } else if self.key < node.key {
            if self.right.is_none() {
                self.right = Some(Rc::new(RefCell::new(node)));
            } else {
                self.right.as_mut().unwrap().borrow_mut().insert(node);
            }
            return;
        }

        if self.left.is_none() {
            self.left = Some(Rc::new(RefCell::new(node)));
        } else {
            self.left.as_mut().unwrap().borrow_mut().insert(node);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insert() {
        let root_node = Node {
            key: 12,
            value: "twelve",
            left: None,
            right: None,
        };

        let new_node = Node {
            key: 11,
            value: "eleven",
            left: None,
            right: None,
        };

        let new_node2 = Node {
            key: 14,
            value: "fourteen",
            left: None,
            right: None,
        };

        let new_node3 = Node {
            key: 13,
            value: "thirteen",
            left: None,
            right: None,
        };

        let new_node4 = Node {
            key: 15,
            value: "fifteen",
            left: None,
            right: None,
        };
        let mut bt = BinaryTree {
            root: Some(root_node),
        };

        bt.insert(new_node);
        bt.insert(new_node2);
        bt.insert(new_node4);
        bt.insert(new_node3);
        assert_eq!(bt.root.is_some(), true);
        assert_eq!(bt.root.as_ref().unwrap().value, "twelve");
        assert_eq!(
            (bt.root.clone().unwrap().left.unwrap().borrow()).value,
            "eleven"
        );
        let fourteen = bt
            .root
            .clone()
            .unwrap()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .clone();
        assert_eq!(fourteen.value, "fourteen");
        assert_eq!(fourteen.left.as_ref().unwrap().borrow().value, "thirteen");
        assert_eq!(fourteen.right.as_ref().unwrap().borrow().value, "fifteen");
    }
}
