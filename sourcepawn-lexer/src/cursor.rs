use std::str::Chars;

pub(crate) const EOF: char = '\0';

pub(crate) struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
}

impl <'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            initial_len: input.len(),
            chars: input.chars(),
        }
    }

    pub(crate) fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub(crate) fn peek(&self) -> char {
        self.nth_char(0)
    }

    pub(crate) fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap_or(EOF)
    }

    pub(crate) fn chars(&self) -> Chars<'a> {
        self.chars.clone()
    }

    pub(crate) fn bump(&mut self) -> Option<char> {
        Some(self.chars.next()?)
    }
}
