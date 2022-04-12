use clap::Parser;
use gateway::local_file_gateway::LocalFileGateway;
use usecase::storage_usecase;

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    text: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let _storage = storage_usecase::create_storage(LocalFileGateway);

    match cli.text {
        Some(text) => println!("{}", text),
        _ => println!("No text :)"),
    }
}
