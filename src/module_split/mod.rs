use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Split", version = "1.0", about = "Split stdin by a delimeter")]
struct Cli {
    /// Dividing character
    #[arg(short, long)]
    divisor: Option<String>,
}

fn parse_args() -> Cli {
    Cli::parse()
}

pub fn main() {
    let args = parse_args();

    println!("This is my split");
    match &args.divisor {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, World!"),
    }
}
