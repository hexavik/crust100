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

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            println!("Stack is empty.");
            None
        } else {
            self.top -= 1;
            let popped = self.elements.remove(self.top);
            println!("Popped: {}", popped);
            Some(popped)
        }
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }
}

fn main() {
    let mut mystack: Stack<i32> = Stack::new();

    mystack.push(10);
    mystack.push(20);
    mystack.push(30);
    println!("Top element: {:?}", mystack.peek());
    mystack.pop();
    println!("Top element: {:?}", mystack.peek());
    println!("Stack empty: {}", mystack.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(5);
        assert_eq!(stack.peek(), Some(&5));
        stack.pop();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_is_empty() {
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
    }
}
