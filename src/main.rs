mod lexer;
mod token;

use std::io::{BufRead, Write};

use crate::{lexer::Lexer, token::Token};

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

        let mut lexer = Lexer::new(input.to_string());

        if input == "exit\n" {
            break
        }

        println!("Tokens: \n");

        loop {
            let token = lexer.next_token();
            if token == Token::Eof {
                break;
            }
            println!("{:?}", token);
        }

        println!();
    }
}
