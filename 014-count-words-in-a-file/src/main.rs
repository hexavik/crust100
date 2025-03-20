use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    //let contents = fs::read_to_string("sample.txt").expect("Unable to read file");
    //println!("with text:\n{}", contents);

    let file = File::open("sample.txt")?;
    let reader = BufReader::new(file);

    // let mut word_count = 0;
    // for line_result in reader.lines() {
    //     if let Ok(line) = line_result {
    //         word_count += line.split_whitespace().count();
    //     }
    // }

    let word_count: usize = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.split_whitespace().count())
        .sum();

    println!("Words in a file: {}", word_count);

    Ok(())
}
