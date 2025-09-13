#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token {
    Illegal,

    // Identifiers + literals
    Ident(String), // "foobar", "x"
    Int(String),   // "123"

    // Operators
    Assign,   // "="
    Plus,     // "+"
    Minus,    // "-"
    Bang,     // "!"
    Asterisk, // "*"
    Slash,    // "/"

    LT, // "<"
    GT, // ">"

    Eq,    // ==
    NotEq, // !=

    // Delimiters
    Comma,     // ","
    Semicolon, // ";"

    LParen, // "("
    RParen, // ")"
    LBrace, // "{"
    RBrace, // "}"

    // Keywords
    Function, // "func"
    Let,      // "let"
    True,     // "true"
    False,    // "false"
    If,       // "if"
    Else,     // "else"
    Return,   // "return"
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "let" => Token::Let,
        "func" => Token::Function,
        "return" => Token::Return,
        "if" => Token::If,
        "else" => Token::Else,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Ident(ident.to_string()),
    }
}
