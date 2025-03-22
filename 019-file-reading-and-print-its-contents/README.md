# 019 Read a File and Prints Its Contents to the Console

## Problem Statement

Write a program that takes a file path as input, reads the contents of the file, and prints them to the console.

## Requirements

- The program should **accept a file path** as input.
- It should **open the file**, read its contents, and **display them** on the console.
- Handle **errors gracefully**, such as when the file does not exist.

## Input

Provide a **file path** as input:

```bash
$ cargo run sample.txt
```

if `sample.txt` contains:

```bash
Hello, Rust!
This is a sample text file.
```

## Output

The program should print:

```bash
Hello, Rust!
This is a sample text file.
```

## Constraints

- The program should handle **large files efficiently**.
- It should return an appropriate error message if:
  - The file does not exist.
  - The file cannot be read.
- Avoid unnecessary memory usage for large files.

> [!NOTE]
>
> - Can be implemented using **Rust’s `std::fs::File` and `std::io::Read`**.
> - Consider using **BufReader** for optimized performance when reading large files.
> - Implement error handling with **Rust’s `Result` type** for better robustness.
> - Bonus: Support **reading binary files** instead of just text files.
