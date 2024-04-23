use crate::token::Token;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Expression {
    Int(i32),
    Identifier(String),
    Boolean(bool),
    String(String),
    Array(Vec<Box<Expression>>),
    Index {
        idx: Box<Expression>,
        left: Box<Expression>,
    },
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
        condition: Box<AstNode>,
        consequence: Box<BlockStatement>,
        alternative: Option<Box<BlockStatement>>,
    },
    FunctionExpression {
        parameters: Vec<Token>,
        body: Box<BlockStatement>,
    },
    CallExpression {
        function: Box<Expression>,
        arguments: Vec<Box<Expression>>,
    },
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Box<AstNode>>,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Statement {
    ReturnStatement(Box<Expression>),
    LetStatement { name: Box<Expression>, value: Box<Expression> },
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum AstNode {
    Statement(Box<Statement>),
    Expression(Box<Expression>),
    Program { statements: Vec<Box<AstNode>> },
}
