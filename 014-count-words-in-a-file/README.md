# 014 Build a Program That Counts Words in a File

## Problem Statement

Write a Rust program that reads a text file and counts the number of words in it. The program should handle different file sizes efficiently and return an accurate word count.

## Requirements

- Implement a function:

```rust
fn count_words_in_file(filename: &str) -> Result<usize, std::io::Error>
```

- The function should **open the file**, **read its contents**, and **count the number of words**.
- Words should be separated by **whitespace or punctuation**.
- The function should handle **large files efficiently** without consuming excessive memory.

## Input

A test file path (sample.txt).

```bash
Cargo run -- sample.txt
```

## Output

An integer representing the number of words in the file.

```bash
7
```

## Constraints

- Consider memory-efficient reading for **large files** (e.g., processing line-by-line instead of loading the whole file at once).
- Words should be **case insensitive**.
- Handle **file read errors gracefully**.

> [!NOTE]
>
> - Use **BufReader** for efficient file reading.
> - Consider **iterators** and **split_whitespace()** to tokenize words.
> - **Regex** can be used for more complex word detection.
