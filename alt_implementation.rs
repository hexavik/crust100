struct LinkedList {
    data: Option<i32>,
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            data: None,
            next: None,
        }
    }

    fn enqueue(&mut self, data: i32) {
        match self.data {
            None => {
                self.data = Some(data);
                self.next = None;
            }
            Some(_) => {
                let mut current = self;
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(Box::new(LinkedList {
                    data: Some(data),
                    next: None,
                }))
            }
        }
    }

    fn dequeue(&mut self) {
        let removed_data = self.data.take();
        if let Some(mut next_node) = self.next.take() {
            self.data = next_node.data.take();
            self.next = next_node.next.take();
            println!("Removed Node: {:?}", removed_data.unwrap());
        } else {
            println!("Error: Empty list.");
        }
    }

    fn display_list(&self) {
        let mut current = self;
        while let Some(data) = current.data {
            print!("{} -> ", data);
            if let Some(ref next_node) = current.next {
                current = next_node;
            } else {
                break;
            }
        }
        println!("None");
    }
}

fn main() {
    let mut queue = LinkedList::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    queue.dequeue();
    queue.display_list();
    queue.enqueue(40);
    queue.enqueue(50);
    queue.display_list();
    queue.dequeue();
    queue.display_list();
}
