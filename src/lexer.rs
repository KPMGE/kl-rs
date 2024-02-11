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

        self.skip_whitespaces();

        if self.current_char.is_none() {
            return Token::Eof;
        }

        let ch = self.current_char.unwrap();

        let token = match ch {
            '(' => Token::RightParentesis,
            ')' => Token::LeftParentesis,
            '{' => Token::RightBrace,
            '}' => Token::LeftBrace,
            '=' => Token::Assign,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => Token::Bang,
            '*' => Token::Asterisk,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            ',' => Token::Comma,
            '/' => Token::Slash,
            c => {
                if c.is_letter() {
                    let identifier = self.read_identifier();
                    return match keywords_map.get(&identifier.as_str()) {
                        Some(tok) => tok.clone(),
                        None => Token::Identifier(identifier),
                    }
                } 

                if c.is_ascii_digit() {
                    let num = self.read_number();
                    return Token::Int(num.to_string());
                }

                Token::Illegal
            }
        };
        
        self.read_char();
        token
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                self.read_char();
                continue
            }
            break
        }

        self.input[start_pos..self.current_position].to_string()
   }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_letter() {
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

    fn skip_whitespaces(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
                continue
            }
            break
        }
    }

}

trait IsLetter {
    fn is_letter(&self) -> bool;
}

impl IsLetter for char {
    fn is_letter(&self) -> bool {
        ('a'..'z').contains(self) || ('A'..'Z').contains(self)
        // match *self {
        //     'a' .. 'z' | 'A' .. 'Z' => true,
        //     _ => false
        // }
    }
}
