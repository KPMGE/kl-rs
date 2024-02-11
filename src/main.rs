mod lexer;
mod token;

use token::Token;

use crate::lexer::Lexer;

fn main() {
    let code = "let     fn";
    let mut lexer = Lexer::new(code.to_string());

    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            break;
        }

        println!("token: {:?}", token);
    }
}
