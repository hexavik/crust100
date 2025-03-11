# Build a CLI app to covert temperatures between Celsius and Farhenheit

## Problem Statement

Create a simple **command-line interface (CLI) application** in Rust that converts temperatures between **Celsius** and **Fahrenheit**.

The app should:

- Convert **Celsius to Fahrenheit**.
- Convert **Fahrenheit to Celsius**.
- Accept inputs and display outputs in the terminal.

## Requirements

- Parse command-line arguments for:
  - The temperature value (e.g., `32`, `100`).
  - The unit (either `C` for Celsius or `F` for Fahrenheit).
- Implement two conversion functions:
  - `to_celsius(fahrenheit: f64) -> f64`
  - `to_fahrenheit(celsius: f64) -> f64`
- Ensure clear output, like:

```bash
100°C is 212°F
```

- Handle invalid inputs gracefully.

## Input

Command-line arguments in the format:

```bash
cargo run -- 100 C
```

or

```bash
cargo run -- 212 F
```

## Output

The converted temperature value and unit, for example:

```bash
100°C is 212°F
212°F is 100°C
```

## Constraints

- The input temperature should be a valid floating-point number.
- Only allow valid units (`C` or `F`).
- Ensure conversions follow the formulas.
  - Celsius to Fahrenheit: `(C × 9/5) + 32`
  - Fahrenheit to Celsius: `(F − 32) × 5/9`

> [!NOTE]
>
> - Use `std::env::args()` to capture command-line inputs.
> - Consider using **pattern matching** to validate the temperature unit.
> - You could add **error handling** to catch cases where users forget inputs or use invalid formats.
