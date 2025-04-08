use clap::Parser;
use std::{collections::HashSet, io::{self, Read}};

/// CLI tool to capture and display stdout of a command
#[derive(Parser, Debug)]
#[command(
    author = "Your Name",
    version = "1.0",
    about = "Removes duplicate entries from a list"
)]
struct Cli {
    /// Specify the separating character
    #[arg(short, long)]
    divisor: Option<String>,
}

fn process_input<'a>(input: &'a str, divisor: Option<&'a str>) -> HashSet<&'a str> {

    let lines: HashSet<&str> = input.lines().into_iter().collect();
    lines
}

pub fn main() {
    let cli = Cli::parse();

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let results = process_input(&input, cli.divisor.as_deref());

    for entry in results {
        println!("{}", entry);
    }
}