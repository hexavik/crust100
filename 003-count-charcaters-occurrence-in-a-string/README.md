# Implement a Function to Count Occurrences of Each Character in a String

## Problem Statement

Write a Rust function that takes a string as input and returns the count of each character's occurrences. The function should produce a mapping of characters to their frequencies.

## Requirements

- Implement a function that accepts a string slice (`&str`).
- Use a suitable data structure (like `HashMap`) to store character counts.
- Ignore spaces and count only alphanumeric characters.
- Display each character along with its count in the output.

## Input

```bash
Enter a string: "hello world"
```

## Output

```bash
Character counts:
h: 1
e: 1
l: 3
o: 2
w: 1
r: 1
d: 1
```

## Constraints

- The input string contains only printable ASCII characters.
- **Case-sensitive** — 'A' and 'a' are counted separately.
- Ignore **whitespace** characters (space, tab, etc.).
