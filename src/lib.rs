pub mod lexer;
pub mod token;

use std::io::{BufRead, Write};

use crate::{lexer::Lexer, token::Token};

trait AstNode {
    fn get_token_literal(&self) -> String;
}

trait Expression: AstNode {}

trait Statement: AstNode {}

struct IntExpression {
    token: Token, // Token::Int(val)
}

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

impl Expression for IntExpression {}

#[derive(Debug)]
struct Identifier {
    token: Token, // Token::Idetifier(name)
}

struct LetStatement {
    token: Token, // Token::Let
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

        while self.current_token != Token::Eof {
            match self.current_token {
                Token::Let => {
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

        if !matches!(self.current_token, Token::Identifier(_)) {
            panic!("ERROR(parse_let_statement): token after let must be an identifer!");
        }

        let name = self.parse_identifier();

        self.advance_tokens();

        if self.current_token != Token::Assign {
            panic!("ERROR(parse_let_statement): token after let identifier must be an equal sign!");
        }

        self.advance_tokens();

        let value = self.parse_expression();

        self.advance_tokens();

        if self.current_token != Token::Semicolon {
            panic!("ERROR(parse_let_statement): semicolon expected!");
        }

        LetStatement {
            token: Token::Let,
            value: Box::new(value),
            name,
        }
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
            _ => panic!("ERROR(parse_identifier): expected identifier token, got {:?}", self.current_token)
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
