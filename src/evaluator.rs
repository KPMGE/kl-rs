use crate::ast::{AstNode, Expression};
use crate::token::Token;
pub struct Evaluator {}

pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&self, node: AstNode) -> Object {
        match node {
            AstNode::Expression(expression) => match expression {
                Expression::Int {
                    token: Token::Int(value),
                } => Object::Integer(value.parse::<i32>().expect("Could not parse integer")),
                Expression::Boolean { value, .. } => Object::Boolean(value),
                _ => todo!(),
            },
            AstNode::Statement(_) => todo!(),
        }
    }
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => format!("{value}"),
            Object::Boolean(value) => format!("{value}"),
            Object::Null => format!("null"),
        }
    }
}
