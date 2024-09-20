use std::env::args;
use std::fs;
use std::process::exit;

mod lexer;

use lexer::lex;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }

    let file_path = &args[1];

    // Read the content from the specified file
    let content = match fs::read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            exit(1);
        }
    };

    // Try to lex and parse the input
    match lex(&content) {
        Ok(_) => {
            println!("Valid JSON: \n {}.", content);
            exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
