# Largest Number Finder in an Array

## Problem Statement

Write a Rust function that takes an array of integers as input and returns the **largest number** in the array. The function should work for arrays of any size, including empty arrays.

## Requirements

- Implement a function that accepts a **slice of integers** (`&[i32]`).
- The function should **return the largest number** in the array.
- If the array is **empty**, the function should return an `Option<i32>` to safely handle the absence of a maximum value.

## Input

Fixed input array:

```bash
let numbers = [3, 5, 7, 2, 8, -1, 4, 10];
```

## Output

The function should return:

```bash
The largest number is: 10
```

If the array is empty:

```bash
The array is empty.
```

## Constraints

- The array may contain **positive**, **negative**, or **zero** values.
- The function should return `None` for **empty arrays**.
- The time complexity should be **O(n)** — a single pass through the array.
