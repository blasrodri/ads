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

    #[allow(dead_code)]
    fn delete(&mut self, node: Node<K, V>)
    where
        K: Eq + Clone + PartialOrd,
        V: Eq + Clone,
    {
        if self.root.is_none() {
            return
        }

        if node.key == self.root.as_ref().unwrap().key {
            if self.root.as_ref().unwrap().left.is_none() && self.root.as_ref().unwrap().right.is_none() {
                self.root = None;
            } else if self.root.as_ref().unwrap().right.is_some() {
                // promote this node to root
                // let recursion handle the rest
                let right = self
                    .root
                    .as_ref()
                    .unwrap()
                    .right
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .clone();
                self.root.as_mut().unwrap().key =  right.key.clone();
                self.root.as_mut().unwrap().value =  right.value.clone();
                self.root.as_mut().unwrap().delete(right);
            }
            return;
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

    #[allow(dead_code)]
    fn delete(&mut self, node: Node<K, V>)
    where
        K: Eq + Clone + PartialOrd,
        V: Eq + Clone,
    {
        // self is the parent of the node we want to delete.
        // The invariant we rely on is that, in this function
        // we always have a parent. The is true, because the only case
        // where this does not hold is when the node to delete is the root.
        // And that is addressed on the `delete` function within the BinaryTree
        if self.right.is_none() && self.left.is_none() {
            // the node does not exist. Thus, nothing to delete.
            return;
        }
        // The node *may* exist
        if self.key < node.key && self.right.is_some() {
            let right_key = self.right.as_ref().unwrap().borrow().key.clone();
            if node.key != right_key {
                return self.right.as_ref().unwrap().borrow_mut().delete(node);
            } else {
                //the child is the one to delete
                let right_value = self.right.as_ref().unwrap().borrow().key.clone();
                self.right.as_ref().unwrap().borrow_mut().key =
            }
        }
        if self.key > node.key && self.left.is_some() {
            let left_key = self.left.as_ref().unwrap().borrow().key.clone();
            if node.key != left_key {
                return self.left.as_ref().unwrap().borrow_mut().delete(node);
            }
        }

        if self.key == node.key {
            if self.right.is_some() {
                self.key = self.right.as_ref().unwrap().borrow().key.clone();
                self.value = self.right.as_ref().unwrap().borrow().value.clone();
            }
        }
    }
        // The node does not exist...
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

    #[test]
    fn test_delete() {
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

        let mut bt = BinaryTree {
            root: Some(root_node.clone()),
        };

        bt.insert(new_node.clone());
        bt.insert(new_node2.clone());
        assert_eq!(bt.root.is_some(), true);
        assert_eq!(bt.root.as_ref().unwrap().value, "twelve");
        bt.delete(root_node.clone());
        //bt.delete(new_node.clone());
        dbg!(&bt);
    }
}
