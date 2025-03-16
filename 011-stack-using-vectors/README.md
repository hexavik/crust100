# Implement a Stack Using Vectors

## Problem Statement

Implement a **stack** data structure using **vectors**. The stack should support the following operations:

- **Push:** Add an element to the top of the stack.
- **Pop:** Remove and return the top element of the stack.
- **Peek:** View the top element without removing it.
- **is_empty:** Check if the stack is empty.

## Requirements

- Use a struct `Stack<T>` to represent the stack.
- Implement the following methods:
  - `push(value: T):` Adds a value to the stack.
  - `pop() -> Option<T>:` Removes and returns the top value (if available).
  - `peek() -> Option<&T>:` Returns a reference to the top value without removing it.
  - `is_empty() -> bool:` Returns `true` if the stack is empty.
- The stack should be **generic**, meaning it should work for any data type.

## Input

Operations to **push, pop, or peek** values.

```bash
push 10
push 20
peek
pop
is_empty
```

## Output

Results of stack operations.

```bash
Pushed: 10
Pushed: 20
Top element: 20
Popped: 20
Stack is not empty
```

## Constraints

- The **pop and peek** operations should handle cases where the stack is empty gracefully.
- The program should support **multiple data types** using generics.

> [!NOTE]
>
> - Use `Vec<T>` intenally to store stack elements.
> - Consider using `Option<T>` to hanlde empty stack cases.
> - Implmenting **unit tests** can help ensure correctness.
