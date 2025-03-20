# 015 Implement a Basic `Result` Type Error Handling Scenario for Division by Zero

## Problem Statement

Implement a function that performs division between two numbers whike handling **division by zero** using the `Result` type. Instead of panicking, the function should return an appropriate error message.

## Requirements

- Implement a function: `fn safe_divide(a: f64, b: f64) -> Result<f64, String>`
- If `b` is **not zero**, return the result of `a / b`.
- If b is **zero**, return an error using `Err("Cannot divide by zero")`.

## Input

Two floating-point numbers, `a` and `b`.

```bash
a = 10.0, b = 2.0
a = 5.0, b = 0.0
```

## Output

If division is successful:

```bash
5.0
```

If division by zero occurs:

```bash
Error: Cannot divide by zero
```

## Constraints

- The function should **not panic** under any circumstance.
- Must return a `Result<f64, String>` instead of using `panic!()`.
- Should be **generic enough** to hanlde diffeent floating-point inputs.

> [!NOTE]
>
> - Use Rust's `Result` type to handle errors gracefully.
> - Consider using **match expressions** to handle both `Ok` and `Err` cases.
> - Can be extented to handle **integer division** with `Option` instead.
