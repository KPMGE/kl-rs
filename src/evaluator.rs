
use crate::parser::{Statement, Expression};
use crate::token::Token;
pub struct Evaluator {}

pub enum Object {
    Integer(i32),
    Boolean(bool)
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&self, node: Statement) -> Object {
        match node {
            Statement::ExpressionStatement { value, .. } => {
                if let Expression::Int { token: Token::Int(value) } = value {
                    Object::Integer(value.parse::<i32>().expect("Could not parse integer"))
                } else {
                    todo!()
                }
            },
            _ => todo!()
        }
    }
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => format!("{value}"),
            Object::Boolean(value) => format!("{value}")
        }
    }
}
