#![allow(unused_variables)]
use std::env;
use std::fs;

use codecrafters_interpreter::Tokenizer;

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
                for token in Tokenizer::new(&file_contents) {
                    println!("{token}");
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {command}");
        }
    }
}
