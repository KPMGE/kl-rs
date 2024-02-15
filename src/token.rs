#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Token {
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
    Int(String),
    Identifier(String),
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
