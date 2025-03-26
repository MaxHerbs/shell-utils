use clap::Parser;
use std::io::{self, Read};

/// Collapse an input to a single line
#[derive(Parser, Debug)]
#[command(
    author = "Your Name",
    version = "1.0.0",
    about = "Collapse the stdin to a single line"
)]
struct Cli;

fn process_input(input: &str) -> String {
    input.replace("\n", " ")
}

pub fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let result = process_input(&input);

    println!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input_whitespace() {
        let input_str = "this is my test case\nAnother line here!";
        let result = process_input(input_str);
        assert_eq!(result, "this is my test case Another line here!")
    }
}
