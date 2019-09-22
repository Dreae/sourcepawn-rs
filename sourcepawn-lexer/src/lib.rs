mod cursor;

use crate::cursor::{Cursor, EOF};

pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token {
            kind,
            len,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    LineComment,
    BlockComment,
    Whitespace,
    Ident,
    Literal { kind: LiteralKind, suffix_start: usize },
    Semicolon,
    Comma,
    Dot,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Eq,
    Bang,
    Lt,
    Gt,
    Minus,
    Plus,
    And,
    Or,
    Star,
    Slash,
    Caret,
    Percent,
    Unknown
}
use self::TokenKind::*;

#[derive(Clone, Copy, Debug)]
pub enum LiteralKind {
    Char,
    String,
    Integer,
    Float0
}
use self::LiteralKind::*;

pub fn first_token(input: &str) -> Token {
    Cursor::new(input).advance_token()
}

pub fn tokenize(mut source: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if source.is_empty() {
            return None;
        }

        let token = first_token(source);
        source = &source[token.len..];
        Some(token)
    })
}

impl Cursor<'_> {
    fn advance_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();
        let token_kind = match first_char {
            '/' => match self.peek() {
                '/' => self.line_comment(),
                _ => Slash,
            }
            _ => Unknown
        };

        Token::new(token_kind, self.len_consumed())
    }

    fn line_comment(&mut self) -> TokenKind {
        self.bump();
        loop {
            match self.peek() {
                '\n' => break,
                EOF => break,
                _ => {
                    self.bump();
                }
            }
        }
        LineComment
    }
}
