use std::process::exit;

use clap::Parser;
use gateway::local_file_gateway::LocalFileGateway;
use usecase::initialize_usecase;

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    text: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = initialize_usecase::run(LocalFileGateway) {
        println!("Process exit: reason {:?}", e);
        exit(1)
    }

    match cli.text {
        Some(text) => println!("{}", text),
        _ => println!("No text :)"),
    }
}
