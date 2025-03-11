# Implement a Basic Linked List with Push and Display Methods

## Problem Statement

Implement a singly linked list in Rust with the following methods:

- `push(value: i32)` — adds a new node containing `value` to the end of the list.
- `display()` — prints all the elements in the linked list.

The goal is to practice **structs**, **ownership**, and **Box pointers** in Rust.

## Requirements

- Create a `Node` struct containing:
  - `value: i32`
  - `next: Option<Box<Node>>`
- Create a `LinkedList` struct with:
  - `head: Option<Box<Node>>`
- Implement methods:
  - `push(&mut self, value: i32)`
  - `display(&self)`

## Input

A series of integers to be added to the linked list.

```rust
let mut list = LinkedList::new();
list.push(10);
list.push(20);
list.push(30);
list.display();
```

## Output

The values of the linked list nodes, printed in order.

```bash
10 -> 20 -> 30
```

## Constraints

- The list should support dynamic resizing (no fixed array sizes).
- Implement **singly linked lists** only — no doubly linked or circular linked lists for now.
- **No use of Vec or other collections** — build it from scratch using structs and pointers.

> [!NOTE]
>
> - Use `Option<Box<Node>>` to create nullable "pointers".
> - Consider using `while let` loops to traverse the list.
> - Ensure the `push()` method correctly updates the tail node’s `next` pointer.
> - Think about **ownership rules** when adding new nodes — avoid invalid references.
