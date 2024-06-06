use std::io::{self, BufRead, Write};

use kl_rs::{evaluator::Evaluator, lexer::Lexer, parser::Parser, token::Token};

gflags::define! {
    -h, --help = false
}
gflags::define! {
    -v, --verbose = false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    gflags::parse();

    if HELP.flag {
        gflags::print_help_and_exit(0);
    }

    let evaluator = Evaluator::new();

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

        let lexer = Lexer::new(input.as_str());
        let mut parser = Parser::new(lexer);

        if VERBOSE.flag {
            debug_lexer(Lexer::new(input.as_str()));
            debug_parser(Parser::new(Lexer::new(input.as_str())));
        }

        let program = parser.parse_program()?;

        let object = evaluator.eval(program);
        println!("{}", object.inspect());
    }

    Ok(())
}

fn debug_lexer(mut lexer: Lexer) {
    println!("Tokens: \n");

    let mut token = lexer.next().unwrap();

    while token != Token::Eof {
        println!("{:?}", token);
        token = lexer.next().unwrap();
    }

    println!();
}

fn debug_parser(mut parser: Parser) {
    match parser.parse_program() {
        Ok(node) => {
            println!("Parsed program: ");
            if let kl_rs::ast::AstNode::Program { ref statements } = node {
                for statement in statements {
                    println!("statement: {:#?}", statement);
                }
            }
            println!();
        }
        Err(e) => {
            panic!("{:#?}", e);
        }
    }
}
