use crate::ast::{AstNode, BlockStatement, Expression, Statement};
use crate::token::Token;
use std::collections::HashMap;

#[derive(Default)]
pub struct Evaluator {
    context: HashMap<String, Object>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    String(String),
    Return(Box<Object>),
    Function {
        parameters: Vec<Token>,
        body: BlockStatement,
        scope: HashMap<String, Object>,
    },
    Null,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            context: HashMap::new(),
        }
    }

    pub fn eval(&mut self, node: AstNode) -> Object {
        match node {
            AstNode::Program { statements } => self.eval_program(statements),
            AstNode::Statement(statement) => self.eval_statement(statement),
            AstNode::Expression(expression) => self.eval_expression(expression),
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Object {
        match expression {
            Expression::Int(value) => Object::Integer(value),
            Expression::Boolean(value) => Object::Boolean(value),
            Expression::String(value) => Object::String(value),
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
            Expression::Identifier(let_name) => self
                .context
                .get(&let_name)
                .expect("ERROR: Could not find identifer")
                .clone(),
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
                let function = self.eval(AstNode::Expression(*function));

                match function {
                    Object::Function {
                        parameters,
                        body,
                        mut scope,
                    } => self.eval_function_call(parameters, arguments, body, &mut scope),
                    obj => panic!("Wrong object, expected Object::Function, got: {:?}", obj),
                }
            }
        }
    }

    fn eval_function_call(
        &mut self,
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

        self.context = scope.clone();
        let result = self.eval_block_statement(body.statements);
        self.context = previous_context;

        result
    }

    fn eval_statement(&mut self, statement: Statement) -> Object {
        match statement {
            Statement::ReturnStatement(value) => {
                let result_object = self.eval(AstNode::Expression(value));
                Object::Return(Box::new(result_object))
            }
            Statement::LetStatement { name, value } => {
                let let_name = match name {
                    Expression::Identifier(identifier_name) => identifier_name,
                    _ => panic!(),
                };

                let result_object = self.eval(AstNode::Expression(value));
                self.context.insert(let_name.clone(), result_object.clone());
                result_object
            }
        }
    }

    fn eval_expressions(&mut self, expressions: Vec<Expression>) -> Vec<Object> {
        expressions
            .iter()
            .map(|expression| self.eval(AstNode::Expression(expression.clone())))
            .collect()
    }

    fn eval_program(&mut self, statements: Vec<AstNode>) -> Object {
        let mut result = Object::Null;

        for statement in statements {
            result = self.eval(statement.clone());
            if let Object::Return(return_value) = result {
                return Object::Return(return_value);
            }
        }

        result
    }

    fn eval_block_statement(&mut self, statements: Vec<AstNode>) -> Object {
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
