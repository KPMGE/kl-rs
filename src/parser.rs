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

#[derive(Debug, Clone)]
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

    pub fn parse_program(&mut self) -> AstNode {
        let mut program = AstNode::Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::Eof {
            match program {
                AstNode::Program { ref mut statements } => match self.current_token {
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
                },
                _ => panic!("Expected AstNode::Program"),
            }

            self.advance_tokens();
        }

        program
    }

    fn parse_let_statement(&mut self) -> Option<AstNode> {
        self.advance_tokens();

        if !matches!(self.current_token, Token::Identifier(_)) {
            self.report_expected_token_error(
                Token::Identifier("some identifier".to_string()),
                self.current_token.clone(),
            );
            return None;
        }

        let name = Box::new(self.parse_identifier()?);

        self.advance_tokens();

        if self.current_token != Token::Assign {
            self.report_expected_token_error(Token::Assign, self.current_token.clone());
            return None;
        }

        self.advance_tokens();

        let value = Box::new(self.parse_expression(Precedence::Lowest)?);

        self.advance_tokens();

        if self.current_token != Token::Semicolon {
            self.report_expected_token_error(Token::Semicolon, self.current_token.clone());
            return None;
        }

        Some(AstNode::Statement(Statement::LetStatement { name, value }))
    }

    fn parse_expression_statement(&mut self) -> Option<AstNode> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.current_token == Token::Semicolon {
            self.advance_tokens();
        }

        Some(AstNode::Expression(expression))
    }

    fn parse_return_statement(&mut self) -> Option<AstNode> {
        self.advance_tokens();

        if !matches!(self.current_token, Token::Int(_)) {
            self.report_expected_token_error(
                Token::Int("number".to_string()),
                self.current_token.clone(),
            );
            return None;
        }

        let expression = Box::new(self.parse_expression(Precedence::Lowest)?);

        Some(AstNode::Statement(Statement::ReturnStatement(expression)))
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

    fn parse_string(&mut self) -> Option<Expression> {
        if let Token::String(s) = &self.current_token {
            return Some(Expression::String(s.clone()));
        }
        None
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        if let Token::Identifier(name) = &self.current_token {
            return Some(Expression::Identifier(name.clone()));
        }
        None
    }

    fn parse_int(&mut self) -> Option<Expression> {
        if let Token::Int(num_str) = &self.current_token {
            let num = match num_str.parse::<i32>() {
                Ok(num) => Some(num),
                Err(_) => {
                    self.report_error(&format!("error parsing integer: {}", num_str));
                    None
                }
            }?;
            return Some(Expression::Int(num));
        }
        None
    }

    fn parse_array_expression(&mut self) -> Option<Expression> {
        if !self.expect_current_token(Token::LeftBracket) {
            self.report_expected_token_error(Token::LeftBracket, self.current_token.clone());
            return None;
        }

        let mut elements = Vec::new();

        if self.current_token == Token::RightBracket {
            return Some(Expression::Array(elements));
        }

        let exp = self.parse_expression(Precedence::Lowest)?;
        elements.push(exp);

        while self.next_token == Token::Comma {
            self.advance_tokens();
            self.advance_tokens();

            let exp = self.parse_expression(Precedence::Lowest)?;
            elements.push(exp);
        }

        self.advance_tokens();

        if !self.expect_current_token(Token::RightBracket) {
            self.report_expected_token_error(Token::RightBracket, self.current_token.clone());
            return None;
        }

        Some(Expression::Array(elements))
    }

    fn parse_infix_expression(&mut self, left_expression: Expression) -> Option<Expression> {
        let operator = self.current_token.clone();
        let precedence = self.next_token.precedence();

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
        let value = match self.current_token {
            Token::True => true,
            Token::False => false,
            _ => {
                self.report_error("boolean expressions should have either 'true' or 'false'");
                return None;
            }
        };

        Some(Expression::Boolean(value))
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_next_token(Token::LeftParentesis) {
            self.report_expected_token_error(Token::LeftParentesis, self.next_token.clone());
            return None;
        }
        self.advance_tokens();

        let condition = Box::new(self.parse_expression(Precedence::Lowest)?);

        if !self.expect_next_token(Token::RightParentesis) {
            self.report_expected_token_error(Token::LeftParentesis, self.next_token.clone());
            return None;
        }
        self.advance_tokens();

        let consequence = Box::new(self.parse_block_statement()?);

        self.advance_tokens();

        let alternative = match self.current_token {
            Token::Else => {
                self.advance_tokens();
                self.parse_block_statement()
            }
            _ => None,
        };

        Some(Expression::IfExpression {
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_function_expression(&mut self) -> Option<Expression> {
        self.advance_tokens();

        if self.current_token != Token::LeftParentesis {
            self.report_expected_token_error(Token::LeftParentesis, self.current_token.clone());
            self.advance_tokens();
            return None;
        }

        let parameters = self.parse_function_parameters()?;
        let body = Box::new(self.parse_block_statement()?);

        Some(Expression::FunctionExpression { parameters, body })
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Token>> {
        let mut parameters = Vec::new();

        if self.expect_next_token(Token::RightParentesis) {
            self.advance_tokens();
            return Some(parameters);
        }

        self.advance_tokens();

        let identifier = self.current_token.clone();
        parameters.push(identifier);

        while self.next_token == Token::Comma {
            self.advance_tokens();
            self.advance_tokens();
            let identifier = self.current_token.clone();
            parameters.push(identifier);
        }

        if !self.expect_next_token(Token::RightParentesis) {
            self.report_expected_token_error(Token::RightParentesis, self.next_token.clone());
            return None;
        }

        self.advance_tokens();

        Some(parameters)
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

        Some(BlockStatement { statements })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        self.advance_tokens();
        let arguments = self.parse_call_expression_arguments()?;

        Some(Expression::CallExpression {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_call_expression_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut arguments = Vec::new();

        if self.current_token == Token::RightParentesis {
            self.advance_tokens();
            return Some(arguments);
        }

        let expression = self.parse_expression(Precedence::Lowest)?;
        arguments.push(expression);

        while self.next_token == Token::Comma {
            self.advance_tokens();
            self.advance_tokens();
            let expression = self.parse_expression(Precedence::Lowest)?;
            arguments.push(expression);
        }

        if !self.expect_next_token(Token::RightParentesis) {
            self.report_expected_token_error(Token::RightParentesis, self.next_token.clone());
            return None;
        }

        Some(arguments)
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

    fn report_error(&mut self, message: &str) {
        self.errors.push(message.to_string())
    }
}

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

    fn prefix_parse_fn(&self) -> Option<fn(&mut Parser) -> Option<Expression>> {
        match self {
            Token::String(_) => Some(Parser::parse_string),
            Token::Identifier(_) => Some(Parser::parse_identifier),
            Token::Int(_) => Some(Parser::parse_int),
            Token::Bang => Some(Parser::parse_prefix_expression),
            Token::LeftParentesis => Some(Parser::parse_grouped_expression),
            Token::LeftBracket => Some(Parser::parse_array_expression),
            Token::Minus => Some(Parser::parse_prefix_expression),
            Token::True | Token::False => Some(Parser::parse_boolean_expression),
            Token::If => Some(Parser::parse_if_expression),
            Token::Function => Some(Parser::parse_function_expression),
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
            Token::LeftParentesis => Some(Parser::parse_call_expression),
            _ => None,
        }
    }
}
