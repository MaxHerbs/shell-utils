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

fn process_input(input: &str, divisor: Option<&str>, index: usize, skip: usize) -> Vec<String> {
    let split_fn: Box<dyn Fn(&str) -> Option<&str>> = if let Some(div) = divisor {
        Box::new(move |line: &str| line.split(div).nth(index))
    } else {
        Box::new(move |line: &str| line.split_whitespace().nth(index))
    };

    input
        .lines()
        .skip(skip)
        .filter_map(|line| split_fn(line).map(String::from))
        .collect()
}

pub fn main() {
    let cli = Cli::parse();

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let results = process_input(&input, cli.divisor.as_deref(), cli.index, cli.skip);

    for part in results {
        println!("{}", part);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input_whitespace() {
        let input_str = "this is my test case";
        let result = process_input(input_str, None, 1, 0);
        assert_eq!(result, vec!["is"])
    }

    #[test]
    fn test_specific_delimeter() {
        let input_str = "this:is:my:test";
        let result = process_input(input_str, Some(":"), 1, 0);
        assert_eq!(result, vec!["is"])
    }

    #[test]
    fn multiline_text() {
        let input_str = "
            this is my test
            line two of my test
            this is line three
        ";
        let result = process_input(input_str, None, 1, 0);
        let solution = vec!["is", "two", "is"];
        assert_eq!(result, solution)
    }

    #[test]
    fn out_of_range() {
        let input_str = "
            this is my test
            line two of my test
            this is line three
        ";
        let result = process_input(input_str, None, 8, 0);
        let solution: Vec<String> = vec![];
        assert_eq!(result, solution)
    }

    #[test]
    fn mismatched() {
        let input_str = "
            this is my test
            two
            1
        ";
        let result = process_input(input_str, None, 2, 0);
        let solution: Vec<String> = vec!["my".to_owned()];
        assert_eq!(result, solution)
    }
}
