use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::explorer::Explorer;

mod explorer;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Go { path: PathBuf },
    List,
}

fn main() {
    let args = Args::parse();
    let mut exporer = Explorer {
        path: PathBuf::from("."),
    };
    match args.command {
        Commands::Go { path } => {
            exporer.set_path(path);
        }
        Commands::List => {
            let list = exporer.list_dict();
            for file in list {
                print!("{file} ");
            }
        }
    }
}
