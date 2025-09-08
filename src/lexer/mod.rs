use crate::token::{Token, TokenType};

#[derive(Default)]
pub struct Lexer {
    input: String,
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
        self.read_position += 1;
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.ch?;
        let (token_type, literal) = match ch {
            '=' => (TokenType::Assign, ch),
            ';' => (TokenType::Semicolon, ch),
            '(' => (TokenType::LParen, ch),
            ')' => (TokenType::RParen, ch),
            '{' => (TokenType::LBrace, ch),
            '}' => (TokenType::RBrace, ch),
            '+' => (TokenType::Plus, ch),
            ',' => (TokenType::Comma, ch),
            _ => (TokenType::Illegal, ch),
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
        let input = "=+(){},;".to_owned();
        let tests = [
            (TokenType::Assign, '='),
            (TokenType::Plus, '+'),
            (TokenType::LParen, '('),
            (TokenType::RParen, ')'),
            (TokenType::LBrace, '{'),
            (TokenType::RBrace, '}'),
            (TokenType::Comma, ','),
            (TokenType::Semicolon, ';'),
        ];

        let mut lexer = Lexer::new(input);

        for (expected_type, expected_literal) in tests {
            let token = lexer.next().expect("Expected a token");
            assert_eq!(token.token_type, expected_type);
            assert_eq!(token.literal, expected_literal);
        }

        assert!(lexer.next().is_none());
    }
}
