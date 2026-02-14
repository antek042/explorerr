use rustyline::hint::HistoryHinter;
use rustyline::{completion::FilenameCompleter, CompletionType, Config, Editor};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};
use std::{env, fs, path::PathBuf};
use walkdir::WalkDir;

mod explorer;

#[derive(Completer, Helper, Hinter, Highlighter, Validator)]
struct Helper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

fn main() {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let helper = Helper {
        completer: FilenameCompleter::new(),
        hinter: HistoryHinter::new(),
    };
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(helper));

    let mut input = String::new();
    let mut explorer_ = explorer::Explorer {
        path: PathBuf::from("."),
    };
    loop {
        let prompt = format!("{} ", env::current_dir().unwrap().display());

        let readline = match rl.readline(&prompt) {
            Ok(line) => line,
            Err(_) => break,
        };
        let input = readline.trim().to_string();

        rl.add_history_entry(&input).unwrap();

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        match command {
            "go" => {
                env::set_current_dir(parts[1]).unwrap();
                explorer_.set_path(PathBuf::from(env::current_dir().unwrap()));
            }
            "list" => {
                let dict_files = explorer_.list_dict();
                for file in dict_files {
                    print!("{file} ");
                }
                println!();
            }
            "move" => {
                fs::rename(parts[1], parts[2]).unwrap();
            }
            "find" => {
                let pattern = parts[1];
                let walker = WalkDir::new(env::current_dir().unwrap()).into_iter();

                let finded_files: Vec<String> = walker
                    .filter_map(|f| f.ok())
                    .filter(|f| f.file_type().is_file())
                    .filter_map(|f| {
                        let name = f.file_name().to_string_lossy();

                        let matches = if pattern.starts_with('*') {
                            let extension = &pattern[1..];
                            name.ends_with(extension)
                        } else {
                            name == pattern
                        };

                        if matches {
                            Some(f.path().display().to_string())
                        } else {
                            None
                        }
                    })
                    .collect();
                for file in finded_files {
                    println!("Found: {}", file);
                }
            }
            "quit" => {
                break;
            }
            _ => {
                println!("Undefinded command!");
            }
        }
    }
}
