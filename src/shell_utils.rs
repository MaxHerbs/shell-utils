use clap::Parser;
use std::collections::HashMap;
use std::env;

#[derive(Parser, Debug)]
#[command(
    name = "Shell-Utils",
    version = "1.0",
    about = "General shell utils for command-line manipulations"
)]
struct Cli {
    /// Create the neccessary symlinks
    #[arg(short, long)]
    expand: bool,
}

fn parse_args() -> Cli {
    Cli::parse()
}

pub fn main(callables: HashMap<String, fn()>) {
    let args = parse_args();
    println!("This is my shell utils");

    if args.expand {
        let listed_callables: Vec<&String> = callables.keys().collect();
        println!("{:?}", listed_callables);
        let calling_binary = env::current_exe().unwrap();
        println!("{:?}", calling_binary);
    }
}

// fn drop_last<T>(list: &Vec<T>) -> Vec<T> {
//     let mut iterator = list.iter().peekable();
//     let mut returning_vec: Vec<T> = Vec::new();
//
//     while let Some(item) = iterator.next() {
//         // Check if there's another element after the current one
//         if iterator.peek().is_some() {
//             returning_vec.push(&item);
//         }
//     }
//     vec![]
// }
