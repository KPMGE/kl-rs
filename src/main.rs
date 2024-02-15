mod lexer;
mod token;

use std::io::{BufRead, Write};

use crate::{lexer::Lexer, token::Token, token::TokenType};

trait AstNode {
    fn get_token_literal(&self) -> String;
}

trait Expression: AstNode {}

trait Statement: AstNode {}

struct IntExpression {
    token: TokenType,
    value: String,
}

impl AstNode for IntExpression {
    fn get_token_literal(&self) -> String {
        self.value.clone()
    }
}

impl Expression for IntExpression {}

#[derive(Debug)]
struct Identifier {
    token: TokenType,
    value: String,
}

struct LetStatement {
    token: TokenType,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl AstNode for LetStatement {
    fn get_token_literal(&self) -> String {
        self.value.get_token_literal()
    }
}

impl Statement for LetStatement {}

#[derive(Debug)]
struct Program<T: Statement> {
    statements: Vec<T>,
}

#[derive(Debug)]
struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            next_token,
        }
    }

    fn parse_program(&mut self) -> Program<LetStatement> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token.token_type != TokenType::Eof {
            match self.current_token.token_type {
                TokenType::Let => {
                    let statement = self.parse_let_statement();
                    program.statements.push(statement);
                }
                _ => {}
            }

            self.advance_tokens();
        }

        program
    }

    fn parse_let_statement(&mut self) -> LetStatement {
        self.advance_tokens();

        if self.current_token.token_type != TokenType::Identifier {
            panic!("ERROR(parse_let_statement): token after let must be an identifer!");
        }


        let name = self.parse_identifier();

        self.advance_tokens();

        // println!("current: {:?}", self.current_token.token_type);
        // println!("next: {:?}", self.next_token);

        // if self.current_token.token_type != TokenType::Equals {
        //     panic!("ERROR(parse_let_statement): token after let identifier must be an equal sign!");
        // }

        self.advance_tokens();

        let value = self.parse_expression();

        self.advance_tokens();

        if self.current_token.token_type != TokenType::Semicolon {
            panic!("ERROR(parse_let_statement): semicolon expected");
        }

        LetStatement {
            token: TokenType::Let,
            value: Box::new(value),
            name,
        }
    }

    fn parse_expression(&self) -> impl Expression {
        match self.current_token.token_type {
            TokenType::Int => IntExpression {
                token: self.current_token.token_type.clone(),
                value: self
                    .current_token
                    .literal
                    .clone()
                    .expect("ERROR(parse_expression): None value for literal"),
            },
            _ => todo!(),
        }
    }

    fn parse_identifier(&self) -> Identifier {
        Identifier {
            token: self.current_token.token_type.clone(),
            value: self.current_token.literal.clone().unwrap(),
        }
    }

    fn advance_tokens(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }
}

fn main() {
    let program = "let x = 5;";
    let lexer = Lexer::new(program.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    for statament in program.statements {
        println!("statement value: {}", statament.get_token_literal());
    }

    // let stdin = std::io::stdin();
    // let mut handle = stdin.lock();
    //
    // loop {
    //     let mut input = String::new();
    //
    //     print!(">> ");
    //     std::io::stdout()
    //         .flush()
    //         .expect("error while flusing stdout");
    //
    //     handle
    //         .read_line(&mut input)
    //         .expect("error while reading from stdin");
    //
    //     let mut lexer = Lexer::new(input.to_string());
    //
    //     if input == "exit\n" {
    //         break
    //     }
    //
    //     println!("Tokens: \n");
    //
    //     loop {
    //         let token = lexer.next_token();
    //         if token.token_type == TokenType::Eof {
    //             break;
    //         }
    //         println!("{:?}", token);
    //     }
    //
    //     println!();
    // }
}
