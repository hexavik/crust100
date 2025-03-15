# An `enum` for days of the week and print a custom message for each

## Problem Statement

Create a program that defines an **enum** for the **days of the week** and prints a **custom message** for each day.

The program should:

- Use an `enum` to represent the **seven days** of the week.
- Implement a function that prints **a different message** for each day.
- Allow users to provide a day (e.g., **Monday**) as input and get the corresponding message.

## Requirements

- Define an `enum DaysOfWeek` with **seven variants** (`Monday`, `Tuesday`, etc.).
- Implement a function (This function should match each day and print a unique message):

```rust
fn print_message(day: DaysOfWeek)
```

- Allow input from users via **command-line arguments**.

## Input

A string representing the day of the week.

```bash
cargo run -- Monday
```

## Output

A **custom message** corresponding to the provided day.

```bash
Monday: Start of a new week! Stay productive.
Saturday: Time to relax and have fun!
```

## Constraints

- The input should be **case-insensitive** (`monday` and `Monday` should be treated the same).
- Handle invalid inputs gracefully by displaying a helpful error message.
- The enum must include **all seven days**.

> [!NOTE]
>
> - Use **match expressions** to associate messages with days.
> - Consider using **to_lowercase()** to handle case-insensitive inputs.
> - An optional **default message** can be printed for invalid inputs.
