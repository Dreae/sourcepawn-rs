mod cursor;

use crate::cursor::{Cursor, EOF};
use std::string::String;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    LineComment,
    BlockComment,
    Whitespace,
    Ident { name: String },
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
    Keyword { kind: KeywordKind },
    Unknown
}
use self::TokenKind::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
    Char,
    String,
    Integer,
    Float
}
use self::LiteralKind::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeywordKind {
    Public,
    Const,
    Native,
    Return,
    For,
    If,
    Else,
    While,
    MethodMap,
    Stock,
    Do,
    Switch,
    Case,
    Break,
    Default,
    Continue,
    New,
    Decl,
    Delete,
    Forward,
    Property,
    Enum,
    Functag,
    Funcenum,
    Struct,
    Typedef,
    Typeset,
    Static,
    ViewAs,
}
use self::KeywordKind::*;

// Taken from the rust compiler:
// https://github.com/rust-lang/rust/blob/master/src/librustc_lexer/src/lib.rs#L105
// See [UAX #31](http://unicode.org/reports/tr31) for definitions of these
// classes.

/// True if `c` is considered a whitespace according to Rust language definition.
pub fn is_whitespace(c: char) -> bool {
    // This is Pattern_White_Space.
    //
    // Note that this set is stable (ie, it doesn't change with different
    // Unicode versions), so it's ok to just hard-code the values.

    match c {
        // Usual ASCII suspects
        | '\u{0009}' // \t
            | '\u{000A}' // \n
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space

        // NEXT LINE from latin1
            | '\u{0085}'

        // Bidi markers
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}' // PARAGRAPH SEPARATOR
            => true,
        _ => false,
    }
}

pub fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

pub fn first_token(input: &str) -> Token {
    Cursor::new(input).advance_token()
}

pub fn is_keyword(ident: &str) -> Option<KeywordKind> {
    match ident {
        "if" => Some(If),
        "else" => Some(Else),
        "for" => Some(For),
        "while" => Some(While),
        "do" => Some(Do),
        "switch" => Some(Switch),
        "case" => Some(Case),
        "default" => Some(Default),
        "return" => Some(Return),
        "break" => Some(Break),
        "continue" => Some(Continue),
        "new" => Some(New),
        "decl" => Some(Decl),
        "delete" => Some(Delete),
        "forward" => Some(Forward),
        "native" => Some(Native),
        "property" => Some(Property),
        "public" => Some(Public),
        "stock" => Some(Stock),
        "enum" => Some(Enum),
        "funcenum" => Some(Funcenum),
        "functag" => Some(Functag),
        "methodmap" => Some(MethodMap),
        "struct" => Some(Struct),
        "typedef" => Some(Typedef),
        "typeset" => Some(Typeset),
        "static" => Some(Static),
        "view_as" => Some(ViewAs),
        _ => None,
    }
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
            },
            c if is_whitespace(c) => self.whitespace(),
            c if is_ident_start(c) => self.ident(c),
            ';' => Semicolon,
            ',' => Comma,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '=' => Eq,
            '!' => Bang,
            '>' => Gt,
            '<' => Lt,
            '-' => Minus,
            '+' => Plus,
            '*' => Star,
            '&' => And,
            '|' => Or,
            '^' => Caret,
            '%' => Percent,
            '"' => self.string(),
            '\'' => self.char(),
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

    fn whitespace(&mut self) -> TokenKind {
        while is_whitespace(self.peek()) {
            self.bump();
        }

        Whitespace
    }

    fn ident(&mut self, start: char) -> TokenKind {
        let mut name = String::new();
        name.push(start);
        while is_ident_continue(self.peek()) {
            name.push(self.bump().unwrap());
        }

        match is_keyword(&name) {
            Some(kind) => Keyword { kind },
            _ => Ident { name }
        }
    }

    fn string(&mut self) -> TokenKind {
        let suffix_start = self.len_consumed();
        loop {
            match self.peek() {
                '"' => {
                    self.bump();
                    break;
                },
                EOF => break,
                '\\' if self.nth_char(1) == '\\' || self.nth_char(1) == '"' => {
                    self.bump()
                },
                _ => self.bump()
            };
        }

        Literal { kind: String, suffix_start }
    }

    fn char(&mut self) -> TokenKind {
        let suffix_start = self.len_consumed();
        loop {
            match self.peek() {
                '\'' => {
                    self.bump();
                    break;
                },
                EOF => break,
                '\\' => {
                    self.bump();
                    self.bump();
                },
                _ => {
                    self.bump();
                }
            };
        }

        Literal { kind: Char, suffix_start }
    }
}
