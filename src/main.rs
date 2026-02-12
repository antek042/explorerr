use std::{
    io::{self, Read},
    path::PathBuf,
};

mod explorer;

fn main() {
    let mut input = String::new();
    loop {
        input.clear();

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
    }
}
