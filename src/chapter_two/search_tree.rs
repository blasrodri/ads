use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct BinaryTree<K, V> {
    key: K,
    element: V,
    left: Option<Rc<RefCell<BinaryTree<K, V>>>>,
    right: Option<Rc<RefCell<BinaryTree<K, V>>>>,
}

impl<K, V> BinaryTree<K, V> {
    #[allow(dead_code)]
    fn find(&self, key: K) -> Option<V>
    where
        K: Eq + PartialOrd + std::fmt::Debug,
        V: Clone + std::fmt::Debug,
    {
        if self.key == key {
            return Some(self.element.clone());
        } else if self.key > key && self.left.is_some() {
            let left = self.left.as_ref().unwrap().borrow();
            return left.find(key);
        } else if self.key < key && self.right.is_some() {
            let right = self.right.as_ref().unwrap().borrow();
            return right.find(key);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_unavailable_key() {
        let tree = BinaryTree {
            key: 10,
            element: "asd",
            left: Some(Rc::new(RefCell::new(BinaryTree {
                key: 4,
                element: "askldjaslkdj",
                left: None,
                right: None,
            }))),
            right: None,
        };
        assert_eq!(tree.find(9), None);
    }

    #[test]
    fn test_search_uvailable_keys() {
        let tree = BinaryTree {
            key: 10,
            element: "asd",
            left: Some(Rc::new(RefCell::new(BinaryTree {
                key: 4,
                element: "askl",
                left: None,
                right: None,
            }))),
            right: None,
        };
        assert_eq!(tree.find(10), Some("asd"));
        assert_eq!(tree.find(4), Some("askl"));
    }
}
