use sourcepawn_lexer::Token;
use sourcepawn_lexer::TokenKind::*;
use sourcepawn_lexer::KeywordKind;
use std::iter::Peekable;
use std::slice::Iter;

mod ast;
mod error;

use ast::*;
use error::ParserError;

type Result<T> = std::result::Result<T, ParserError>;

struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    current_pos: usize,
    current_token: Token,
 }

impl <'a> Parser<'a> {
    fn new<'b>(tokens: &'b [Token]) -> Parser<'b> {
        Parser {
            tokens: tokens.iter().peekable(),
            current_token: Token::dummy(),
            current_pos: 0
        }
    }

    fn parse(&mut self) -> Result<Program> {
        let mut statements = vec![];
        loop {
            match self.top_level_statement() {
                Ok(statement) => statements.push(statement),
                Err(e) => return Err(e)
            }
        }

        Ok(Program {
            statements
        })
    }

    fn top_level_statement(&mut self) -> Result<TopLevelStatement> {
        if self.current_token.is_ident() {
            
        }
        
        unimplemented!()
    }

    fn function_declaration(&mut self) -> Result<FunctionDecl> {
        match self.current_token {
            Token { kind: Keyword { kind: KeywordKind::Public }, len: _} => unimplemented!(),
            _ => unimplemented!(),
        }
    }

    fn function_definition(&mut self) -> Result<Function> {
        match self.current_token {
            Token { kind: Keyword { kind: KeywordKind::Public }, len: _} => unimplemented!(),
            _ => unimplemented!(),
        }
    }

    fn consume_whitespace(&mut self) {
        loop {
            match self.next() {
                Token { kind: Whitespace, len: _ } => continue,
                _ => break,
            };
        }
    }

    fn next(&mut self) -> &Token {
        match self.tokens.next() {
            Some(token) => {
                self.current_pos = self.current_pos + token.len;
                self.current_token = *token
            },
            _ => { 
                self.current_token = Token::eof()
            }
        }
        &self.current_token
    }

    fn peek(&mut self) -> Option<&&Token> {
        self.tokens.peek()
    }

}

pub fn parse(tokens: &[Token]) {
    Parser::new(tokens).parse();
}
