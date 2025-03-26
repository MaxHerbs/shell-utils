use clap::Parser;
use std::io::{self, Read};

/// CLI tool to capture and display stdout of a command
#[derive(Parser, Debug)]
#[command(
    author = "Your Name",
    version = "1.0",
    about = "Splits stdout by a given delimeter and outputs target slice."
)]
struct Cli {
    /// Index of the item to return
    index: usize,

    /// Number of lines to skip
    #[arg(short, long, default_value = "0")]
    skip: usize,
}

pub fn main() {
    let cli = Cli::parse();

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    input
        .lines()
        .skip(cli.skip)
        .map(|line| line.split_whitespace().nth(cli.index).unwrap_or(""))
        .for_each(|part| println!("{}", part));
}
