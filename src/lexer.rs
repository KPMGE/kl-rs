use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    current_position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            current_position: 0,
            read_position: 0,
            current_char: None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();

        match self.current_char {
            Some('(') => Token::RightParentesis,
            Some(')') => Token::LeftParentesis,
            Some('{') => Token::RightBrace,
            Some('}') => Token::LeftBrace,
            Some('=') => Token::Assign,
            Some('+') => Token::Plus,
            Some(',') => Token::Comma,
            None => Token::Eof,
            _ => Token::Illegal,
        }
    }

    fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.current_char = None;
            return;
        }

        let ch = self.input.chars().nth(self.read_position);

        self.current_char = ch;
        self.current_position = self.read_position;
        self.read_position += 1;
    }
}
