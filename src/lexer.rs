use std::collections::HashMap;

use crate::token::Token;

use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = HashMap::from([
        ("fn", Token::Function),
        ("let", Token::Let),
        ("if", Token::If),
        ("else", Token::Else),
        ("true", Token::True),
        ("false", Token::False),
        ("return", Token::Return),
    ]);
}

#[derive(Debug)]
pub struct Lexer<'l> {
    input: &'l str,
    current_position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl<'l> Lexer<'l> {
    pub fn new(input: &'l str) -> Lexer<'_> {
        Lexer {
            input,
            current_position: 0,
            read_position: 1,
            current_char: input.chars().nth(0),
        }
    }

    fn read_string(&mut self) -> Option<&str> {
        if self.current_char? != '"' {
            return None;
        }

        self.read_char();

        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c == '"' {
                break;
            }
            self.read_char();
        }

        Some(&self.input[start_pos..self.current_position])
    }

    fn read_number(&mut self) -> &str {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                self.read_char();
                continue;
            }
            break;
        }

        &self.input[start_pos..self.current_position]
    }

    fn read_identifier(&mut self) -> &str {
        let start_pos = self.current_position;

        while let Some(c) = self.current_char {
            if c.is_letter() {
                self.read_char();
                continue;
            }
            break;
        }

        &self.input[start_pos..self.current_position]
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

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();

        if self.current_char.is_none() {
            return Some(Token::Eof);
        }

        let ch = self.current_char?;

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
            '"' => Token::String(self.read_string()?.to_string()),
            '*' => Token::Asterisk,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '/' => {
                if self.peek_char(self.read_position)? == '*' {
                    self.skip_comments();
                    return self.next();
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
                    return match KEYWORDS.get(identifier) {
                        Some(tok) => Some(tok.clone()),
                        None => Some(Token::Identifier(identifier.to_string())),
                    };
                }

                if c.is_ascii_digit() {
                    let num = self.read_number();
                    return Some(Token::Int(num.to_string()));
                }

                Token::Illegal
            }
        };

        self.read_char();
        Some(token)
    }
}
