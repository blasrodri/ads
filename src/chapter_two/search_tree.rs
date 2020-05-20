use std::boxed::Box;

#[derive(Clone, Debug)]
struct BinarySearchTree<K, V> {
    root: Option<Edge<K, V>>,
}

#[derive(Clone, Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Edge<K, V>,
    right: Edge<K, V>,
}

#[derive(Clone, Debug)]
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
    K: Clone + PartialOrd + Eq,
    V: Clone + PartialOrd + Eq,
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
}

impl<K, V> Edge<K, V>
where
    K: Clone + PartialOrd + Eq,
    V: Clone + PartialOrd + Eq,
{
    fn insert(&mut self, key_to_insert: K, value_to_insert: V) {
        match self {
            Edge { node } => match node {
                None => *node = Some(Box::new(Node::from((key_to_insert, value_to_insert)))),
                Some(boxed_node) => match &mut **boxed_node {
                    Node {
                        key, left, right, ..
                    } => {
                        if key_to_insert == *key {
                            *boxed_node = Box::new(Node {
                                key: key.clone(),
                                value: value_to_insert,
                                left: left.clone(),
                                right: right.clone(),
                            });
                        } else if key_to_insert > *key {
                            right.insert(key_to_insert, value_to_insert);
                        } else {
                            left.insert(key_to_insert, value_to_insert);
                        }
                    }
                },
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_nodes() {
        let mut bt = BinarySearchTree {
            root: Some(Edge::from((1, "asd"))),
        };
        bt.insert(2, "BLA");
        bt.insert(0, "123");
        bt.insert(15, "@@@");
        bt.insert(2, "BLAS");
        dbg!(&bt);
    }
}
