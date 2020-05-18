use std::ops::DerefMut;

#[allow(dead_code)]
struct Stack<T> {
    head: Option<Node<T>>,
}

#[derive(Clone)]
struct Node<T> {
    element: Option<T>,
    next: Option<Box<Node<T>>>,
}

impl<T> Stack<T>
where
    T: Clone,
{
    #[allow(dead_code)]
    fn new(head: Option<Node<T>>) -> Self {
        Stack { head }
    }

    #[allow(dead_code)]
    fn pop(&mut self) -> Option<T> {
        match &mut self.head {
            None => None,
            Some(node) => {
                let element = node.element.as_ref().cloned();
                self.head = match &mut node.next {
                    None => None,
                    Some(next) => Some(next.deref_mut().clone()),
                };
                element
            }
        }
    }
    #[allow(dead_code)]
    fn push(&mut self, element: T) {
        let mut node = Node {
            element: Some(element),
            next: None,
        };
        if self.head.is_none() {
            self.head = Some(node);
        } else {
            let temp = self.head.as_ref().cloned();
            node.next = temp.map(Box::new);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pop_empty_stack() {
        let mut s: Stack<i32> = Stack::new(None);
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_pop_non_empty_stack() {
        let node = Node {
            element: Some(3),
            next: None,
        };
        let node2 = Node {
            element: Some(2),
            next: Some(Box::new(node)),
        };
        let mut s: Stack<i32> = Stack::new(Some(node2));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn test_push_pop() {
        let mut s: Stack<i32> = Stack::new(None);
        s.push(12);
        assert_eq!(s.pop(), Some(12));
        assert_eq!(s.pop(), None);
    }
}
