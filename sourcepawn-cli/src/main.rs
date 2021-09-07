use clap::{Arg, App};
use sourcepawn_lexer::tokenize;
use sourcepawn_parser::parse;
use std::fs::read_to_string;
use std::process::exit;
use sourcepawn_lexer::Token;

fn main() {
    let matches = App::new("sourcepawn")
        .arg(Arg::with_name("INPUT")
             .help("The sourcepawn file to use")
             .required(true)
             .index(1))
        .get_matches();

    let source: String = match read_to_string(matches.value_of("INPUT").unwrap()) {
        Ok(string) => string,
        Err(err) => {
            println!("Error reading input file: {}", err);
            exit(1);
        }
    };
    let token_stream = tokenize(&source);
    let tokens: Vec<Token> = token_stream.collect();
    println!("{:?}", tokens);
    parse(&tokens);
}
