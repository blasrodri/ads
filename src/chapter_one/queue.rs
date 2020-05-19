/// After trying to implement a queue using a doubly linked
/// list, I surrendered. That's why I moved towards a vector
/// implementation.
#[derive(Debug)]
struct Queue<T> {
    buff: Vec<T>,
    size: usize,
    current_element: usize,
}

impl<T> Queue<T>
where
    T: Clone + std::fmt::Debug,
{
    #[allow(dead_code)]
    fn new() -> Self {
        Queue {
            buff: Vec::with_capacity(100),
            size: 0,
            current_element: 0,
        }
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[allow(dead_code)]
    fn enqueue(&mut self, element: T) {
        if self.buff.capacity() == self.size {
            self.buff.reserve(self.size * 2);
        }
        self.buff.insert(self.size, element);
        self.size += 1;
    }

    #[allow(dead_code)]
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let result = self.buff.get(self.current_element).cloned();
        self.current_element += 1;
        if self.current_element > 1000 {
            let size = self.size - self.current_element;
            let mut v: Vec<T> = Vec::with_capacity(self.size - self.current_element + 1000);
            v.extend_from_slice(&self.buff[self.current_element..self.size]);
            self.buff = v;
            self.current_element = 0;
            self.size = size;
        }
        result
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enqueue_element() {
        let mut q: Queue<i32> = Queue::new();
        q.enqueue(2);
        assert_eq!(q.is_empty(), false);
    }

    #[test]
    fn test_enqueue_and_dequeue_element() {
        let mut q: Queue<i32> = Queue::new();
        q.enqueue(2);
        q.enqueue(20);
        q.enqueue(2000);
        assert_eq!(q.is_empty(), false);
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(20));
        assert_eq!(q.dequeue(), Some(2000));
        for _ in 0..1_000 {
            q.enqueue(4000);
        }
        for _ in 0..1_000 {
            assert_eq!(q.dequeue(), Some(4000));
        }
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn test_dequeue_empty_queue() {
        let mut q: Queue<i32> = Queue::new();
        assert_eq!(q.dequeue(), None);
    }
}
