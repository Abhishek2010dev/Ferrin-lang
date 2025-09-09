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

impl TokenType {
    pub fn from_ident(ident: &str) -> Self {
        match ident {
            "func" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Ident,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}
