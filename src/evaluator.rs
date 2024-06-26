use crate::ast::{AstNode, BlockStatement, Expression, Statement};
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::builtin::{BuiltinFn, BUILTIN_FUNCTIONS};

#[derive(Default)]
pub struct Evaluator {
    context: RefCell<HashMap<String, Object>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Return(Box<Object>),
    Builtin(BuiltinFn),
    Null,
    Function {
        parameters: Vec<Token>,
        body: Box<BlockStatement>,
        scope: HashMap<String, Object>,
    },
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            context: RefCell::new(HashMap::new()),
        }
    }

    pub fn eval(&self, node: AstNode) -> Object {
        match node {
            AstNode::Program { statements } => self.eval_program(statements),
            AstNode::Statement(statement) => self.eval_statement(*statement),
            AstNode::Expression(expression) => self.eval_expression(*expression),
        }
    }

    fn eval_expression(&self, expression: Expression) -> Object {
        match expression {
            Expression::Array(elems) => {
                let elements = self.eval_expressions(elems);
                Object::Array(elements)
            }
            Expression::Int(value) => Object::Integer(value),
            Expression::Boolean(value) => Object::Boolean(value),
            Expression::String(value) => Object::String(value),
            Expression::Prefix { operator, right } => {
                let right = self.eval(AstNode::Expression(right));
                self.eval_prefix_expression(operator, right)
            }
            Expression::Infix {
                operator,
                left,
                right,
            } => {
                let left = self.eval(AstNode::Expression(left));
                let right = self.eval(AstNode::Expression(right));
                self.eval_infix_expression(left, right, operator)
            }
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
                ..
            } => {
                let condition = self.eval(AstNode::Expression(condition));

                if condition.is_truthy() {
                    return self.eval_block_statement(consequence.statements);
                }

                if let Some(alternative_block) = alternative {
                    return self.eval_block_statement(alternative_block.statements);
                }

                Object::Null
            }
            Expression::Identifier(name) => self.eval_identifier(name),
            Expression::FunctionExpression {
                parameters, body, ..
            } => Object::Function {
                parameters,
                body,
                scope: HashMap::new(),
            },
            Expression::CallExpression {
                function,
                arguments,
                ..
            } => {
                let function = self.eval(AstNode::Expression(function));

                match function {
                    Object::Builtin(builtin_fn) => {
                        let args = self.eval_expressions(arguments);
                        builtin_fn(args)
                    }
                    Object::Function {
                        parameters,
                        body,
                        mut scope,
                    } => self.eval_function_call(parameters, arguments, *body, &mut scope),
                    obj => panic!("Wrong object, expected Object::Function, got: {:?}", obj),
                }
            }
        }
    }

    fn eval_identifier(&self, name: String) -> Object {
        if let Some(function) = BUILTIN_FUNCTIONS.get(name.as_str()) {
            return Object::Builtin(*function);
        }

        self.context
            .borrow()
            .get(&name)
            .expect("ERROR: Could not find identifer")
            .clone()
    }

    fn eval_function_call(
        &self,
        parameters: Vec<Token>,
        arguments: Vec<Expression>,
        body: BlockStatement,
        scope: &mut HashMap<String, Object>,
    ) -> Object {
        let previous_context = self.context.clone();
        let arguments = self.eval_expressions(arguments);

        // set the parameters in the given scope
        parameters
            .iter()
            .enumerate()
            .for_each(|(idx, param)| match param {
                Token::Identifier(param_name) => {
                    scope.insert(param_name.clone(), arguments.get(idx).unwrap().clone());
                }
                _ => panic!(),
            });

        self.context.borrow_mut().clear();
        scope.iter().for_each(|(key, value)| {
            self.context.borrow_mut().insert(key.clone(), value.clone());
        });

        let result = self.eval_block_statement(body.statements);

        self.context.borrow_mut().clear();
        previous_context.borrow().iter().for_each(|(key, value)| {
            self.context.borrow_mut().insert(key.clone(), value.clone());
        });

        result
    }

    fn eval_statement(&self, statement: Statement) -> Object {
        match statement {
            Statement::ReturnStatement(value) => {
                let result_object = self.eval(AstNode::Expression(value));
                Object::Return(Box::new(result_object))
            }
            Statement::LetStatement { name, value } => {
                let let_name = match *name {
                    Expression::Identifier(identifier_name) => identifier_name,
                    _ => panic!(),
                };

                let result_object = self.eval(AstNode::Expression(value));
                self.context.borrow_mut().insert(let_name, result_object.clone());
                result_object
            }
        }
    }

    fn eval_expressions(&self, expressions: Vec<Expression>) -> Vec<Object> {
        expressions
            .iter()
            .map(|expression| self.eval(AstNode::Expression(Box::new(expression.clone()))))
            .collect()
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
            Object::String(value) => value.to_string(),
            Object::Return(value) => value.inspect(),
            Object::Function { .. } => "function".to_string(),
            Object::Builtin(..) => "null".to_string(),
            Object::Null => "null".to_string(),
            Object::Array(elems) => {
                let elements_str = elems
                    .iter()
                    .map(|e| e.inspect())
                    .collect::<Vec<String>>()
                    .join(", ");

                format!("[{}]", elements_str)
            }
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
