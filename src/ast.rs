use crate::token::Token;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Expression {
    Int(i32),
    Identifier(String),
    Boolean(bool),
    String(String),
    Prefix {
        operator: Token, // Token::Bang, Token::Minus
        right: Box<Expression>,
    },
    Infix {
        operator: Token, // Token::Plus, Token::Minus, Token::Equals etc.
        left: Box<Expression>,
        right: Box<Expression>,
    },
    IfExpression {
        condition: Box<AstNode>,
        consequence: BlockStatement,         // Statement::BlockStatement
        alternative: Option<BlockStatement>, // Statement::BlockStatement
    },
    FunctionExpression {
        parameters: Vec<Token>, // Vec<Token::Identifier>
        body: BlockStatement,
    },
    CallExpression {
        function: Box<Expression>, // Expression::FunctionExpression or Expression::Identifier
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct BlockStatement {
    pub token: Token, // Token::LeftBrace
    pub statements: Vec<AstNode>,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Statement {
    ReturnStatement(Expression),
    LetStatement {
        name: Expression, // Expression::Identifer
        value: Expression,
    },
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum AstNode {
    Statement(Statement),
    Expression(Expression),
    Program { statements: Vec<AstNode> },
}
