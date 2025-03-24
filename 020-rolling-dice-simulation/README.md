# 020 Simulate Rolling a Dice and Keep Track of the Results Using a HashMap

## Problem Statement

Write a Rust program that simulates rolling a six-sided dice multiple times and keeps track of the results using a **HashMap**.

## Requirements

- The program should **simulate rolling a six-sided dice** multiple times.
- Store the count of each outcome (1 to 6) in a **HashMap**.
- Display the final count of each face after all rolls.
- Allow the user to specify the **number of rolls** as input.

## Input

The user provides the number of dice rolls:

```bash
$ cargo run 10
```

## Output

Exmapple output for 10 rolls:

```bash
Results after 10 rolls:
1: 2 times
2: 1 time
3: 3 times
4: 0 times
5: 2 times
6: 2 times
```

## Constraints

- The program should use **random number generation** to simulate dice rolls.
- Ensure the counts are **stored efficiently** in a **HashMap**.
- The user **must provide a valid number** for rolls; otherwise, handle errors gracefully.

> [!NOTE]
>
> - Use **Rust’s `rand` crate** to generate random numbers.
> - Implement **error handling** for invalid inputs (e.g., non-numeric values).
> - Consider displaying **percentages** of each outcome in addition to counts.
