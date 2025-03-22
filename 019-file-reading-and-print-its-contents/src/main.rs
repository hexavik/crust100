// We use clap::parser to derive macro to automatically generate
// command-line argument parsing code. This eliminates the need to
// manually parse arguments.
use clap::Parser;
use std::fs::File;
use std::io::{self, Read};

// We define a struct Args to represent the command-line arguments.
// - The filepath field holds the file path.
// - The #[clap(value_parser)] attribute tells clap to parse the
// argument as a string.
// - The #[clap(author, version, about, long_about = None)] adds
// helpful information to the help output.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to read
    #[clap(value_parser)]
    filepath: String,
}

// To use ? operator for handling the errors, main should have
// return type of Result
fn main() -> io::Result<()> {
    // This function automatically parses the command-lien arguments
    // and creates an Args instance.
    let args = Args::parse();

    // Open the file specified by the filepath argument.
    // The ? operator handles potential errors.
    let mut file = File::open(&args.filepath)?;

    // To store binary data and read all bytes into the buffer
    // let mut buffer: Vec<u8> = Vec::new();
    // file.read_to_end(&mut buffer)?;

    // println!("File size: {} bytes", buffer.len());
    // println!("File contents:\n{:?}", buffer);

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("File contents:\n{}", contents);

    Ok(())
}
