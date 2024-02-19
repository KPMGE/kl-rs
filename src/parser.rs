use crate::{
    ast::{AstNode, Expression},
    lexer::Lexer,
    token::Token,
};

#[derive(Debug)]
struct IntExpression {
    token: Token, // Token::Int(val)
}

#[derive(Debug)]
pub struct Identifier {
    token: Token, // Token::Idetifier(name)
}

pub enum Statement {
    LetStatement {
        token: Token, // Token::Let
        name: Identifier,
        value: Box<dyn Expression>,
    },
    ReturnStatement {
        token: Token, // Token::Return
        value: Box<dyn Expression>,
    },
    ExpressionStatement {
        token: Token, // first expression token
        value: Box<dyn Expression>,
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

        let name = self
            .parse_identifier()?
            .downcast::<Identifier>()
            .map_or(None, |identifier| Some(identifier))?;

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
            value,
            name: *name,
        })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        Some(Statement::ExpressionStatement {
            token: self.current_token.clone(),
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

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Box<dyn Expression>> {
        if let Some(parse_fn) = self.get_parse_function(&self.current_token) {
            return parse_fn(self);
        }

        match &self.current_token {
            Token::Int(num) => Some(Box::new(IntExpression {
                token: Token::Int(num.to_string()),
            })),
            _ => None,
        }
    }

    fn parse_identifier(&self) -> Option<Box<dyn Expression>> {
        if let Token::Identifier(_) = &self.current_token {
            let identifier = Identifier {
                token: self.current_token.clone(),
            };
            return Some(Box::new(identifier));
        }
        None
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

    fn get_parse_function(
        &self,
        token: &Token,
    ) -> Option<fn(&Parser) -> Option<Box<dyn Expression>>> {
        match *token {
            Token::Identifier(_) => Some(Parser::parse_identifier),
            _ => None,
        }
    }
}

impl Expression for IntExpression {}

impl Expression for Identifier {}

impl AstNode for IntExpression {
    fn get_token_literal(&self) -> String {
        match &self.token {
            Token::Int(num) => num.to_string(),
            _ => panic!(
                "ERROR(get_token_literal): expected token type Int, found {:?}",
                self.token
            ),
        }
    }
}

impl AstNode for Identifier {
    fn get_token_literal(&self) -> String {
        match &self.token {
            Token::Identifier(name) => name.to_string(),
            _ => panic!(
                "ERROR(get_token_literal): expected token type Identifier, found {:?}",
                self.token
            ),
        }
    }
}

impl AstNode for Statement {
    fn get_token_literal(&self) -> String {
        todo!()
    }
}
