
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum TokenType {
    Illegal,
    Equals,
    NotEquals,
    Function,
    Eof,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    GreaterThan,
    LessThan,
    Int,
    Identifier,
    LeftParentesis,
    Let,
    True,
    False,
    Return,
    If,
    Else,
    Comma,
    Semicolon,
    RightParentesis,
    LeftBrace,
    RightBrace,
}
