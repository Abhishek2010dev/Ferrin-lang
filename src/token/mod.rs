#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenType {
    Illegal,

    // Identifiers + literals
    Ident, // add, foobar, x, y ....
    Int,   // 123849

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: char,
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Self {
        Self {
            token_type,
            literal: ch,
        }
    }
}
