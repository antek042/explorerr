use clap::Parser;
use std::path::PathBuf;

use crate::explorer::Explorer;

mod explorer;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long, default_value = ".")]
    pub path: String,
}

fn main() {
    let args = Args::parse();
    let temp = Explorer {
        path: PathBuf::from(args.path),
    };
    let list = temp.list_dict();
    for x in list {
        println!("{x}");
    }
}
