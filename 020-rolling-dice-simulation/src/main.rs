use clap::Parser;
use rand::{Rng, rng};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of rolls to simulate
    #[arg(value_parser)]
    rolls: i32,
}

fn main() {
    let args = Args::parse();
    let mut rolls = args.rolls;
    let mut rng = rng();

    let mut countmap: HashMap<i32, usize> = HashMap::new();

    while rolls > 0 {
        let roll = rng.random_range(1..=6);
        *countmap.entry(roll).or_insert(0) += 1;
        rolls -= 1;
    }

    println!("Results after {} rolls:", args.rolls);
    // for (roll, count) in countmap {
    //     if count > 1 {
    //         println!("{}: {} times", roll, count);
    //     } else {
    //         println!("{}: {} time", roll, count);
    //     }
    // }
    for roll in 1..=6 {
        let count = countmap.get(&roll).unwrap_or(&0);

        if count == &1 {
            println!("{}: {} time", roll, count);
        } else {
            println!("{}: {} times", roll, count);
        }
    }
}
