use crate::{ast::AstNode, lexer::Lexer, token::Token};

#[derive(Debug, Eq, PartialEq)]
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
    Infix {
        operator: Token, // Token::Plus, Token::Minus, Token::Equals etc.
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Boolean {
        token: Token, // Token::True or Token::False
    },
    IfExpression {
        token: Token, // Token::If
        condition: Box<Expression>,
        consequence: BlockStatement,         // Statement::BlockStatement
        alternative: Option<BlockStatement>, // Statement::BlockStatement
    },
}

#[derive(Debug, Eq, PartialEq)]
pub struct BlockStatement {
    pub token: Token, // Token::LeftBrace
    pub statements: Vec<Statement>,
}

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
            self.report_expected_token_error(Token::Identifier("some identifier".to_string()), self.current_token.clone());
            return None;
        }

        let name = self.parse_identifier()?;

        self.advance_tokens();

        if self.current_token != Token::Assign {
            self.report_expected_token_error(Token::Assign, self.current_token.clone());
            return None;
        }

        self.advance_tokens();

        let value = self.parse_expression(Precedence::Lowest)?;

        self.advance_tokens();

        if self.current_token != Token::Semicolon {
            self.report_expected_token_error(Token::Semicolon, self.current_token.clone());
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

        if self.current_token == Token::Semicolon {
            self.advance_tokens();
        }

        Some(Statement::ExpressionStatement {
            token,
            value: expression,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.advance_tokens();

        if !matches!(self.current_token, Token::Int(_)) {
            self.report_expected_token_error(Token::Int("number".to_string()), self.current_token.clone());
            return None;
        }

        let expression = self.parse_expression(Precedence::Lowest)?;

        Some(Statement::ReturnStatement {
            token: Token::Return,
            value: expression,
        })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix_parse_fn = self.current_token.prefix_parse_fn()?;
        let left_expression = prefix_parse_fn(self)?;

        let is_next_token_precedence_higher = precedence < self.next_token.precedence();

        while !self.expect_current_token(Token::Semicolon) && is_next_token_precedence_higher {
            if let Some(infix_parse_fn) = self.next_token.get_infix_parse_fn() {
                self.advance_tokens();
                return infix_parse_fn(self, left_expression);
            }
        }

        Some(left_expression)
    }

    fn expect_current_token(&mut self, token: Token) -> bool {
        if self.current_token == token {
            self.advance_tokens();
            return true;
        }
        false
    }

    fn expect_next_token(&mut self, token: Token) -> bool {
        if self.next_token == token {
            self.advance_tokens();
            return true;
        }
        false
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

    fn parse_infix_expression(&mut self, left_expression: Expression) -> Option<Expression> {
        let operator = self.current_token.clone();
        let precedence = self.current_token.precedence();

        self.advance_tokens();

        let right_expression = self.parse_expression(precedence)?;

        Some(Expression::Infix {
            operator,
            left: Box::new(left_expression),
            right: Box::new(right_expression),
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.advance_tokens();

        let expression = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_next_token(Token::RightParentesis) {
            return None;
        }

        Some(expression)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.current_token.clone();

        self.advance_tokens();

        Some(Expression::Prefix {
            operator,
            right: Box::new(self.parse_expression(Precedence::Prefix)?),
        })
    }

    fn parse_boolean_expression(&mut self) -> Option<Expression> {
        Some(Expression::Boolean {
            token: self.current_token.clone(),
        })
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_next_token(Token::LeftParentesis) {
            self.report_expected_token_error(Token::LeftParentesis, self.next_token.clone());
            return None;
        }
        self.advance_tokens();

        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_next_token(Token::RightParentesis) {
            self.report_expected_token_error(Token::LeftParentesis, self.next_token.clone());
            return None;
        }
        self.advance_tokens();

        let consequence = self.parse_block_statement()?;

        Some(Expression::IfExpression {
            token: Token::If,
            condition: Box::new(condition),
            consequence,
            alternative: None,
        })
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        if !self.expect_current_token(Token::LeftBrace) {
            self.report_expected_token_error(Token::LeftBrace, self.current_token.clone());
            return None;
        }

        let mut statements = Vec::new();

        while self.current_token != Token::RightBrace && self.current_token != Token::Eof {
            match self.current_token {
                Token::Let => {
                    if let Some(statement) = self.parse_let_statement() {
                        statements.push(statement);
                    }
                }
                Token::Return => {
                    if let Some(statement) = self.parse_return_statement() {
                        statements.push(statement);
                    }
                }
                _ => {
                    if let Some(statement) = self.parse_expression_statement() {
                        statements.push(statement);
                    }
                }
            }

            self.advance_tokens();
        }

        Some(BlockStatement {
            token: Token::LeftBrace,
            statements,
        })
    }

    fn advance_tokens(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn report_expected_token_error(&mut self, expected_token: Token, actual_token: Token) {
        self.errors.push(format!(
            "expected token to be '{:?}' got '{:?}'",
            expected_token, actual_token
        ));
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
            Expression::Infix { .. } => todo!(),
            Expression::Boolean { .. } => todo!(),
            Expression::IfExpression { .. } => todo!(),
        }
    }
}

impl AstNode for Statement {
    fn get_token_literal(&self) -> String {
        todo!()
    }
}

impl Token {
    fn precedence(&self) -> Precedence {
        match self {
            Token::Equals | Token::NotEquals => Precedence::Equals,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            Token::LessThan | Token::GreaterThan => Precedence::LessGreater,
            _ => Precedence::Lowest,
        }
    }

    fn prefix_parse_fn(&self) -> Option<fn(&mut Parser) -> Option<Expression>> {
        match self {
            Token::Identifier(_) => Some(Parser::parse_identifier),
            Token::Int(_) => Some(Parser::parse_int),
            Token::Bang => Some(Parser::parse_prefix_expression),
            Token::LeftParentesis => Some(Parser::parse_grouped_expression),
            Token::Minus => Some(Parser::parse_prefix_expression),
            Token::True | Token::False => Some(Parser::parse_boolean_expression),
            Token::If => Some(Parser::parse_if_expression),
            _ => None,
        }
    }

    fn get_infix_parse_fn(&self) -> Option<fn(&mut Parser, Expression) -> Option<Expression>> {
        match self {
            Token::Plus => Some(Parser::parse_infix_expression),
            Token::Minus => Some(Parser::parse_infix_expression),
            Token::Slash => Some(Parser::parse_infix_expression),
            Token::Asterisk => Some(Parser::parse_infix_expression),
            Token::GreaterThan => Some(Parser::parse_infix_expression),
            Token::LessThan => Some(Parser::parse_infix_expression),
            Token::Equals => Some(Parser::parse_infix_expression),
            Token::NotEquals => Some(Parser::parse_infix_expression),
            _ => None,
        }
    }
}
