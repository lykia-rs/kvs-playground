use clap::{Parser, Subcommand};
use project_2::KvStore;
use std::process::exit;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();
    let mut store = KvStore::new();

    match &cli.command {
        Commands::Get { key } => {
            store.get(key.to_owned());
            exit(1);
        }
        Commands::Set { key, value } => {
            store.set(key.to_owned(), value.to_owned());
            exit(1);
        }
        Commands::Rm { key } => {
            store.remove(key.to_owned());
            exit(1);
        }
    }
}
