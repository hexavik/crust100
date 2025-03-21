# 016 Simple Password Generator with Options for Length and Complexity

## Problem Statement

build a program that generates a **random password** based on user-defined parameters such as **length** and **compplexity** (uppercase, lowercase, numbers, special characters).

## Requirements

- Create a function: `fn generate_password(length: usize, compplexity: u8) -> String`
- The function should allow:
  - **Length selection** (minimum 6 charcaters)
  - **Complexity levels:**
    - `1`:Lowercase letters only (`a-z`).
    - `2`:Lowercase + Uppercase (`a-z`, `A-Z`).
    - `3`:Lowercase + Uppercase + Numbers (`0-9`).
    - `4`:Lowercase + Uppercase + Numbers + Special characters (`!@#$%^&*`).
- Ensure the generated password is **random**.

## Input

- `length`: Integer specifying password length.
- `complexity`: Integer specifying the complexity level.

```bash
length = 12, complexity = 3
```

## Output

A randomly generated password based on given constraints.

```bash
Generated Password: A1b3F9XyL8mQ
```

## Constraints

- Password length **must be at least 6**.
- Complexity **must be between 1 and 4**.
- Use **cryptographically secure random number generation**.
- Avoid hardcoded passwords.

> [!NOTE]
>
> - Use **Rust's** `rand` **crate** for generating random characters.
> - Consider using **iterators and** `.choose()` for efficient character selection.
> - Can be extended to include **user-friendly passphrases**.
