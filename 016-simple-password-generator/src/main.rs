use rand::rng;
use rand::seq::IteratorRandom;
use std::io;

fn generate_password(length: usize, complexity: u8) -> String {
    let mut password = String::new();
    let mut rng = rng();

    // Old implementation, but since choose() must use along with
    // string slice rather than a range as specified below
    //
    // for _ in 0..length {
    //     let getchar = match rng.random_range(0..complexity) {
    //         0 => ('a'..='z').choose(&mut rng).unwrap(),
    //         1 => ('A'..='Z').choose(&mut rng).unwrap(),
    //         2 => ('0'..='9').choose(&mut rng).unwrap(),
    //         _ => ('!'..='/').choose(&mut rng).unwrap(),
    //     };

    //     password.push(getchar);
    // }

    // Define character sets
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()-_=+[]{};:'\",.<>?/";

    // Build the charcater pool based on complexity
    let mut charset = String::from(lowercase);
    if complexity > 1 {
        charset.push_str(uppercase);
    }
    if complexity > 2 {
        charset.push_str(numbers);
    }
    if complexity > 3 {
        charset.push_str(symbols);
    }

    // Generate the pasword
    for _ in 0..length {
        if let Some(ch) = charset.chars().choose(&mut rng) {
            password.push(ch);
        }
    }

    password
}

fn main() {
    loop {
        println!("Enter password length (min 6):");

        let mut length = String::new();
        io::stdin()
            .read_line(&mut length)
            .expect("Failed to read line");

        // Convert length to usize
        let length: usize = length.trim().parse().expect("Invalid length");
        if length < 6 {
            println!("Password length must be of 6 characters minimum.");
            continue;
        }

        println!("Select password complexity:");
        println!(
            "1: lowercase letters only\n2: lowercase + uppercase\n3: lowercase + uppercase + numbers\n4: lowercase + uppercase + numbers + symbols"
        );

        let mut complexity = String::new();
        io::stdin()
            .read_line(&mut complexity)
            .expect("Failed to read line");

        // Convert complexity to u8
        let complexity: u8 = complexity.trim().parse().expect("Invalid complexity");
        if complexity > 4 {
            println!("Error: Password complexity must be within 1 to 4");
            continue;
        }
        println!("{}", generate_password(length, complexity).to_string());
        break;
    }
}
