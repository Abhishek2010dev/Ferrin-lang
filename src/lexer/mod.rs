use crate::token::{Token, lookup_ident};

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: input.as_bytes(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }
    #[inline(always)]
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    #[inline(always)]
    fn peek_char(&self) -> Option<u8> {
        self.input.get(self.read_position).copied()
    }

    #[inline(always)]
    fn skip_whitespace(&mut self) {
        while matches!(self.ch, Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.read_char();
        }
    }

    #[inline(always)]
    fn read_identifier(&mut self) -> &str {
        self.read_with_condition(|ch| (ch as char).is_alphabetic())
    }

    #[inline(always)]
    fn read_with_condition<F>(&mut self, condition: F) -> &str
    where
        F: Fn(u8) -> bool,
    {
        let start = self.position;
        while let Some(ch) = self.ch {
            if condition(ch) {
                self.read_char();
            } else {
                break;
            }
        }
        std::str::from_utf8(&self.input[start..self.position]).unwrap()
    }

    #[inline(always)]
    fn read_number(&mut self) -> &str {
        self.read_with_condition(|x| x.is_ascii_digit())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let token = match self.ch? {
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b',' => Token::Comma,
            b'!' if self.peek_char()? == b'=' => {
                self.read_char();
                Token::NotEq
            }
            b'=' if self.peek_char()? == b'=' => {
                self.read_char();
                Token::Eq
            }
            b'=' => Token::Assign,

            b'!' => Token::Bang,
            ch if ch.is_ascii_digit() => {
                return Some(Token::Int(self.read_number().to_owned()));
            }
            ch if (ch as char).is_alphabetic() => {
                return Some(lookup_ident(self.read_identifier()));
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
        let mut lexer = Lexer::new(input);

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
