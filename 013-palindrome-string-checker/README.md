# 013 Write a Function to Check if a String is a Palindrome

## Problem Statement

Write a function that determines whether a given string is a **palindrome**. A **palindrome** is a word, phrase, number, or sequence that reads the same **forward and backward** (ignoring spaces, punctuation, and case sensitivity).

## Requirements

- Implement a function: `fn is_palindrome(s: &str) -> bool`
- The function should return `true` if the input string is a palindrome, otherwise `false`.
- Ignore **case sensitivity** and **non-alphanumeric characters**.
- The function should handle **empty strings** correctly.

## Input

A single string:

```bash
"racecar"
"A man, a plan, a canal, Panama!"
"hello"
```

## Output

A boolean value (`true` or `false`) indicating whether the string is a palindrome.

```bash
true
true
false
```

## Constraints

- The function should efficiently check for palindromes without unnecessary computations.
- The input string can contain **letters**, **numbers**, **spaces**, and **punctuation**.
- Consider performance optimizations for large strings.

> [!NOTE]
>
> - Convert the string to **lowercase** to handle case insensitivity.
> - Use **two-pointer technique** to compare characters from the beginning and end.
> - **Regex or iterators** can help filter out non-alphanumeric characters before comparison.
