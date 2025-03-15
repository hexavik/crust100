enum DaysOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DaysOfWeek {
    fn from_str(s: &str) -> Result<DaysOfWeek, String> {
        match s.to_lowercase().as_str() {
            "monday" => Ok(DaysOfWeek::Monday),
            "tuesday" => Ok(DaysOfWeek::Tuesday),
            "wednesday" => Ok(DaysOfWeek::Wednesday),
            "thursday" => Ok(DaysOfWeek::Thursday),
            "friday" => Ok(DaysOfWeek::Friday),
            "saturday" => Ok(DaysOfWeek::Saturday),
            "sunday" => Ok(DaysOfWeek::Sunday),
            _ => Err(format!("Invalid day: {}", s)),
        }
    }
}

fn print_message(day: &DaysOfWeek) {
    match day {
        DaysOfWeek::Monday => println!("Monday - Moon's Day"),
        DaysOfWeek::Tuesday => println!("Tuesday - Mars' Day"),
        DaysOfWeek::Wednesday => println!("Wednesday - Mercury's Day"),
        DaysOfWeek::Thursday => println!("Thursday - Jupiter's Day"),
        DaysOfWeek::Friday => println!("Friday - Venus' Day"),
        DaysOfWeek::Saturday => println!("Saturday - Saturn's Day"),
        DaysOfWeek::Sunday => println!("Sunday - Sun's Day"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    match DaysOfWeek::from_str(&args[1]) {
        Ok(day) => print_message(&day),
        Err(err) => eprintln!("{}", err),
    }
}
