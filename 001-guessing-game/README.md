# Implement a Simple Number Guessing Game

## Problem Statement

Create a command-line number guessing game in Rust. The program will randomly generate a number within a certain range, and the user will have to guess the number. After each guess, the program should provide feedback — whether the guess was too high, too low, or correct. The game ends when the user correctly guesses the number.

## Requirements

Your program must:

- Generate a random number within a specified range (e.g., 1 to 100).
- Prompt the user to input their guess.
- Provide feedback:
  - "Too high!" if the guess is greater than the target number.
  - "Too low!" if the guess is less than the target number.
  - "Congratulations! You guessed the number in X attempts!" if the guess is correct.
- Count and display the number of attempts once the user guesses correctly.
- Ensure the program exits gracefully once the correct guess is made.

## Input

The program should take user input through standard input (the console).

- **Range:** The program generates a random number between 1 and 100.
- **Guess:** The user inputs an integer guess.

```bash
Guess the number (between 1 and 100):
> 50
```

## Output

The program should provide feedback after each guess:

- If the guess is too high:

```bash
Too high! Try again.
```

- If the guess is too low:

```bash
Too low! Try again.
```

- If the guess is correct:

```bash
Congratulations! You guessed the number in 7 attempts!
```

## Constraints

- The random number must be geenrated using Rust's `rand` crate.
- The program should handle invalid inputs gracefully (e.g., non-numeric guesses).
- Ensure guesses are within bounds (1 to 100):
  - If the user inputs an out-of-bound guess, print a warning and ask for a new input.
