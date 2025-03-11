fn convert_celsius_to_farhenheit(temp: f32) -> f32 {
    (temp * 9.0 / 5.0) + 32.0
}

fn convert_farhenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <temperature> <unit (C or F)>", args[0]);
        std::process::exit(1);
    }

    let temp_value = &args[1];
    let unit = &args[2].to_uppercase();

    let temp = match temp_value.parse::<f32>() {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Error: Invlaid temprature value");
            std::process::exit(1);
        }
    };

    if unit == "C" {
        println!("{}°C is {}°F", temp, convert_celsius_to_farhenheit(temp));
    } else if unit == "F" {
        println!("{}°F is {}°C", temp, convert_farhenheit_to_celsius(temp));
    } else {
        eprintln!("Error: Invalid unit. Use C or F");
        std::process::exit(1);
    }
}
