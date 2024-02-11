use std::collections::HashMap;

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
            input: input.clone(),
            current_position: 0,
            read_position: 1,
            current_char: input.chars().nth(0),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let keywords_map = HashMap::from([("fn", Token::Function), ("let", Token::Let)]);

        let token = match self.current_char {
            Some('(') => Token::RightParentesis,
            Some(')') => Token::LeftParentesis,
            Some('{') => Token::RightBrace,
            Some('}') => Token::LeftBrace,
            Some('=') => Token::Assign,
            Some('+') => Token::Plus,
            Some(',') => Token::Comma,
            Some(c) => {
                if c.is_alphabetic() {
                    let identifier = self.read_identifier();
                    match keywords_map.get(&identifier.as_str()) {
                        Some(tok) => tok.clone(),
                        None => Token::Identifier(identifier),
                    }
                } else {
                    Token::Illegal
                }
            }
            None => Token::Eof,
        };
        
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_alphabetic() {
                self.read_char();
                continue
            }
            break
        }

        let identifier = &self.input[start_pos..self.current_position];
        identifier.to_string()
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
