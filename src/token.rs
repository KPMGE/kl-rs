#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Function,
    Eof,
    Assign,
    Plus,
    Int(String),
    Identifier(String),
    LeftParentesis,
    Let,
    Comma,
    RightParentesis,
    LeftBrace,
    RightBrace,
}
