use std::collections::HashMap;

use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
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
        let keywords_map = HashMap::from([
            ("fn", TokenType::Function),
            ("let", TokenType::Let),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("true", TokenType::True),
            ("false", TokenType::False),
            ("return", TokenType::Return),
        ]);

        self.skip_whitespaces();

        if self.current_char.is_none() {
            return Token {
                token_type: TokenType::Eof,
                literal: None,
            };
        }

        let ch = self.current_char.unwrap();

        let token_type = match ch {
            '(' => TokenType::RightParentesis,
            ')' => TokenType::LeftParentesis,
            '{' => TokenType::RightBrace,
            '}' => TokenType::LeftBrace,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Asterisk,
            '<' => TokenType::LessThan,
            '>' => TokenType::GreaterThan,
            ',' => TokenType::Comma,
            '/' => TokenType::Slash,
            ';' => TokenType::Semicolon,
            '=' => match self.peek_char(self.read_position) {
                Some('=') => {
                    self.read_char();
                    TokenType::Equals
                }
                _ => TokenType::Assign,
            },
            '!' => match self.peek_char(self.read_position) {
                Some('=') => {
                    self.read_char();
                    TokenType::NotEquals
                }
                _ => TokenType::Bang,
            },
            c => {
                if c.is_letter() {
                    let identifier = self.read_identifier();
                    return match keywords_map.get(&identifier.as_str()) {
                        Some(tok) => Token {
                            token_type: tok.clone(),
                            literal: Some(identifier),
                        },
                        None => Token {
                            token_type: TokenType::Identifier,
                            literal: Some(identifier),
                        },
                    };
                }

                if c.is_ascii_digit() {
                    let num = self.read_number();
                    return Token {
                        token_type: TokenType::Int,
                        literal: Some(num.to_string()),
                    };
                }

                TokenType::Illegal
            }
        };

        self.read_char();
        Token {
            token_type,
            literal: None,
        }
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                self.read_char();
                continue;
            }
            break;
        }

        self.input[start_pos..self.current_position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_letter() {
                self.read_char();
                continue;
            }
            break;
        }

        let identifier = &self.input[start_pos..self.current_position];
        identifier.to_string()
    }

    fn peek_char(&self, pos: usize) -> Option<char> {
        self.input.chars().nth(pos)
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
                continue;
            }
            break;
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
