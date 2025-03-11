struct LinkedList {
    val: Option<i32>,
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { val: None, next: None }
    }

    fn push(&mut self, data: i32) {
        match self.val {
            None => {
                self.val = Some(data);
                self.next = None;
            }
            Some(_) => {
                let mut current = self;
                while let Some(ref mut next_node) = current.next {
                    current = next_node;
                }
                current.next = Some(Box::new(LinkedList { val: Some(data), next: None }));
            }
        }
    }

    fn display(&self) {
        let mut current = self;
        while let Some(val) = current.val {
            print!("{} -> ", val);
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
    let mut list = LinkedList::new();
    list.push(10);
    list.push(20);
    list.push(30);
    list.display();
}
