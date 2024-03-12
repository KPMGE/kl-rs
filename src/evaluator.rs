use crate::ast::{AstNode, Expression, Statement};
use crate::token::Token;

#[derive(Default)]
pub struct Evaluator {}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Return(Box<Object>),
    Null,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator::default()
    }

    pub fn eval(&self, node: AstNode) -> Object {
        match node {
            AstNode::Program { statements } => self.eval_program(statements),
            AstNode::Statement(statement) => match statement {
                Statement::ReturnStatement { value, .. } => {
                    let result_object = self.eval(AstNode::Expression(value));
                    Object::Return(Box::new(result_object))
                }
                Statement::LetStatement { .. } => todo!(),
            },
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
                Expression::IfExpression {
                    condition,
                    consequence,
                    alternative,
                    ..
                } => {
                    let condition = self.eval(*condition);

                    if condition.is_truthy() {
                        return self.eval_block_statement(consequence.statements);
                    }

                    if let Some(alternative_block) = alternative {
                        return self.eval_block_statement(alternative_block.statements);
                    }

                    Object::Null
                }
                _ => todo!(),
            },
        }
    }

    fn eval_program(&self, statements: Vec<AstNode>) -> Object {
        let mut result = Object::Null;

        for statement in statements {
            result = self.eval(statement.clone());
            if let Object::Return(return_value) = result {
                return Object::Return(return_value);
            }
        }

        result
    }

    fn eval_block_statement(&self, statements: Vec<AstNode>) -> Object {
        let mut result = Object::Null;

        for statement in statements {
            result = self.eval(statement);
            if let Object::Return(return_value) = result {
                return Object::Return(return_value);
            }
        }

        result
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
            _ => Object::Null,
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
            Object::Return(value) => value.inspect(),
            Object::Null => "null".to_string(),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Object::Integer(0) => false,
            Object::Boolean(true) | Object::Integer(..) => true,
            _ => false,
        }
    }
}
