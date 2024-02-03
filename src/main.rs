use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

mod ast;
mod builtins;
mod lexer;
mod parser;

use lexer::{scan_tokens, Lexer, TokenType};
use parser::Parser;

const OUTPUT_TO_FILE: bool = true;

fn read_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!("Usage: cargo run -- <file>")
    }

    match read_file(args[0].as_str()) {
        Ok(source) => {
            let mut lexer = Lexer::new(source);
            scan_tokens(&mut lexer);
            if OUTPUT_TO_FILE {
                let output = format!("{:#?}", lexer.tokens);
                write_file("./lexer.out", &output);
            }
            let mut parser = Parser::new(lexer.tokens);
            println!("{:?}", parser.peek_token_type().unwrap());
        }
        Err(e) => panic!("Failed to read file: {}", e),
    }
}
