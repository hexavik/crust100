fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Error: Cannot devide by zero.".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let a = 10.0;
    let b = 2.0;
    match safe_divide(a, b) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("{}", error),
    }

    let a = 5.0;
    let b = 0.0;
    match safe_divide(a, b) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("{}", error),
    }
}
