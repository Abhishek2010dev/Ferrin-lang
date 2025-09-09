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

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "func" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            ch => Token::Ident(ch.to_string()),
        }
    }
}
