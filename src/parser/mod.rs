use std::iter::Peekable;

use crate::lexer::Lexer;

pub struct Parser<'a> {
    tokens: Peekable<Lexer<'a>>,
}

impl<'a> From<Lexer<'a>> for Parser<'a> {
    fn from(value: Lexer<'a>) -> Self {
        Parser {
            tokens: value.peekable(),
        }
    }
}
