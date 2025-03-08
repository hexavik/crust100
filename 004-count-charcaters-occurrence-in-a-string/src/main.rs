use std::io;
use std::collections::HashMap;

fn main() {
    // Input string
    println!("Enter a string:");

    let mut input_str = String::new();

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line.");

    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for c in input_str.chars() {
        if c.is_ascii_alphabetic() {
            *char_counts.entry(c).or_insert(0) += 1;
        }
    }

    for (c, count) in char_counts {
        println!("{}: {}", c, count);
    }
}
