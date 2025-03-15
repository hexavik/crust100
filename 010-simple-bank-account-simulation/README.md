# Simulate a Simple Bank Account with Deposit, Withdraw, and Balance Method

## Problem Statement

Implement a simulation of a **bank account** with basic functionalities:

- **Deposit:** Add money to the account.
- **Withdraw:** Remove money from the account if sufficient balance is available.
- **Balance Check:** View the current balance.

The program should ensure safe **transactions** while handling invalid operations appropriately.

## Requirements

- Implement a struct `BankAccount` with:
  - A private field to store the **balance**.
  - Methods for `deposit()`, `withdraw()`, and `get_balance()`.
- The withdrawal should **fail** if the balance is insufficient.
- Implement a **simple CLI interface** to interact with the account.

## Input

Commands to **deposit, withdraw, or check balance**.

```bash
deposit 500
withdraw 200
balance
```

## Output

A message confirming the operation:

```bash
Deposited: $500
Withdrawn: $200
Current Balance: $300
```

## Constraints

- The balance **cannot** be negative.
- Deposits and withdrawals should only accept **positive values**.
- The program should handle **invalid inputs gracefully**.

> [!NOTE]
>
> - Use **structs and methods** to encapsulate bank account logic.
> - Consider using **Result<T, E>** for error handling.
> - Implementing **unit tests** can help ensure correctness.
