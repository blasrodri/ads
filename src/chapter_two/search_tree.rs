use std::boxed::Box;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
struct BinarySearchTree<K, V> {
    root: Option<Edge<K, V>>,
}

#[derive(Clone, Debug, PartialEq)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Edge<K, V>,
    right: Edge<K, V>,
}

#[derive(Clone, Debug, PartialEq)]
struct Edge<K, V> {
    node: Option<Box<Node<K, V>>>,
}

impl<K, V> From<Node<K, V>> for Edge<K, V> {
    fn from(node: Node<K, V>) -> Self {
        Edge {
            node: Some(Box::new(node)),
        }
    }
}

impl<K, V> From<(K, V)> for Node<K, V> {
    fn from(key_val: (K, V)) -> Self {
        Node {
            key: key_val.0,
            value: key_val.1,
            left: Edge { node: None },
            right: Edge { node: None },
        }
    }
}

impl<K, V> From<(K, V)> for Edge<K, V> {
    fn from(key_val: (K, V)) -> Self {
        Edge {
            node: Some(Box::new(Node {
                key: key_val.0,
                value: key_val.1,
                left: Edge { node: None },
                right: Edge { node: None },
            })),
        }
    }
}

impl<K, V> BinarySearchTree<K, V>
where
    K: Clone + PartialOrd + Eq + Ord + std::fmt::Debug,
    V: Clone + PartialOrd + Eq + std::fmt::Debug,
{
    #[allow(dead_code)]
    fn new(maybe_node: Option<Node<K, V>>) -> BinarySearchTree<K, V> {
        let root = match maybe_node {
            None => None,
            Some(node) => Some(Edge::from(node)),
        };
        BinarySearchTree { root }
    }

    #[allow(dead_code)]
    fn insert(&mut self, key_to_insert: K, value_to_insert: V) {
        match self.root.as_mut() {
            // There no root
            None => self.root = Some(Edge::from((key_to_insert, value_to_insert))),
            // There is a root
            Some(edge) => match edge {
                Edge { node: None } => {
                    edge.node = Some(Box::new(Node::from((key_to_insert, value_to_insert))));
                }
                Edge { node: Some(_) } => edge.insert(key_to_insert, value_to_insert),
            },
        }
    }

    #[allow(dead_code)]
    fn delete(&mut self, key_to_delete: K) {
        match self.root.as_mut() {
            None => (),
            Some(edge) => edge.delete(key_to_delete),
        }
    }
}

impl<K, V> Edge<K, V>
where
    K: Clone + PartialOrd + Eq + Ord + std::fmt::Debug,
    V: Clone + PartialOrd + Eq + std::fmt::Debug,
{
    fn insert(&mut self, key_to_insert: K, value_to_insert: V) {
        match self {
            Edge { node } => match node {
                None => *node = Some(Box::new(Node::from((key_to_insert, value_to_insert)))),
                Some(boxed_node) => match &mut **boxed_node {
                    Node {
                        key, left, right, ..
                    } => match key_to_insert.cmp(key) {
                        Ordering::Equal => {
                            *boxed_node = Box::new(Node {
                                key: key.clone(),
                                value: value_to_insert,
                                left: left.clone(),
                                right: right.clone(),
                            });
                        }
                        Ordering::Greater => right.insert(key_to_insert, value_to_insert),
                        Ordering::Less => left.insert(key_to_insert, value_to_insert),
                    },
                },
            },
        }
    }

    fn delete(&mut self, key_to_delete: K) {
        match self {
            Edge { node } => match node {
                None => (),
                Some(boxed_node) => match &mut **boxed_node {
                    Node {
                        key,
                        left,
                        right,
                        value,
                    } => match key_to_delete.cmp(key) {
                        Ordering::Equal => match (&mut *left, &mut *right) {
                            (Edge { node: None }, Edge { node: None }) => {
                                *self = Edge { node: None }
                            }
                            (
                                Edge {
                                    node: Some(boxed_node_left),
                                },
                                Edge { node: None },
                            ) => match &mut **boxed_node_left {
                                Node {
                                    key: key_left_only,
                                    value: value_left_only,
                                    left: left_left_only,
                                    right: right_left_only,
                                } => match (&mut *left_left_only, &mut *right_left_only) {
                                    (Edge { node: None }, Edge { node: None }) => {
                                        *key = key_left_only.clone();
                                        *value = value_left_only.clone();
                                        *left = Edge { node: None };
                                    }
                                    (
                                        Edge {
                                            node: Some(boxed_node_left),
                                        },
                                        Edge { node: None },
                                    ) => {
                                        *key = boxed_node_left.key.clone();
                                        *value = boxed_node_left.value.clone();
                                        let key_to_remove = boxed_node_left.key.clone();
                                        left_left_only.delete(key_to_remove);
                                    }
                                    (
                                        Edge { node: None },
                                        Edge {
                                            node: Some(boxed_node_right),
                                        },
                                    ) => {
                                        *key = boxed_node_right.key.clone();
                                        *value = boxed_node_right.value.clone();
                                        let key_to_remove = boxed_node_right.key.clone();
                                        right_left_only.delete(key_to_remove);
                                    }
                                    (
                                        Edge {
                                            node: Some(_boxed_node_left),
                                        },
                                        Edge {
                                            node: Some(boxed_node_right),
                                        },
                                    ) => {
                                        *key = boxed_node_right.key.clone();
                                        *value = boxed_node_right.value.clone();
                                        let key_to_remove = boxed_node_right.key.clone();
                                        right_left_only.delete(key_to_remove);
                                    }
                                },
                            },
                            (
                                Edge { node: None },
                                Edge {
                                    node: Some(boxed_node_right),
                                },
                            ) => match &mut **boxed_node_right {
                                Node {
                                    key: key_right_only,
                                    value: value_right_only,
                                    left: left_right_only,
                                    right: right_right_only,
                                } => match (&mut *left_right_only, &mut *right_right_only) {
                                    (Edge { node: None }, Edge { node: None }) => {
                                        *key = key_right_only.clone();
                                        *value = value_right_only.clone();
                                        *right = Edge { node: None };
                                    }
                                    (
                                        Edge {
                                            node: Some(boxed_node_left),
                                        },
                                        Edge { node: None },
                                    ) => {
                                        *key = key_right_only.clone();
                                        *value = value_right_only.clone();
                                        let key_to_remove = boxed_node_left.key.clone();
                                        left_right_only.delete(key_to_remove);
                                    }
                                    (
                                        Edge { node: None },
                                        Edge {
                                            node: Some(boxed_node_right),
                                        },
                                    ) => {
                                        *key = key_right_only.clone();
                                        *value = value_right_only.clone();
                                        let key_to_remove = boxed_node_right.key.clone();
                                        dbg!(&right_right_only);
                                        dbg!(&key_to_remove);
                                        right_right_only.delete(key_to_remove);
                                    }
                                    (
                                        Edge {
                                            node: Some(_boxed_node_left),
                                        },
                                        Edge {
                                            node: Some(boxed_node_right),
                                        },
                                    ) => {
                                        *key = key_right_only.clone();
                                        *value = value_right_only.clone();
                                        let key_to_remove = boxed_node_right.key.clone();
                                        right_right_only.delete(key_to_remove);
                                    }
                                },
                            },
                            (
                                Edge {
                                    node: Some(boxed_node_left),
                                },
                                Edge {
                                    node: Some(boxed_node_right),
                                },
                            ) => {
                                if boxed_node_right.key == key_to_delete {
                                    match &mut **boxed_node_right {
                                        Node {
                                            key: key_left_and_right,
                                            value: value_left_and_right,
                                            left: left_left_and_right,
                                            right: right_left_and_right,
                                        } => match (
                                            &mut *left_left_and_right,
                                            &mut *right_left_and_right,
                                        ) {
                                            (Edge { node: None }, Edge { node: None }) => {
                                                *key = key_left_and_right.clone();
                                                *value = value_left_and_right.clone();
                                                *right = Edge { node: None };
                                            }
                                            (
                                                Edge {
                                                    node: Some(boxed_node_left),
                                                },
                                                Edge { node: None },
                                            ) => {
                                                *key = key_left_and_right.clone();
                                                *value = value_left_and_right.clone();
                                                let key_to_remove = boxed_node_left.key.clone();
                                                left_left_and_right.delete(key_to_remove);
                                            }
                                            (
                                                Edge { node: None },
                                                Edge {
                                                    node: Some(boxed_node_right),
                                                },
                                            ) => {
                                                *key = key_left_and_right.clone();
                                                *value = value_left_and_right.clone();
                                                let key_to_remove = boxed_node_right.key.clone();
                                                right_left_and_right.delete(key_to_remove);
                                            }
                                            (
                                                Edge {
                                                    node: Some(_boxed_node_left),
                                                },
                                                Edge {
                                                    node: Some(boxed_node_right),
                                                },
                                            ) => {
                                                *key = key_left_and_right.clone();
                                                *value = value_left_and_right.clone();
                                                let key_to_remove = boxed_node_right.key.clone();
                                                right_left_and_right.delete(key_to_remove);
                                            }
                                        },
                                    }
                                } else if boxed_node_left.key == key_to_delete {
                                    match &mut **boxed_node_left {
                                        Node {
                                            key: key_left_only,
                                            value: value_left_only,
                                            left: left_left_only,
                                            right: right_left_only,
                                        } => match (&mut *left_left_only, &mut *right_left_only) {
                                            (Edge { node: None }, Edge { node: None }) => {
                                                *key = key_left_only.clone();
                                                *value = value_left_only.clone();
                                                *left = Edge { node: None };
                                            }
                                            (
                                                Edge {
                                                    node: Some(boxed_node_left),
                                                },
                                                Edge { node: None },
                                            ) => {
                                                *key = key_left_only.clone();
                                                *value = value_left_only.clone();
                                                let key_to_remove = boxed_node_left.key.clone();
                                                left_left_only.delete(key_to_remove);
                                            }
                                            (
                                                Edge { node: None },
                                                Edge {
                                                    node: Some(boxed_node_right),
                                                },
                                            ) => {
                                                *key = key_left_only.clone();
                                                *value = value_left_only.clone();
                                                let key_to_remove = boxed_node_right.key.clone();
                                                right_left_only.delete(key_to_remove);
                                            }
                                            (
                                                Edge {
                                                    node: Some(_boxed_node_left),
                                                },
                                                Edge {
                                                    node: Some(boxed_node_right),
                                                },
                                            ) => {
                                                *key = key_left_only.clone();
                                                *value = value_left_only.clone();
                                                let key_to_remove = boxed_node_right.key.clone();
                                                right_left_only.delete(key_to_remove);
                                            }
                                        },
                                    }
                                }
                            }
                        },
                        Ordering::Greater => right.delete(key_to_delete),
                        Ordering::Less => left.delete(key_to_delete),
                    },
                },
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_insert_nodes() {
        let mut bt = BinarySearchTree {
            root: Some(Edge::from((1, "asd"))),
        };
        bt.insert(2, "BLA");
        bt.insert(0, "123");
        bt.insert(15, "@@@");
        bt.insert(2, "BLAS");
        assert_eq!(
            bt,
            BinarySearchTree {
                root: Some(Edge {
                    node: Some(Box::new(Node {
                        key: 1,
                        value: "asd",
                        left: Edge {
                            node: Some(Box::new(Node {
                                key: 0,
                                value: "123",
                                left: Edge { node: None },
                                right: Edge { node: None }
                            }))
                        },
                        right: Edge {
                            node: Some(Box::new(Node {
                                key: 2,
                                value: "BLAS",
                                left: Edge { node: None },
                                right: Edge {
                                    node: Some(Box::new(Node {
                                        key: 15,
                                        value: "@@@",
                                        left: Edge { node: None },
                                        right: Edge { node: None }
                                    }))
                                },
                            }))
                        },
                    })),
                }),
            }
        );
    }
    #[test]
    fn test_delete_nodes() {
        let mut bt = BinarySearchTree {
            root: Some(Edge::from((1, "asd"))),
        };
        bt.insert(2, "BLA");
        bt.insert(0, "123");
        bt.insert(-1, "123");
        bt.insert(15, "@@@");
        bt.insert(2, "BLAS");
        bt.delete(-1);
        bt.delete(2);
        bt.delete(15);
        bt.delete(1);
        dbg!(&bt);
    }
}
