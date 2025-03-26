use clap::Parser;
use std::collections::HashMap;
use std::env;
use std::os::unix::fs;

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
        let possible_callables: Vec<&String> = callables.keys().collect();
        expand(&possible_callables);
    }
}

fn expand(possible_callables: &Vec<&String>) {
    println!("{:?}", possible_callables);
    let binary_path = env::current_exe().unwrap();
    println!("{:?}", binary_path);

    let mut calling_path = binary_path.clone();
    calling_path.pop();

    for callable in possible_callables {
        let this_symlink = calling_path.join(callable);
        match fs::symlink(&binary_path, &this_symlink) {
            Ok(()) => println!("Created symlink for {} at {:?}", callable, this_symlink),
            Err(error) => println!(
                "Failed to create symlink for {} at {:?}: {}",
                callable, this_symlink, error
            ),
        };
    }
}
