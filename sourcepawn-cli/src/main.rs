use clap::{Arg, App};
use sourcepawn_lexer::tokenize;
use std::fs::read_to_string;
use std::error::Error;
use std::process::exit;

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
            println!("Error reading input file: {}", err.description());
            exit(1);
        }
    };
    let tokens = tokenize(&source);
    for token in tokens {
        println!("{:?}", token);
    }
}
