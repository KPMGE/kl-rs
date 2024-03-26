use std::collections::HashMap;

use crate::token::Token;

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
            ("fn", Token::Function),
            ("let", Token::Let),
            ("if", Token::If),
            ("else", Token::Else),
            ("true", Token::True),
            ("false", Token::False),
            ("return", Token::Return),
        ]);

        self.skip_whitespaces();

        if self.current_char.is_none() {
            return Token::Eof;
        }

        let ch = self.current_char.unwrap();

        let token = match ch {
            '(' => Token::LeftParentesis,
            ')' => Token::RightParentesis,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '"' => Token::String(self.read_string()),
            '*' => Token::Asterisk,
            '/' => {
                if self.peek_char(self.read_position).unwrap() == '*' {
                    self.skip_comments();
                    return self.next_token();
                }
                Token::Slash
            }
            '=' => match self.peek_char(self.read_position) {
                Some('=') => {
                    self.read_char();
                    Token::Equals
                }
                _ => Token::Assign,
            },
            '!' => match self.peek_char(self.read_position) {
                Some('=') => {
                    self.read_char();
                    Token::NotEquals
                }
                _ => Token::Bang,
            },
            c => {
                if c.is_letter() {
                    let identifier = self.read_identifier();
                    return match keywords_map.get(&identifier.as_str()) {
                        Some(tok) => tok.clone(),
                        None => Token::Identifier(identifier),
                    };
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

    fn read_string(&mut self) -> String {
        if self.current_char.unwrap() != '"' {
            panic!(
                "Unexpected start of string, expected: '\"', got: {:?}",
                self.current_char
            );
        }

        let mut str = String::new();

        self.read_char();

        while let Some(c) = self.current_char {
            if c == '"' {
                break;
            }
            str.push(c);
            self.read_char();
        }

        self.read_char();

        str
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

    fn skip_comments(&mut self) {
        while let (Some(ch), Some(next_ch)) =
            (self.current_char, self.peek_char(self.read_position))
        {
            if ch == '*' && next_ch == '/' {
                break;
            }
            self.read_char();
        }
        self.read_char();
        self.read_char();
    }
}

trait IsLetter {
    fn is_letter(&self) -> bool;
}

impl IsLetter for char {
    fn is_letter(&self) -> bool {
        self.is_ascii_lowercase() || self.is_ascii_uppercase()
    }
}
