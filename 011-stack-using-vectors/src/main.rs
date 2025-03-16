use std::fmt;

struct Stack<T> {
    elements: Vec<T>,
    top: usize,
}

impl<T: fmt::Display> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            elements: Vec::new(),
            top: 0
        }
    }

    fn push(&mut self, element: T) {
        println!("Pushed: {}", element);
        self.elements.push(element);
        self.top += 1;
    }

    fn pop(&mut self) {
        if self.top == 0 {
            println!("Stack is empty.");
        } else {
            self.top -= 1;
            let popped = self.elements.remove(self.top);
            println!("Popped: {}", popped);
        }
    }

    fn peek(&self) {
        match self.elements.last() {
            Some(val) => println!("Top element: {}", val),
            None => println!("Stack is empty."),
        }
    }

    fn is_empty(&self) {
        match self.top == 0 {
            true => println!("Stack is empty."),
            false => println!("Stack is not empty.")
        }
    }
}

fn main() {
    let mut mystack: Stack<i32> = Stack::new();

    mystack.push(10);
    mystack.push(20);
    mystack.push(30);
    mystack.peek();
    mystack.pop();
    mystack.peek();
    mystack.is_empty();
}
