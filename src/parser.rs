use std::num::ParseIntError;

use thiserror::Error;

use crate::{
    ast::{AstNode, BlockStatement, Expression, Statement},
    lexer::Lexer,
    token::Token,
};

#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("missing token")]
    MissingToken,
    #[error("expected token {expected:?}, got {actual:?}")]
    UnexpectedToken { expected: Token, actual: Token },
    #[error("no parsing function for `{0:?}`")]
    NoParsingFunction(Token),
    #[error("error while parsing integer")]
    IntErr(#[from] ParseIntError),
    #[error("Invalid expression: `{0}`")]
    InvalidExpression(String),
}

#[derive(Debug)]
pub struct Parser<'p> {
    lexer: Lexer<'p>,
    current_token: Option<Token>,
    next_token: Option<Token>,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: Lexer<'p>) -> Self {
        let mut p = Parser {
            lexer,
            current_token: None,
            next_token: None,
        };
        p.advance_tokens();
        p.advance_tokens();
        p
    }

    pub fn parse_program(&mut self) -> Result<AstNode, ParserError> {
        let mut program = AstNode::Program {
            statements: Vec::new(),
        };

        while let Some(token) = &self.current_token {
            if *token == Token::Eof {
                break;
            }

            match program {
                AstNode::Program { ref mut statements } => match token {
                    Token::Let => {
                        let statement = self.parse_let_statement()?;
                        statements.push(statement);
                    }
                    Token::Return => {
                        let statement = self.parse_return_statement()?;
                        statements.push(statement);
                    }
                    _ => {
                        let statement = self.parse_expression_statement()?;
                        statements.push(statement);
                    }
                },
                _ => panic!("Expected AstNode::Program"),
            }

            self.advance_tokens();
        }

        Ok(program)
    }

    fn get_current_token(&mut self) -> Result<Token, ParserError> {
        self.current_token.clone().ok_or(ParserError::MissingToken)
    }

    fn get_next_token(&mut self) -> Result<Token, ParserError> {
        self.next_token.clone().ok_or(ParserError::MissingToken)
    }

    fn parse_let_statement(&mut self) -> Result<AstNode, ParserError> {
        self.advance_tokens();

        let token = self.get_current_token()?;
        match token {
            Token::Identifier(..) => {
                let name = Box::new(self.parse_identifier()?);

                self.advance_tokens();

                let current_token = self.get_current_token()?;
                if current_token != Token::Assign {
                    return Err(ParserError::UnexpectedToken {
                        expected: Token::Assign,
                        actual: current_token,
                    });
                }

                self.advance_tokens();

                let value = Box::new(self.parse_expression(Precedence::Lowest)?);

                self.advance_tokens();

                let current_token = self.get_current_token()?;
                if current_token != Token::Semicolon {
                    return Err(ParserError::UnexpectedToken {
                        expected: Token::Semicolon,
                        actual: current_token,
                    });
                }

                Ok(AstNode::Statement(Box::new(Statement::LetStatement {
                    name,
                    value,
                })))
            }
            _ => Err(ParserError::UnexpectedToken {
                expected: Token::Identifier("some identifier".to_string()),
                actual: token,
            }),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<AstNode, ParserError> {
        let expression = Box::new(self.parse_expression(Precedence::Lowest)?);

        if self.get_current_token()? == Token::Semicolon {
            self.advance_tokens();
        }

        Ok(AstNode::Expression(expression))
    }

    fn parse_return_statement(&mut self) -> Result<AstNode, ParserError> {
        self.advance_tokens();

        let current_token = self.get_current_token()?;
        if !matches!(current_token, Token::Int(_)) {
            return Err(ParserError::UnexpectedToken {
                expected: Token::Int("number".to_string()),
                actual: current_token,
            });
        }

        let expression = Box::new(self.parse_expression(Precedence::Lowest)?);

        Ok(AstNode::Statement(Box::new(Statement::ReturnStatement(
            expression,
        ))))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
        let prefix_parse_fn = self.get_current_token()?.prefix_parse_fn()?;
        let left_expression = prefix_parse_fn(self)?;

        let is_next_token_precedence_higher = precedence < self.get_next_token()?.precedence();

        while self.get_current_token()? != Token::Semicolon && is_next_token_precedence_higher {
            // self.advance_tokens();
            let infix_parse_fn = self.get_next_token()?.infix_parse_fn()?;
            self.advance_tokens();
            return infix_parse_fn(self, left_expression);
        }

        Ok(left_expression)
    }

    fn parse_string(&mut self) -> Result<Expression, ParserError> {
        let current_token = self.get_current_token()?;

        if let Token::String(s) = current_token {
            return Ok(Expression::String(s.clone()));
        }

        Err(ParserError::UnexpectedToken {
            expected: Token::String("some string".to_string()),
            actual: current_token,
        })
    }

    fn parse_identifier(&mut self) -> Result<Expression, ParserError> {
        let current_token = self.get_current_token()?;

        if let Token::Identifier(name) = &self.get_current_token()? {
            return Ok(Expression::Identifier(name.clone()));
        }

        Err(ParserError::UnexpectedToken {
            expected: Token::String("some identifier".to_string()),
            actual: current_token,
        })
    }

    fn parse_int(&mut self) -> Result<Expression, ParserError> {
        let current_token = self.get_current_token()?;

        if let Token::Int(num_str) = current_token {
            return num_str
                .parse::<i32>()
                .map(|num| Expression::Int(num))
                .map_err(|e| ParserError::IntErr(e));
        }

        Err(ParserError::UnexpectedToken {
            expected: Token::Int("some number".to_string()),
            actual: current_token,
        })
    }

    fn parse_array_expression(&mut self) -> Result<Expression, ParserError> {
        if self.get_current_token()? != Token::LeftBracket {
            return Err(ParserError::UnexpectedToken {
                expected: Token::LeftBracket,
                actual: self.get_current_token()?,
            });
        }

        self.advance_tokens();

        let mut elements = Vec::new();

        if let Some(Token::RightBracket) = self.current_token {
            return Ok(Expression::Array(elements));
        }

        let exp = self.parse_expression(Precedence::Lowest)?;
        elements.push(exp);

        while let Some(Token::Comma) = self.next_token {
            self.advance_tokens();
            self.advance_tokens();

            let exp = self.parse_expression(Precedence::Lowest)?;
            elements.push(exp);
        }

        self.advance_tokens();

        if self.get_current_token()? != Token::RightBracket {
            return Err(ParserError::UnexpectedToken {
                expected: Token::RightBracket,
                actual: self.get_current_token()?,
            });
        }
        self.advance_tokens();

        Ok(Expression::Array(elements))
    }

    fn parse_infix_expression(
        &mut self,
        left_expression: Expression,
    ) -> Result<Expression, ParserError> {
        let operator = self.get_current_token()?;
        let precedence = self.get_next_token()?.precedence();

        self.advance_tokens();

        let right_expression = self.parse_expression(precedence)?;

        Ok(Expression::Infix {
            operator,
            left: Box::new(left_expression),
            right: Box::new(right_expression),
        })
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.advance_tokens();

        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.get_next_token()? != Token::RightParentesis {
            return Err(ParserError::UnexpectedToken {
                expected: Token::RightParentesis,
                actual: self.get_next_token()?,
            });
        }

        self.advance_tokens();

        Ok(expression)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = self.get_current_token()?;

        self.advance_tokens();

        Ok(Expression::Prefix {
            operator,
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        })
    }

    fn parse_boolean_expression(&mut self) -> Result<Expression, ParserError> {
        let value = match self.get_current_token()? {
            Token::True => true,
            Token::False => false,
            _ => {
                return Err(ParserError::InvalidExpression(
                    "boolean expressions should have either 'true' or 'false'".to_string(),
                ))
            }
        };

        Ok(Expression::Boolean(value))
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        if self.get_next_token()? != Token::LeftParentesis {
            return Err(ParserError::UnexpectedToken {
                expected: Token::LeftParentesis,
                actual: self.get_next_token()?,
            });
        }

        self.advance_tokens();
        self.advance_tokens();

        let condition = Box::new(self.parse_expression(Precedence::Lowest)?);

        if self.get_next_token()? != Token::RightParentesis {
            return Err(ParserError::UnexpectedToken {
                expected: Token::RightParentesis,
                actual: self.get_next_token()?,
            });
        }
        self.advance_tokens();
        self.advance_tokens();

        let consequence = Box::new(self.parse_block_statement()?);

        self.advance_tokens();

        let alternative = match self.get_current_token()? {
            Token::Else => {
                self.advance_tokens();
                Some(Box::new(self.parse_block_statement()?))
            }
            _ => None,
        };

        Ok(Expression::IfExpression {
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_function_expression(&mut self) -> Result<Expression, ParserError> {
        self.advance_tokens();

        if self.get_current_token()? != Token::LeftParentesis {
            self.advance_tokens();
            return Err(ParserError::UnexpectedToken {
                expected: Token::LeftParentesis,
                actual: self.get_current_token()?,
            });
        }

        let parameters = self.parse_function_parameters()?;
        let body = Box::new(self.parse_block_statement()?);

        Ok(Expression::FunctionExpression { parameters, body })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut parameters = Vec::new();

        if self.get_next_token()? == Token::RightParentesis {
            self.advance_tokens();
            self.advance_tokens();
            return Ok(parameters);
        }

        self.advance_tokens();

        let identifier = self.get_current_token()?;
        parameters.push(identifier);

        while let Some(Token::Comma) = self.next_token {
            self.advance_tokens();
            self.advance_tokens();
            let identifier = self.get_current_token()?;
            parameters.push(identifier);
        }

        if self.get_next_token()? != Token::RightParentesis {
            return Err(ParserError::UnexpectedToken {
                expected: Token::RightParentesis,
                actual: self.get_next_token()?,
            });
        }

        self.advance_tokens();
        self.advance_tokens();

        Ok(parameters)
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
        if self.get_current_token()? != Token::LeftBrace {
            return Err(ParserError::UnexpectedToken {
                expected: Token::LeftBrace,
                actual: self.get_current_token()?,
            });
        }

        self.advance_tokens();

        let mut statements = Vec::new();

        while let Some(token) = &self.current_token {
            if *token == Token::RightBrace || *token == Token::Eof {
                break;
            }

            match token {
                Token::Let => {
                    if let Ok(statement) = self.parse_let_statement() {
                        statements.push(statement);
                    }
                }
                Token::Return => {
                    if let Ok(statement) = self.parse_return_statement() {
                        statements.push(statement);
                    }
                }
                _ => {
                    if let Ok(statement) = self.parse_expression_statement() {
                        statements.push(statement);
                    }
                }
            }

            self.advance_tokens();
        }

        Ok(BlockStatement { statements })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParserError> {
        self.advance_tokens();
        let arguments = self.parse_call_expression_arguments()?;

        Ok(Expression::CallExpression {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_call_expression_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut arguments = Vec::new();

        if self.get_current_token()? == Token::RightParentesis {
            self.advance_tokens();
            return Ok(arguments);
        }

        let expression = self.parse_expression(Precedence::Lowest)?;
        arguments.push(expression);

        while let Some(Token::Comma) = self.next_token {
            self.advance_tokens();
            self.advance_tokens();
            let expression = self.parse_expression(Precedence::Lowest)?;
            arguments.push(expression);
        }

        if self.get_next_token()? != Token::RightParentesis {
            return Err(ParserError::UnexpectedToken {
                expected: Token::RightParentesis,
                actual: self.get_next_token()?,
            });
        }
        self.advance_tokens();

        Ok(arguments)
    }

    fn advance_tokens(&mut self) {
        self.current_token = self.next_token.take();
        self.next_token = self.lexer.next();
    }
}

type InfixParserFn<'p> = fn(&mut Parser<'p>, Expression) -> Result<Expression, ParserError>;
type PrefixParseFn<'p> = fn(&mut Parser<'p>) -> Result<Expression, ParserError>;

impl Token {
    fn precedence(&self) -> Precedence {
        match self {
            Token::Equals | Token::NotEquals => Precedence::Equals,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::LeftParentesis => Precedence::Call,
            Token::Asterisk | Token::Slash => Precedence::Product,
            Token::LessThan | Token::GreaterThan => Precedence::LessGreater,
            _ => Precedence::Lowest,
        }
    }

    fn prefix_parse_fn<'p>(&self) -> Result<PrefixParseFn<'p>, ParserError> {
        match self {
            Token::String(_) => Ok(Parser::parse_string),
            Token::Identifier(_) => Ok(Parser::parse_identifier),
            Token::Int(_) => Ok(Parser::parse_int),
            Token::Bang => Ok(Parser::parse_prefix_expression),
            Token::LeftParentesis => Ok(Parser::parse_grouped_expression),
            Token::LeftBracket => Ok(Parser::parse_array_expression),
            Token::Minus => Ok(Parser::parse_prefix_expression),
            Token::True | Token::False => Ok(Parser::parse_boolean_expression),
            Token::If => Ok(Parser::parse_if_expression),
            Token::Function => Ok(Parser::parse_function_expression),
            _ => Err(ParserError::NoParsingFunction(self.clone())),
        }
    }

    fn infix_parse_fn<'p>(&self) -> Result<InfixParserFn<'p>, ParserError> {
        match self {
            Token::Plus => Ok(Parser::parse_infix_expression),
            Token::Minus => Ok(Parser::parse_infix_expression),
            Token::Slash => Ok(Parser::parse_infix_expression),
            Token::Asterisk => Ok(Parser::parse_infix_expression),
            Token::GreaterThan => Ok(Parser::parse_infix_expression),
            Token::LessThan => Ok(Parser::parse_infix_expression),
            Token::Equals => Ok(Parser::parse_infix_expression),
            Token::NotEquals => Ok(Parser::parse_infix_expression),
            Token::LeftParentesis => Ok(Parser::parse_call_expression),
            _ => Err(ParserError::NoParsingFunction(self.clone())),
        }
    }
}
