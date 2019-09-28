use sourcepawn_lexer::tokenize;
use sourcepawn_lexer::Token;
use sourcepawn_lexer::TokenKind::*;

mod ast;
mod error;

use ast::*;
use error::ParserError;

type Result<T> = std::result::Result<T, ParserError>;

pub fn parse(tokens: &[Token], source: &str) {
    let mut iter = tokens.iter().cloned();
    toplevel_statement(&mut iter);
}

fn toplevel_statement(tokens: &mut dyn Iterator<Item = Token>) -> Option<TopLevelStatement> {
    match consume_whitespace(tokens) {
        Some(Token { kind: Ident { .. }, len: _ }) => println!("Got ident"),
        _ => println!("Wrong token"),
    };
    None
}

fn function_declaration(tokens: &mut dyn Iterator<Item = Token>) -> Result<TopLevelStatementKind> {
    match consume_whitespace(tokens) {
        Some(Token { kind: Keyword { kind: Public }, len: _}) => Err(ParserError::new("Whoops".to_owned())),
        _ => Err(ParserError::new("Dead".to_owned())),
    }
}

fn consume_whitespace(tokens: &mut dyn Iterator<Item = Token>) -> Option<Token> {
    loop {
        match tokens.next() {
            Some(Token { kind: Whitespace, len: _ }) => tokens.next(),
            Some(token) => return Some(token),
            _ => return None,
        };
    }
}
