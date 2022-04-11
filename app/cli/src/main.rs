use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    text: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.text {
        Some(text) => println!("{}", text),
        _ => println!("No text :)"),
    }
}
