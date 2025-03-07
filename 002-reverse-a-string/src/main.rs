use std::io;

fn reverse_string(source_str: String) -> String {
    let source_len = source_str.len();
    let mut reversed_str = String::with_capacity(source_len);

    for i in 0..source_len {
        reversed_str.push(source_str.chars().nth(source_len - 1 - i).unwrap());
    }

    reversed_str
}

fn main() {
    // Input string
    let mut str = String::new();

    println!("Enter a string: ");

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line.");

    // Remnove '\n' or newline from the string
    let trimmed_str = str.trim();

    println!("Reversed String: {}", reverse_string(trimmed_str.to_string()));
}
