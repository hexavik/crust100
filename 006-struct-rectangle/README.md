# Create a `struct Rectangle` with Methods to Calculate Area and Perimeter

## Problem Statement

Create a `Rectangle` struct in Rust with the following:

- Two fields: `width` and `height`.
- Methods to calculate:
  - **Area** of the rectangle.
  - **Perimeter** of the rectangle.

The goal is to practice defining structs, implementing methods, and using `self` in Rust.

## Requirements

- Define a struct named `Rectangle` with `width` and `height` fields (both of type `f64`).
- Implement the followin methods:
  - `area(&self) -> f64`: Returns the area of the rectangle.
  - `perimeter(&self) -> f64`: Returns the perimeter of the rectangle.
- Create an instance of the `Rectangle` sgtruct and call both methods.

## Input

The program should create a rectangle like this:

```rust
let rect = Rectangle {
    width: 5.0,
    height: 3.0
};
```

## Output

The output should display the area and perimeter:

```bash
Area of the rectangle: 15.0
Perimeter of the rectangle: 16.0
```

## Constraints

- `width` and `height` should be non-negative `(>= 0)`.
- The methods should **borrow self immutably** (`&self`).
- Consider adding checks to ensure **invalid rectangles** (with negative dimensions) cannot be created.
