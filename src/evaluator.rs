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
                Expression::Prefix { operator, right } => {
                    let right = self.eval(AstNode::Expression(*right));
                    self.eval_prefix_expression(operator, right)
                }
                Expression::Infix {
                    operator,
                    left,
                    right,
                } => {
                    let left = self.eval(AstNode::Expression(*left));
                    let right = self.eval(AstNode::Expression(*right));
                    self.eval_infix_expression(left, right, operator)
                }
                _ => todo!(),
            },
            AstNode::Statement(_) => todo!(),
        }
    }

    fn eval_prefix_expression(&self, operator: Token, right: Object) -> Object {
        match operator {
            Token::Bang => self.eval_bang_expression(right),
            Token::Minus => self.eval_minus_prefix_expression(right),
            _ => Object::Null,
        }
    }

    fn eval_bang_expression(&self, right: Object) -> Object {
        match right {
            Object::Boolean(value) => Object::Boolean(!value),
            Object::Null => Object::Boolean(true),
            _ => Object::Boolean(false),
        }
    }

    fn eval_minus_prefix_expression(&self, right: Object) -> Object {
        match right {
            Object::Integer(value) => Object::Integer(-value),
            _ => Object::Null,
        }
    }

    fn eval_infix_expression(&self, left: Object, right: Object, operator: Token) -> Object {
        let left_int = match left {
            Object::Integer(num) => num,
            _ => return Object::Null,
        };
        let right_int = match right {
            Object::Integer(num) => num,
            _ => return Object::Null,
        };

        match operator {
            Token::Plus => Object::Integer(left_int + right_int),
            Token::Minus => Object::Integer(left_int - right_int),
            Token::Asterisk => Object::Integer(left_int * right_int),
            Token::Slash => Object::Integer(left_int / right_int),
            Token::Equals => Object::Boolean(left_int == right_int),
            Token::NotEquals => Object::Boolean(left_int != right_int),
            Token::LessThan => Object::Boolean(left_int < right_int),
            Token::GreaterThan => Object::Boolean(left_int > right_int),
            _ => Object::Null,
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
