#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process;

use codecrafters_interpreter::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {filename}");
                String::new()
            });

            if file_contents.is_empty() {
                println!("EOF  null");
            } else {
                let mut had_error = false;
                for token in Lexer::new(&file_contents) {
                    match token {
                        Ok(token) => println!("{token}"),
                        Err(e) => {
                            had_error = true;
                            eprintln!("{e}");
                        }
                    }
                }

                if had_error {
                    process::exit(65)
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {command}");
        }
    }
}
