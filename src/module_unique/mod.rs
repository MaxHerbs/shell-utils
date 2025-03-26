use clap::Parser;
use std::{
    collections::HashSet,
    io::{self, Read},
};

/// Return only the unique items in a list
#[derive(Parser, Debug)]
#[command(
    author = "Your Name",
    version = "1.0.0",
    about = "Removes duplicate entries from a list"
)]
struct Cli {
    /// Specify the separating character (defaults to newline)
    #[arg(short, long, default_value = "\n")]
    divisor: String,
}

fn process_input<'a>(input: &'a str, divisor: &str) -> HashSet<&'a str> {
    input.split(divisor).collect()
}

pub fn main() {
    let cli = Cli::parse();

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let results = process_input(&input, &cli.divisor);

    for entry in results {
        println!("{entry}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_newline_divisor() {
        let input = "apple\nbanana\napple\norange\nbanana";
        let result = process_input(input, "\n");
        assert_eq!(result.len(), 3);
        assert!(result.contains("apple"));
        assert!(result.contains("banana"));
        assert!(result.contains("orange"));
    }

    #[test]
    fn test_custom_comma_divisor() {
        let input = "car,bike,car,train,bike";
        let result = process_input(input, ",");
        assert_eq!(result.len(), 3);
        assert!(result.contains("car"));
        assert!(result.contains("bike"));
        assert!(result.contains("train"));
    }
}
