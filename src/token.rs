#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Assign,
    Plus,
    Int(String),
    Identifier(String),
    LeftParentesis,
    Comma,
    RightParentesis,
    LeftBrace,
    RightBrace,
}
