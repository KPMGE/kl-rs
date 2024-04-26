use crate::token::Token;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Expression {
    Int(i32),
    Identifier(String),
    Boolean(bool),
    String(String),
    Array(Vec<Expression>),
    Prefix {
        operator: Token,
        right: Box<Expression>,
    },
    Infix {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    IfExpression {
        condition: Box<Expression>,
        consequence: Box<BlockStatement>,
        alternative: Option<Box<BlockStatement>>,
    },
    FunctionExpression {
        parameters: Vec<Token>,
        body: Box<BlockStatement>,
    },
    CallExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<AstNode>,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Statement {
    ReturnStatement(Box<Expression>),
    LetStatement {
        name: Box<Expression>,
        value: Box<Expression>,
    },
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum AstNode {
    Statement(Box<Statement>),
    Expression(Box<Expression>),
    Program { statements: Vec<AstNode> },
}
