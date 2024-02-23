use crate::{ast::AstNode, lexer::Lexer, token::Token};

#[derive(Debug)]
pub enum Expression {
    Int {
        token: Token, // Token::Int(val)
    },
    Identifier {
        token: Token, // Token::Idetifier(name)
    },
    Prefix {
        operator: Token, // Token::Bang, Token::Minus
        right: Box<Expression>,
    },
}

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
    ExpressionStatement {
        token: Token, // first expression token
        value: Expression,
    },
}

enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Parser {
    pub errors: Vec<String>,
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            next_token,
            errors: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::Eof {
            match self.current_token {
                Token::Let => {
                    if let Some(statement) = self.parse_let_statement() {
                        program.statements.push(statement);
                    }
                }
                Token::Return => {
                    if let Some(statement) = self.parse_return_statement() {
                        program.statements.push(statement);
                    }
                }
                _ => {
                    if let Some(statement) = self.parse_expression_statement() {
                        program.statements.push(statement);
                    }
                }
            }

            self.advance_tokens();
        }

        program
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        self.advance_tokens();

        if !matches!(self.current_token, Token::Identifier(_)) {
            self.report_expected_token_error(Token::Identifier("some identifier".to_string()));
            return None;
        }

        let name = self.parse_identifier()?;

        self.advance_tokens();

        if self.current_token != Token::Assign {
            self.report_expected_token_error(Token::Assign);
            return None;
        }

        self.advance_tokens();

        let value = self.parse_expression(Precedence::Lowest)?;

        self.advance_tokens();

        if self.current_token != Token::Semicolon {
            self.report_expected_token_error(Token::Semicolon);
            return None;
        }

        Some(Statement::LetStatement {
            token: Token::Let,
            name,
            value,
        })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        Some(Statement::ExpressionStatement {
            token,
            value: expression,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.advance_tokens();

        if !matches!(self.current_token, Token::Int(_)) {
            self.report_expected_token_error(Token::Int("number".to_string()));
            return None;
        }

        let expression = self.parse_expression(Precedence::Lowest)?;

        Some(Statement::ReturnStatement {
            token: Token::Return,
            value: expression,
        })
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Expression> {
        let prefix_parse_fn = self.get_prefix_parse_function(&self.current_token)?;
        prefix_parse_fn(self)
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        if let Token::Identifier(_) = &self.current_token {
            let identifier = Expression::Identifier {
                token: self.current_token.clone(),
            };
            return Some(identifier);
        }
        None
    }

    fn parse_int(&mut self) -> Option<Expression> {
        if let Token::Int(_) = &self.current_token {
            let int_expression = Expression::Int {
                token: self.current_token.clone(),
            };
            return Some(int_expression);
        }
        None
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.current_token.clone();

        self.advance_tokens();

        Some(Expression::Prefix {
            operator,
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        })
    }

    fn advance_tokens(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn report_expected_token_error(&mut self, expected_token: Token) {
        self.errors.push(format!(
            "expected token to be '{:?}' got '{:?}'",
            expected_token, self.current_token
        ));
    }

    fn get_prefix_parse_function(
        &self,
        token: &Token,
    ) -> Option<fn(&mut Parser) -> Option<Expression>> {
        match *token {
            Token::Identifier(_) => Some(Parser::parse_identifier),
            Token::Int(_) => Some(Parser::parse_int),
            Token::Bang => Some(Parser::parse_prefix_expression),
            Token::Minus => Some(Parser::parse_prefix_expression),
            _ => None,
        }
    }
}

impl AstNode for Expression {
    fn get_token_literal(&self) -> String {
        match self {
            Expression::Int { token } => match &token {
                Token::Int(num) => num.to_string(),
                _ => todo!(),
            },
            Expression::Identifier { token } => match &token {
                Token::Identifier(name) => name.to_string(),
                _ => todo!(),
            },
            Expression::Prefix { right, .. } => right.get_token_literal(),
        }
    }
}

impl AstNode for Statement {
    fn get_token_literal(&self) -> String {
        todo!()
    }
}
