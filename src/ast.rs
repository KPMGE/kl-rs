use crate::token::Token;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Expression {
    Int(i32),
    Identifier(String),
    Boolean(bool),
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
        token: Token, // Token::If
        condition: Box<AstNode>,
        consequence: BlockStatement,         // Statement::BlockStatement
        alternative: Option<BlockStatement>, // Statement::BlockStatement
    },
    FunctionExpression {
        token: Token,           // Token::Fn
        parameters: Vec<Token>, // Vec<Token::Identifier>
        body: BlockStatement,
    },
    CallExpression {
        token: Token,              // Token::LeftParentesis
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
    LetStatement {
        token: Token,     // Token::Let
        name: Expression, // Expression::Identifer
        value: Expression,
    },
    ReturnStatement {
        token: Token, // Token::Return
        value: Expression,
    },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum AstNode {
    Statement(Statement),
    Expression(Expression),
    Program { statements: Vec<AstNode> },
}
