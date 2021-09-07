use sourcepawn_lexer::Token;
use sourcepawn_lexer::TokenKind::*;
use sourcepawn_lexer::KeywordKind;

mod ast;
mod error;

use ast::*;
use error::ParserError;

type Result<T> = std::result::Result<T, ParserError>;

struct Parser<'a> {
    tokens: std::slice::Iter<'a, Token>,
    current_pos: usize,
    current_token: Option<&'a Token>
}

impl <'a> Parser<'a> {
    fn new<'b>(tokens: &'b [Token]) -> Parser<'b> {
        Parser {
            tokens: tokens.iter(),
            current_token: None,
            current_pos: 0
        }
    }

    fn next_node(&mut self) -> Box<dyn ASTNode> {
        self.toplevel_statement();
        unimplemented!();
    }

    fn toplevel_statement(&mut self) -> Option<TopLevelStatement> {
        match self.current_token {
            Some(Token { kind: Ident { .. }, len: _ }) => println!("Got ident"),
            _ => println!("Wrong token"),
        };
        None
    }

    fn function_declaration(&mut self) -> Result<TopLevelStatementKind> {
        match self.current_token {
            Some(Token { kind: Keyword { kind: KeywordKind::Public }, len: _}) => Err(ParserError::new("Whoops".to_owned())),
            _ => Err(ParserError::new("Dead".to_owned())),
        }
    }

    fn function_definition(&mut self) -> Result<TopLevelStatementKind> {
        match self.current_token {
            Some(Token { kind: Keyword { kind: KeywordKind::Public }, len: _}) => Err(ParserError::new("Whoops".to_owned())),
            _ => Err(ParserError::new("Dead".to_owned())),
        }
    }

    fn consume_whitespace(&mut self) {
        loop {
            match self.next() {
                Some(Token { kind: Whitespace, len: _ }) => continue,
                _ => return break,
            };
        }
    }

    fn next(&mut self) -> Option<&Token> {
        self.current_token = self.tokens.next();
        self.current_token
    }

}

pub fn parse(tokens: &[Token]) {
    Parser::new(tokens).next_node();
}
