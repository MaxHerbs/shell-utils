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

    /// Specify the separating character
    #[arg(short, long)]
    divisor: Option<String>,
}

pub fn main() {
    let cli = Cli::parse();

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let split_fn: Box<dyn Fn(&str) -> Option<&str>> = if let Some(divisor) = cli.divisor {
        Box::new(move |line: &str| line.split(&divisor).nth(cli.index))
    } else {
        Box::new(move |line: &str| line.split_whitespace().nth(cli.index))
    };

    input
        .lines()
        .skip(cli.skip)
        .filter_map(|line| split_fn(line))
        .for_each(|part| println!("{}", part));
}
