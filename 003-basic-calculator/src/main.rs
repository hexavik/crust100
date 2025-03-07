use std::io;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn parse_expression(expr: String) -> Option<i32> {
    let mut num1_str = String::new();
    let mut operator_str = String::new();
    let mut num2_str = String::new();

    let mut found_operator = false;

    for c in expr.chars() {
        if c.is_digit(10) && !found_operator {
            num1_str.push(c);
        } else if "+-*/".contains(c) && !found_operator {
            operator_str.push(c);
            found_operator = true;
        } else if c.is_digit(10) && found_operator {
            num2_str.push(c);
        }
    }

    if num1_str.is_empty() || operator_str.is_empty() || num2_str.is_empty() {
        return None;  // Missing part
    }

    let num1: i32 = num1_str.parse().ok()?;
    let num2: i32 = num2_str.parse().ok()?;
    let operator: char = operator_str.chars().next()?;

    match operator {
        '+' => Some(add(num1, num2)),
        '-' => Some(subtract(num1, num2)),
        '*' => Some(multiply(num1, num2)),
        '/' => divide(num1, num2),
        _ => None,
    }
}

fn main() {
    println!("Enter an expression (e.g., 5 + 3): ");

    let mut expression = String::new();

    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line.");

    let result = parse_expression(expression.trim().to_string());

    match result {
        Some(x) => println!("Result: {x}"),
        None => println!("Invalid input"),
    }
}
