# 012 Implement a Queue using Linked Lists

## Problem Statement

Implement a **queue** data structure using a **linked list**. The queue should follow the **FIFO (First In, First Out)** principle and support the following operations:

- **Enqueue:** Add an element to the back of the queue.
- **Dequeue:** Remove and return the front element of the queue.
- **Peek:** View the front element without removing it.
- **is_empty:** Check if the queue is empty.

## Requirements

- Use a **struct** `Queue<T>` to represent the queue.
- Implement the following methods:
  - `enqueue(value: T):` Adds a value to the back of the queue.
  - `dequeue() -> Option<T>:` Removes and returns the front value (if available).
  - `peek() -> Option<&T>:` Returns a reference to the front value without removing it.
  - `is_empty() -> bool:` Returns `true` if the queue is empty.
- The queue should be generic, meaning it should work for any data type.
- Use a linked list (not a `Vec<T>`) to implement the queue.

## Input

Operations to enqueue, dequeue, o rpeek values.

```bash
enqueue 5
enqueue 15
peek
dequeue
is_empty
```

## Output

Results of queue operations.

```bash
Enqueued: 5
Enqueued: 15
Front element: 5
Dequeued: 5
Queue is not empty
```

## Constraints

- The **dequeue and peek** operations should handle cases where the queue is empty gracefully.
- The queue should **not use built-in collections like Vec** for storage.
- Consider implementing a **linked list** with **nodes** containing `value` and `next` pointers.

> [!NOTE]
>
> - Use `Option<Box<Node<T>>` for handling linked list nodes.
> - A **tail pointer** can optimize the `enqueue` operation for **O(1)** efficiency.
> - Implementing **unit tests** can help ensure correctness.
