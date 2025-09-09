use crate::token::Token;

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

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.read_position)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let ch = self.ch?;
        let token = match ch {
            '=' if self.peek_char()? == '=' => {
                self.read_char();
                Token::Eq
            }
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LT,
            '>' => Token::GT,
            ',' => Token::Comma,
            '!' if self.peek_char()? == '=' => {
                self.read_char();
                Token::NotEq
            }
            '!' => Token::Bang,
            ch if ch.is_alphabetic() => {
                return Some(self.read_identifier()?.into());
            }
            ch if ch.is_ascii_digit() => {
                return Some(Token::Int(self.read_number()?));
            }
            _ => Token::Illegal,
        };
        self.read_char();
        Some(token)
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn test_lexer() {
        let input = r#"
let x = 10;
let y = 20;

func add(a, b) {
    return a + b;
}

let result = add(x, y);

if result > 20 {
    return true;
} else {
    return false;
}
        "#;

        let tests = [
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("y".to_string()),
            Token::Assign,
            Token::Int("20".to_string()),
            Token::Semicolon,
            Token::Function,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("a".to_string()),
            Token::Comma,
            Token::Ident("b".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Ident("a".to_string()),
            Token::Plus,
            Token::Ident("b".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::If,
            Token::Ident("result".to_string()),
            Token::GT,
            Token::Int("20".to_string()),
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
        ];
        let mut lexer = Lexer::new(input.to_string());

        for (i, expected_token_type) in tests.iter().enumerate() {
            let token = lexer.next().expect("Expected a token");
            assert_eq!(
                token, *expected_token_type,
                "Token mismatch at position {i}: got {token:?}, expected {expected_token_type:?}",
            );
        }

        assert!(lexer.next().is_none());
    }
}
