use crate::token::{Token, TokenType};

#[derive(Default)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>,     // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> Option<String> {
        let position = self.position;
        while self
            .ch
            .map(|c| c.is_alphabetic() || '_' == c)
            .unwrap_or(false)
        {
            self.read_char();
        }
        self.input
            .get(position..self.position)
            .map(|v| v.to_string())
    }

    fn skip_whitespace(&mut self) {
        while self.ch.map(|ch| ch.is_whitespace()).unwrap_or(false) {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Option<String> {
        let position = self.position;
        while self.ch.map(|ch| ch.is_ascii_digit()).unwrap_or(false) {
            self.read_char();
        }
        self.input
            .get(position..self.position)
            .map(|v| v.to_string())
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let ch = self.ch?;
        let (token_type, literal) = match ch {
            '=' => (TokenType::Assign, ch.to_string()),
            ';' => (TokenType::Semicolon, ch.to_string()),
            '(' => (TokenType::LParen, ch.to_string()),
            ')' => (TokenType::RParen, ch.to_string()),
            '{' => (TokenType::LBrace, ch.to_string()),
            '}' => (TokenType::RBrace, ch.to_string()),
            '+' => (TokenType::Plus, ch.to_string()),
            ',' => (TokenType::Comma, ch.to_string()),
            ch if ch.is_alphabetic() => {
                let literal = self.read_identifier()?;
                return Some(Token::new(TokenType::from_ident(&literal), literal));
            }
            ch if ch.is_ascii_digit() => {
                let literal = self.read_number()?;
                return Some(Token::new(TokenType::Int, literal));
            }
            _ => (TokenType::Illegal, ch.to_string()),
        };
        self.read_char();
        Some(Token::new(token_type, literal))
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer, token::TokenType};

    #[test]
    fn test_lexer() {
        let input = r#"
let five = 5;
let ten = 10;
let add = func(x, y) {
x + y;
};
let result = add(five, ten);
        "#;

        let tests = [
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "func"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let token = lexer.next().expect("Expected a token");
            assert_eq!(
                token.token_type, *expected_type,
                "Token type mismatch at position {}: got {:?}, expected {:?}",
                i, token.token_type, expected_type
            );
            assert_eq!(
                token.literal, *expected_literal,
                "Token literal mismatch at position {}: got '{}', expected '{}'",
                i, token.literal, expected_literal
            );
        }

        assert!(lexer.next().is_none());
    }
}
