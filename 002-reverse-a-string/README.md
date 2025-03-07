# Reverse a string

## Problem Statement

Write a Rust program that takes a string input from the user and reverses it **without using any built-in reverse methods** or iterators like `chars().rev()`. Your solution should use manual character manipulation to achieve the result.

## Requirements

- The program should accept a single string input.
- It should reverse the string using only basic loops and manual indexing.
- Built-in reverse functions (`rev()`, `collect()`, etc.) **must not** be used.

## Input

```bash
Enter a string: hello
```

## Output

```bash
Reversed string: olleh
```

## Constraints

- The string can contain **any printable ASCII characters**.
- The maximum length of the string is **1000 characters**.
- The input string may include **whitespace**.
- The program should handle **empty strings** gracefully.
