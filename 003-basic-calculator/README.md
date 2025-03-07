# Basic Calculator supporting +, -, *, /

## Problem Statement

Write a Rust program that functions as a basic calculator. The calculator should support addition, subtraction, multiplication, and division. The user will input a simple expression (e.g., `5 + 3`), and the program should compute and display the result.

## Requirements

- The program should accept a mathematical expression consisting of two operands and an operator (`+`, `-`, `*`, `/`).
- It should correctly parse the input and perform the requested operation.
- Division by zero must be handled gracefully.

## Input

```bash
Enter an expression (e.g., 5 + 3): 12 / 4
```

## Output

```bash
Result: 3
```

## Constraints

- The operands are integers or floating-point numbers.
- The operator can be one of: `+`, `-`, `*`, `/`.
- Division by zero should print a suitable error message.
- The program assumes valid input for now (bonus challenges cover error handling).
