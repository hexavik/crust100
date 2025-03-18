struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
struct Queue<T> {
    front: Option<Box<Node<T>>>,
    rear: Option<*mut Node<T>>, // Using `Option` to track rear safely
}

impl<T: std::fmt::Display> Queue<T> {
    fn new() -> Self {
        Queue {
            front: None,
            rear: None,
        }
    }

    fn enqueue(&mut self, data: T) {
        let mut new_node = Box::new(Node { data, next: None });
        let raw_node: *mut Node<T> = &mut *new_node; // Get a mutable raw pointer safely

        if let Some(rear_ptr) = self.rear {
            unsafe {
                (*rear_ptr).next = Some(new_node); // Update rear's next safely
            }
        } else {
            self.front = Some(new_node); // If empty, front becomes the new node
        }

        self.rear = Some(raw_node); // Move rear to the new node
    }

    fn dequeue(&mut self) -> Option<T> {
        if let Some(mut old_front) = self.front.take() {
            self.front = old_front.next.take(); // Move front to the next node
            if self.front.is_none() {
                self.rear = None; // If empty, reset rear
            }
            Some(old_front.data)
        } else {
            None
        }
    }

    fn display(&self) {
        let mut current = self.front.as_ref();
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = node.next.as_ref();
        }
        println!("None");
    }
}

fn main() {
    let mut q: Queue<i32> = Queue::new();

    q.enqueue(10);
    q.enqueue(20);
    q.enqueue(30);
    q.display();

    q.dequeue();
    q.display();

    q.enqueue(40);
    q.display();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut q: Queue<i32> = Queue::new();

        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);

        assert_eq!(q.dequeue(), Some(10));
        assert_eq!(q.dequeue(), Some(20));
        assert_eq!(q.dequeue(), Some(30));
        assert_eq!(q.dequeue(), None);

        q.enqueue(40);
        assert_eq!(q.dequeue(), Some(40));
        assert_eq!(q.dequeue(), None);
    }
}
