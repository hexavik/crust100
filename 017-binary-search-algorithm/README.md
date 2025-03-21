# 017 Implement a Binary Search Algorithm

## Problem Statement

Implement the **binary search algorithm**, which efficiently finds a target value in a **sorted array** by repeatedly dividing the search interval in half.

## Requirements

- Implement a function:

```rust
fn binary_search(arr: &[i32], target: i32) -> Option<usize>
```

- The function should:
  - Take a **sorted array** and a **target value** as input.
  - Return the **index** of the target if found, otherwise return `None`.
  - Use **divide-and-conquer** approach (Binary Search).
- The solution should have **O(log n) time complexity**.

## Input

A sorted list of integers and a target value.

```bash
arr = [2, 3, 5, 7, 11, 13, 17, 19, 23]
target = 7
```

## Output

The index of the target value if found, otherwise `None`.

```bash
Output Index: 3
```

## Constraints

- The array **must be sorted** before applying binary search.
- The function should handle **empty arrays**.
- Avoid unnecessary recursive calls to **prevent stack overflow**.

> [!NOTE]
>
> - Can be implemented using **recursion** or **iteration**.
> - Consider using **Rust's slice indexing** for better performance.
> - Extending it to **search for floating-point numbers** can be an additional challenge.
