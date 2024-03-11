use std::io::{self, BufRead, Write};

use kl_rs::{evaluator::Evaluator, lexer::Lexer, parser::Parser, token::Token};

fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut input = String::new();

        print!(">> ");
        std::io::stdout()
            .flush()
            .expect("error while flusing stdout");

        handle
            .read_line(&mut input)
            .expect("error while reading from stdin");

        match input.as_str() {
            "exit\n" => break,
            "clear\n" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().expect("could not flush stdout");
                continue;
            }
            _ => {}
        }

        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer.clone());

        println!("Tokens: \n");

        let mut token = lexer.next_token();

        while token != Token::Eof {
            println!("{:?}", token);
            token = lexer.next_token();
        }
        println!();

        let program = parser.parse_program();

        println!("Parsed program: ");
        if let kl_rs::ast::AstNode::Program { ref statements } = program {
            for statement in statements {
                println!("statement: {:#?}", statement);
            }
        }
        println!();

        println!("ERRORS: ");
        for err in parser.errors {
            println!("ERROR: {:?}", err);
        }
        println!();

        println!("EVALUATED: ");
        let evaluator = Evaluator::new();
        let object = evaluator.eval(program);
        println!("{}", object.inspect());
    }
}
