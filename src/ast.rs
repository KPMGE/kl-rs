use crate::token::Token;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Expression {
    Int(i32),
    Identifier(String),
    Boolean(bool),
    String(String),
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
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    FunctionExpression {
        parameters: Vec<Token>,
        body: BlockStatement,
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
    ReturnStatement(Expression),
    LetStatement {
        name: Expression,
        value: Expression,
    },
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum AstNode {
    Statement(Statement),
    Expression(Expression),
    Program { statements: Vec<AstNode> },
}
