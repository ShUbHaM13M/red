mod app;
mod command_parser;
mod lexer;

use command_parser::command_parser;
use lexer::tokenizer::{tokenize, Token};
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut app = command_parser(args);
    let result = app.open_file(0);
    if result.is_err() {
        eprintln!("Unable to open the file");
        exit(1)
    }
    let content = app.get_active_file_content();
    if content.is_err() {
        eprintln!("Unable to read content of the file");
    }

    // println!("\x1b[2J");
    let tokens = tokenize(&content.unwrap());
    // println!("{:#?}", tokens);
    for token in tokens {
        match token {
            Token::Keyword(data) => print!("\x1b[35m{data}\x1b[0m"),
            Token::Identifier(data) => print!("{data}"),
            Token::Macro(data) => print!("\x1b[1;34m{data}\x1b[0m"),
            Token::StringLit(data) => print!("\x1b[32m{data}\x1b[0m"),
            Token::Delimeter(data) => print!("\x1b[31m{data}\x1b[0m"),
            Token::Comment(data) => print!("\x1b[4m{data}\x1b[0m"),
            Token::NewLine() => println!(),
            Token::Whitespace() => print!(" "),
        }
    }
}
