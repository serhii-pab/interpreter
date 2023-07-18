use crate::token::Token;
use std::str::Chars;

pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,

    // maybe we can get rid of them later
    position: u32,
    read_position: u32,

    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        let mut l = Self {
            ch: None,
            chars: input.chars(),
            input,

            position: 0,
            read_position: 0,
        };
        l.read_char();
        l
    } 

    fn read_char(&mut self) -> Option<char> {
        self.ch = self.chars.next();
        self.ch
    }

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some('=') => Token::Assign,
            Some(';') => Token::Semicolon,
            Some('(') => Token::Lparen,
            Some(')') => Token::Rparen,
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            Some('+') => Token::Plus,
            Some(',') => Token::Comma,
            Some(_) => Token::Illegal,
            None => Token::Eof,
        };

        self.read_char();

        token
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let mut lexer = Lexer::new(input);

        let expected_tokens = [
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
        ];

        for expected_token in expected_tokens {
            assert_eq!(expected_token, lexer.next_token());
        }
    }
}
