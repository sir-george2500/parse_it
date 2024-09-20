use ::std::fs;
use ::std::process::exit;
use std::env::args;
mod lexer;

use lexer::lex;

fn main() {
    // get the environment variable
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path_or_json_string>", args[0]);
        exit(1);
    }

    let input = &args[1];

    let content = if let Ok(file_content) = fs::read_to_string(input) {
        file_content
    } else {
        input.clone()
    };

    //     Try to lex and parse the input
    match lex(&content) {
        Ok(_) => {
            println!("Valid JSON: {}.", content);
            exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
