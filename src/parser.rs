use crate::{
    ast::{AstNode, Expression, Statement},
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

pub struct LetStatement {
    pub token: Token, // Token::Let
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct Program<T>
where
    T: Statement,
{
    pub statements: Vec<T>,
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

    pub fn parse_program(&mut self) -> Program<LetStatement> {
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
                _ => {}
            }

            self.advance_tokens();
        }

        program
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        self.advance_tokens();

        if !matches!(self.current_token, Token::Identifier(_)) {
            self.report_expected_token_error(Token::Identifier("some identifier".to_string()));
            return None;
        }

        let name = self.parse_identifier();

        self.advance_tokens();

        if self.current_token != Token::Assign {
            self.report_expected_token_error(Token::Assign);
            return None;
        }

        self.advance_tokens();

        let value = self.parse_expression();

        self.advance_tokens();

        if self.current_token != Token::Semicolon {
            self.report_expected_token_error(Token::Semicolon);
            return None;
        }

        Some(LetStatement {
            token: Token::Let,
            value: Box::new(value),
            name,
        })
    }

    fn parse_expression(&self) -> impl Expression {
        match &self.current_token {
            Token::Int(num) => IntExpression {
                token: Token::Int(num.to_string()),
            },
            _ => todo!(),
        }
    }

    fn parse_identifier(&self) -> Identifier {
        match &self.current_token {
            Token::Identifier(_) => Identifier {
                token: self.current_token.clone(),
            },
            _ => panic!(
                "ERROR(parse_identifier): expected identifier token, got {:?}",
                self.current_token
            ),
        }
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
}

impl Expression for IntExpression {}

impl Expression for Identifier {}

impl Statement for LetStatement {}

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

impl AstNode for LetStatement {
    fn get_token_literal(&self) -> String {
        self.value.get_token_literal()
    }
}
